use std::borrow::Cow;

use derive_builder::Builder;
use gen_api_wrapper::{endpoint_prelude::Endpoint, params::QueryParams};
use http::Method;

#[derive(Debug, Clone, Builder)]
pub struct PackagesEndpoint<'a> {
    #[builder(setter(into))]
    owner: Cow<'a, str>,

    #[builder(setter(into))]
    package: Cow<'a, str>,
}

impl<'a> Endpoint for PackagesEndpoint<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        format!("api/v1/packages/{}", self.owner).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        // TODO: maybe make configurable - MCL - 2023-07-29
        params.push("type", "generic");
        params.push("q", &self.package);
        params
    }
}

impl<'a> PackagesEndpoint<'a> {
    pub fn buidler() -> PackagesEndpointBuilder<'a> {
        PackagesEndpointBuilder::default()
    }
}

#[derive(Debug, Clone, Builder)]
pub struct PackageFilesEndpoint<'a> {
    #[builder(setter(into))]
    owner: Cow<'a, str>,

    #[builder(setter(into))]
    package: Cow<'a, str>,

    #[builder(setter(into))]
    version: Cow<'a, str>,
}

impl<'a> Endpoint for PackageFilesEndpoint<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        // TODO: make 'generic' configurable - MCL - 2023-07-29
        format!(
            "api/v1/packages/{}/generic/{}/{}/files",
            self.owner, self.package, self.version,
        )
        .into()
    }
}

impl<'a> PackageFilesEndpoint<'a> {
    pub fn buidler() -> PackageFilesEndpointBuilder<'a> {
        PackageFilesEndpointBuilder::default()
    }
}

#[derive(Debug, Clone, Builder)]
pub struct PackageFileEndpoint<'a> {
    #[builder(setter(into))]
    owner: Cow<'a, str>,

    #[builder(setter(into))]
    package: Cow<'a, str>,

    #[builder(setter(into))]
    version: Cow<'a, str>,

    #[builder(setter(into))]
    pub file: Cow<'a, str>,
}

impl<'a> Endpoint for PackageFileEndpoint<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        // TODO: make 'generic' configurable - MCL - 2023-07-29
        format!(
            "api/packages/{}/generic/{}/{}/{}",
            self.owner, self.package, self.version, self.file,
        )
        .into()
    }
}

impl<'a> PackageFileEndpoint<'a> {
    pub fn buidler() -> PackageFileEndpointBuilder<'a> {
        PackageFileEndpointBuilder::default()
    }
}

#[derive(Debug, Clone, Builder)]
pub struct PackageUploadEndpoint<'a> {
    #[builder(setter(into))]
    owner: Cow<'a, str>,

    #[builder(setter(into))]
    package: Cow<'a, str>,

    #[builder(setter(into))]
    version: Cow<'a, str>,

    #[builder(setter(into))]
    pub file: Cow<'a, str>,
}

impl<'a> Endpoint for PackageUploadEndpoint<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        // TODO: make 'generic' configurable - MCL - 2023-07-29
        format!(
            "api/packages/{}/generic/{}/{}/{}",
            self.owner, self.package, self.version, self.file,
        )
        .into()
    }
}

impl<'a> PackageUploadEndpoint<'a> {
    pub fn buidler() -> PackageUploadEndpointBuilder<'a> {
        PackageUploadEndpointBuilder::default()
    }
}
