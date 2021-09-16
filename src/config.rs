use std::path::PathBuf;

use serde::Deserialize;

pub type Config = Vec<RepositoryConfig>;

pub async fn read_config() -> anyhow::Result<Config> {
    let config_path = std::env::var("CONFIG_PATH")
        .ok()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("config.yml"));
    let contents = tokio::fs::read_to_string(config_path).await?;
    serde_yaml::from_str(&contents).map_err(Into::into)
}

#[derive(Debug, Clone, Deserialize)]
pub struct RepositoryConfig {
    /// The user or organization that owns the repo on github.
    pub owner: String,
    /// The name of the repo.
    pub name: String,
    /// The users interests for this repository.
    pub interests: Vec<Interest>,
}

/// The events the user would like to monitor for.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Interest {
    Commit(CommitInterest),
    Release(ReleaseInterest),
}

impl Interest {
    /// Gets all the container configs for an interest.
    pub fn containers(&self) -> &Vec<ContainerConfig> {
        match self {
            Interest::Commit(interest) => &interest.containers,
            Interest::Release(interest) => &interest.containers,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommitInterest {
    // A branch that the commit must occur on for the event to start a container.
    pub branch: Option<String>,
    /// All the containers that are spawned when the interest is met.
    pub containers: Vec<ContainerConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseInterest {
    /// All the containers that are spawned when the interest is met.
    pub containers: Vec<ContainerConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContainerConfig {
    /// The image used for the container.
    pub image: String,
    /// The path that the event that triggered the interest will be stored at, so the container can
    /// reference the data within that event without fetching it from github.
    pub event_path: Option<PathBuf>,
    /// The number of seconds the container is allowed to run before it is killed.
    pub timeout: Option<u32>,
}
