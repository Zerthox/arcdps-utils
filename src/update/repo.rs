use reqwest::blocking::ClientBuilder;
use semver::Version;
use serde::Deserialize;

/// Information about a GitHub repository.
#[derive(Debug, Clone)]
pub struct Repository {
    pub owner: String,
    pub repo: String,
}

impl Repository {
    /// User agent used for requests.
    pub const AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

    /// Creates new repository information.
    #[inline]
    pub fn new(owner: impl Into<String>, repo: impl Into<String>) -> Self {
        Self {
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    /// Retrieves the latest version from the latest release tag.
    pub fn latest_version_blocking(&self) -> Option<Version> {
        let Self { owner, repo } = self;
        let url = format!("https://api.github.com/repos/{owner}/{repo}/releases/latest");
        let client = ClientBuilder::new()
            .user_agent(Self::AGENT)
            .build()
            .unwrap();
        let res = client.get(url).send();
        res.ok()
            .and_then(|res| res.json::<Release>().ok())
            .and_then(|release| release.tag_name.parse().ok())
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Release {
    pub tag_name: String,
}
