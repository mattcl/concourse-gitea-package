use std::{collections::HashSet, path::PathBuf};

use anyhow::{anyhow, bail, Result};
use clap::Args;
use clap_stdin::FileOrStdin;
use gen_api_wrapper::query::AsyncQuery;

use crate::{
    client::GiteaClient,
    endpoints::{PackageFilesEndpoint, PackageUploadEndpoint},
    models::PackageFile,
    params::{OutOutput, OutParams, Version},
};

#[derive(Debug, Clone, Args)]
pub struct Out {
    sources: PathBuf,
    params: FileOrStdin<OutParams>,
}

impl Out {
    pub async fn run(&self) -> Result<()> {
        let params = self.params.clone().into_inner();
        let client = GiteaClient::try_from(&params.source)?;

        if self.params.params.files.is_empty() {
            bail!("Must specify at least one file to upload");
        }

        // see if we have files that already exist for the specified version
        let endpoint = PackageFilesEndpoint::buidler()
            .owner(&params.source.owner)
            .package(&params.source.package)
            .version(&params.params.version)
            .build()?;

        // TODO: This could fail for just connectivity reasons, but gitea will
        // prevent the overwrite anyway - MCL - 2023-07-30
        let existing_files: Vec<PackageFile> =
            endpoint.query_async(&client).await.ok().unwrap_or_default();

        let existing_names: HashSet<&String> =
            HashSet::from_iter(existing_files.iter().map(|f| &f.name));

        for file in self.params.params.files.iter() {
            let filename = file
                .file_name()
                .ok_or_else(|| {
                    anyhow!(
                        "Could not determine basename for {}",
                        file.to_string_lossy()
                    )
                })?
                .to_string_lossy();

            if existing_names.contains(&filename.to_string()) {
                eprintln!(
                    "Skipping '{}' because it already exists for version {}",
                    filename, &self.params.params.version
                );
                continue;
            }

            eprintln!("Uploading {}", filename);

            let endpoint = PackageUploadEndpoint::buidler()
                .owner(&params.source.owner)
                .package(&params.source.package)
                .version(&params.params.version)
                .file(filename)
                .build()?;

            let target = self.sources.join(file);
            if !target.is_file() {
                bail!(
                    "File '{}' is not a file or does not exist",
                    file.to_string_lossy()
                );
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
