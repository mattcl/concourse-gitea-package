use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Args;
use clap_stdin::FileOrStdin;
use gen_api_wrapper::query::AsyncQuery;

use crate::{
    client::GiteaClient,
    endpoints::{PackageFileEndpoint, PackageFilesEndpoint},
    models::PackageFile,
    params::{GetOutput, GetParams},
};

#[derive(Debug, Clone, Args)]
pub struct Get {
    destination: PathBuf,
    params: FileOrStdin<GetParams>,
}

impl Get {
    pub async fn run(&self) -> Result<()> {
        let params = self.params.clone().into_inner();
        let client = GiteaClient::try_from(&params.source)?;

        let endpoint = PackageFilesEndpoint::buidler()
            .owner(&params.source.owner)
            .package(&params.source.package)
            .version(&params.version.version)
            .build()?;

        let files: Vec<PackageFile> = endpoint.query_async(&client).await.with_context(|| {
            format!(
                "Could not find files for '{}' at '{}'",
                &params.source.package, &self.params.version.version
            )
        })?;

        // download each file to the specified location
        for file in files {
            eprintln!("Fetching {}", &file.name);
            let endpoint = PackageFileEndpoint::buidler()
                .owner(&params.source.owner)
                .package(&params.source.package)
                .version(&params.version.version)
                .file(&file.name)
                .build()?;

            client
                .download(&self.destination, &endpoint)
                .await
                .with_context(|| format!("Failed downloading '{}'", &file.name))?;
        }

        eprintln!("Finished fetching files");

        println!(
            "{}",
            serde_json::to_string(&GetOutput::from(&self.params.version))?
        );

        Ok(())
    }
}
