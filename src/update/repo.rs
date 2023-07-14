use semver::Version;
use serde::Deserialize;

/// Information about a GitHub repository.
#[derive(Debug, Clone)]
pub struct Repository {
    url: String,
}

impl Repository {
    /// User agent used for requests.
    pub const AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

    /// Creates new repository information.
    #[inline]
    pub fn new(owner: impl Into<String>, repo: impl Into<String>) -> Self {
        Self {
            url: format!(
                "https://api.github.com/repos/{}/{}/releases/latest",
                owner.into(),
                repo.into()
            ),
        }
    }

    /// Retrieves the latest version from the latest release tag.
    pub fn latest_version_blocking(&self) -> Option<Version> {
        let res = minreq::get(&self.url)
            .with_header("User-Agent", Self::AGENT)
            .send();
        let release = res.ok().and_then(|res| res.json::<Release>().ok());
        release.and_then(|release| release.tag_name.parse().ok())
    }

    /// Opens the latest release.
    pub fn open_release(&self) {
        let _ = open::that(&self.url);
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Release {
    pub tag_name: String,
}
