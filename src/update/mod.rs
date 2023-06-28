//! Plugin update checking.

mod repo;
mod ui;

pub use self::repo::*;
pub use self::ui::*;

use semver::Version;
use std::{
    sync::{Arc, OnceLock},
    thread,
};

/// Update checker.
#[derive(Debug, Clone)]
pub struct Updater {
    name: String,
    repo: Repository,
    current: Version,
    latest: Arc<OnceLock<Version>>,
}

impl Updater {
    /// Creates a new updater without checking for updates.
    #[inline]
    pub fn unchecked(name: impl Into<String>, repo: Repository, current: Version) -> Self {
        Self {
            name: name.into(),
            repo,
            current,
            latest: Default::default(),
        }
    }

    /// Creates a new updater and automatically checks for updates.
    #[inline]
    pub fn new(name: impl Into<String>, repo: Repository, current: Version) -> Self {
        let mut updater = Self::unchecked(name, repo, current);
        updater.check();
        updater
    }

    /// Whether the current version is outdated.
    #[inline]
    pub fn is_outdated(&self) -> bool {
        match self.latest.get() {
            Some(latest) => latest > &self.current,
            None => false,
        }
    }

    /// Initiates the update check.
    #[inline]
    pub fn check(&mut self) {
        if self.latest.get().is_none() {
            let latest = self.latest.clone();
            let repo = self.repo.clone();
            thread::spawn(move || {
                if let Some(version) = repo.latest_version_blocking() {
                    let _ = latest.set(version);
                }
            });
        }
    }

    /// Resets the update check.
    #[inline]
    pub fn reset(&mut self) {
        self.latest = Default::default();
    }

    fn latest_if_outdated(&self) -> Option<&Version> {
        self.latest.get().filter(|latest| *latest > &self.current)
    }
}
