use semver::Version;
use serde::Deserialize;

/// Information about a GitHub repository.
#[derive(Debug, Clone)]
pub struct Repository {
    repo: String,
}

impl Repository {
    /// User agent used for requests.
    pub const AGENT: &'static str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

    /// Creates new repository information.
    #[inline]
    pub fn new(owner: impl AsRef<str>, repo: impl AsRef<str>) -> Self {
        Self {
            repo: format!("{}/{}", owner.as_ref(), repo.as_ref()),
        }
    }

    /// Retrieves the latest version from the latest release tag.
    pub fn latest_version_blocking(&self) -> Option<Version> {
        minreq::get(format!(
            "https://api.github.com/repos/{}/releases/latest",
            self.repo
        ))
        .with_header("User-Agent", Self::AGENT)
        .send()
        .ok()
        .and_then(|res| res.json::<Release>().ok())
        .and_then(|release| release.tag_name.parse().ok())
    }

    /// Opens the latest release.
    pub fn open_release(&self) {
        let _ = open::that(format!("https://github.com/{}/releases/latest", self.repo));
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Release {
    pub tag_name: String,
}
