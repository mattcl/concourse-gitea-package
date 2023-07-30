use std::{fs::File, io::Write, path::Path};

use anyhow::{Context, Result};
use async_trait::async_trait;
use bytes::Bytes;
use futures_util::StreamExt;
use gen_api_wrapper::{
    client::{AsyncClient, RestClient},
    endpoint_prelude::Endpoint,
    error::ApiError,
    query,
};
use http::{
    header::{self, AUTHORIZATION},
    HeaderMap, HeaderValue, Request, Response,
};
use reqwest::{Body, Client};
use thiserror::Error;
use tokio_util::codec::{BytesCodec, FramedRead};
use url::Url;

use crate::{
    endpoints::{PackageFileEndpoint, PackageUploadEndpoint},
    params::Source,
};

#[derive(Debug, Error)]
pub enum GiteaError {
    #[error("failed to parse url: {}", source)]
    UrlParse {
        #[from]
        source: url::ParseError,
    },
    #[error("error setting auth header: {}", source)]
    AuthError {
        #[from]
        source: AuthError,
    },
    #[error("failed to talk to the gitea api: {}", source)]
    Communication {
        #[from]
        source: reqwest::Error,
    },
    #[error("api error: {}", source)]
    Api {
        #[from]
        source: ApiError<RestError>,
    },
}

#[derive(Debug, Error)]
pub enum RestError {
    #[error("error setting auth headers: {}", source)]
    AuthError {
        #[from]
        source: AuthError,
    },
    #[error("failed to talk to the gitea api: {}", source)]
    Communication {
        #[from]
        source: reqwest::Error,
    },
    #[error("http error: {}", source)]
    Http {
        #[from]
        source: http::Error,
    },
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("header value error: {}", source)]
    HeaderValue {
        #[from]
        source: http::header::InvalidHeaderValue,
    },
}

#[derive(Clone)]
struct Auth {
    token: String,
}

impl Auth {
    pub fn set_header<'a>(
        &self,
        headers: &'a mut HeaderMap<HeaderValue>,
    ) -> Result<&'a mut HeaderMap<HeaderValue>, AuthError> {
        let mut header_value = HeaderValue::from_str(&format!("token {}", &self.token))?;
        header_value.set_sensitive(true);
        headers.insert(AUTHORIZATION, header_value);
        Ok(headers)
    }
}

/// A client for interacting with the Gitea API.
///
/// Can either be used directly or as an argument to the endpoint structs.
#[derive(Clone)]
pub struct GiteaClient {
    client: Client,
    uri: Url,
    auth: Auth,
}

impl GiteaClient {
    /// Make a new [GiteaClient].
    ///
    /// This will fail if the provided `api_url` does not parse.
    pub fn new(uri: Url, token: &str) -> Result<Self, GiteaError> {
        let client = Client::builder().build()?;

        Ok(Self {
            client,
            uri,
            auth: Auth {
                token: token.into(),
            },
        })
    }

    pub async fn download<'a>(
        &self,
        destination: &Path,
        endpoint: &PackageFileEndpoint<'a>,
    ) -> Result<()> {
        // we're just going to do this directly so we can get at the body as bytes
        // TODO: it would be nice if the gen wrapper handled this
        // - MCL - 2023-07-29
        let url = self.rest_endpoint(&endpoint.endpoint())?;

        let mut req = Request::builder()
            .method(endpoint.method())
            .uri(query::url_to_http_uri(url));
        self.auth.set_header(req.headers_mut().unwrap())?;
        let (req, data) = if let Some((mime, data)) = endpoint.body()? {
            let req = req.header(header::CONTENT_TYPE, mime);
            (req, data)
        } else {
            (req, Vec::new())
        };
        let http_request = req.body(data)?;
        let request = http_request.try_into()?;
        let rsp = self.client.execute(request).await?;

        // we're going to do this in chunks
        let target = destination.join(&endpoint.file.to_string());
        let mut file = File::create(&target)
            .with_context(|| format!("Failed to create file '{}'", &target.to_string_lossy()))?;

        let mut stream = rsp.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item?;
            file.write_all(&chunk)?;
        }

        Ok(())
    }

    pub async fn upload<'a>(
        &self,
        target: &Path,
        endpoint: &PackageUploadEndpoint<'a>,
    ) -> Result<()> {
        // we're just going to do this directly.
        // TODO: it would be nice if the gen wrapper handled this
        // - MCL - 2023-07-29
        let url = self.rest_endpoint(&endpoint.endpoint())?;

        let mut req = Request::builder()
            .method(endpoint.method())
            .uri(query::url_to_http_uri(url));
        self.auth.set_header(req.headers_mut().unwrap())?;

        let file = tokio::fs::File::open(target)
            .await
            .with_context(|| format!("Could not open file: '{}'", target.to_string_lossy()))?;

        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);

        let http_request = req.body(body)?;
        let request = http_request.try_into()?;
        self.client
            .execute(request)
            .await
            .with_context(|| format!("Failed to upload file '{}'", target.to_string_lossy()))?;

        Ok(())
    }
}

impl TryFrom<&Source> for GiteaClient {
    type Error = GiteaError;

    fn try_from(value: &Source) -> std::result::Result<Self, Self::Error> {
        Self::new(value.uri.clone(), &value.token)
    }
}

impl RestClient for GiteaClient {
    type Error = RestError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>> {
        Ok(self.uri.join(endpoint)?)
    }
}

#[async_trait]
impl AsyncClient for GiteaClient {
    async fn rest_async(
        &self,
        mut request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<<Self as RestClient>::Error>> {
        use futures_util::TryFutureExt;
        let call = || async {
            self.auth.set_header(request.headers_mut().unwrap())?;
            let http_request = request.body(body)?;
            let request = http_request.try_into()?;
            let rsp = self.client.execute(request).await?;

            let mut http_rsp = Response::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            for (key, value) in rsp.headers() {
                headers.insert(key, value.clone());
            }
            Ok(http_rsp.body(rsp.bytes().await?)?)
        };
        call().map_err(ApiError::client).await
    }
}
