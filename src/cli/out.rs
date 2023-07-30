use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::Args;
use clap_stdin::FileOrStdin;

use crate::{
    client::GiteaClient,
    endpoints::PackageUploadEndpoint,
    params::{OutOutput, OutParams, Version},
};

#[derive(Debug, Clone, Args)]
pub struct Out {
    sources: PathBuf,
    params: FileOrStdin<OutParams>,
}

impl Out {
    pub async fn run(&self) -> Result<()> {
        let client = GiteaClient::try_from(&self.params.source)?;

        if self.params.params.files.is_empty() {
            bail!("Must specify at least one file to upload");
        }

        for file in self.params.params.files.iter() {
            eprintln!("Uploading {}", file);

            let endpoint = PackageUploadEndpoint::buidler()
                .owner(&self.params.source.owner)
                .package(&self.params.source.package)
                .version(&self.params.params.version)
                .file(file)
                .build()?;

            let target = self.sources.join(file);
            if !target.is_file() {
                bail!("File '{}' is not a file or does not exist", file);
            }

            client.upload(&target, &endpoint).await?;
        }

        eprintln!("Finished uploading files");

        let version = Version {
            version: self.params.params.version.clone(),
        };
        println!("{}", serde_json::to_string(&OutOutput::from(&version))?);

        Ok(())
    }
}
