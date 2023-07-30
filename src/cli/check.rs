use anyhow::Result;
use clap::Args;
use clap_stdin::FileOrStdin;
use gen_api_wrapper::query::AsyncQuery;

use crate::{
    client::GiteaClient,
    endpoints::PackagesEndpoint,
    models::Package,
    params::{CheckParams, Version},
};

#[derive(Debug, Clone, Args)]
pub struct Check {
    params: FileOrStdin<CheckParams>,
}

impl Check {
    pub async fn run(&self) -> Result<()> {
        let client = GiteaClient::try_from(&self.params.source)?;

        let endpoint = PackagesEndpoint::buidler()
            .owner(&self.params.source.owner)
            .package(&self.params.source.package)
            .build()?;

        let mut packages: Vec<Package> = endpoint.query_async(&client).await?;

        // FIXME: We need to paginate this and even then the performance might be
        // pretty bad if there are tons of versions. For now, let's just get
        // this working- MCL - 2023-07-29

        // We have to filter because the query param matches substrings.
        // TODO: see if there's an actual syntax to have the query be an exact
        // match so we don't have to filter this out. - MCL - 2023-07-29
        packages.retain(|p| p.name == self.params.source.package);

        if let Some(ref previous) = self.params.version {
            let pos = packages.iter().position(|p| p.version == previous.version);
            if let Some(pos) = pos {
                let cutoff_id = packages[pos].id;

                packages.retain(|p| p.id > cutoff_id);

                if packages.is_empty() {
                    // we specified version _is_ the latest, so we just return
                    // that
                    let versions = vec![previous];
                    println!("{}", serde_json::to_string(&versions)?);

                    return Ok(());
                }
            }
        }

        // we're making the assumption that the ids are never decreasing so we
        // don't need to actually compare the timestamps
        packages.sort_by(|a, b| a.id.cmp(&b.id));

        let versions: Vec<Version> = packages
            .iter()
            .map(|p| Version {
                version: p.version.clone(),
            })
            .collect();

        println!("{}", serde_json::to_string(&versions)?);

        Ok(())
    }
}
