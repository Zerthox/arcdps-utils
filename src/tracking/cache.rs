use super::{Entry, Player, Tracker};
use arcdps::Profession;
use std::ops;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Player tracker with caching.
#[derive(Debug, Clone)]
pub struct CachedTracker<T> {
    /// Inner tracker.
    tracker: Tracker<T>,

    /// Cache for data.
    cache: Vec<CacheEntry<T>>,

    /// How to cache for self.
    pub self_policy: CachePolicy,

    /// How to cache for other players.
    pub others_policy: CachePolicy,
}

impl<T> CachedTracker<T> {
    /// Creates a new cached tracker with the given policies.
    pub const fn new(self_policy: CachePolicy, others_policy: CachePolicy) -> Self {
        Self {
            tracker: Tracker::new(),
            cache: Vec::new(),
            self_policy,
            others_policy,
        }
    }

    /// Creates a new tracker caching data for own characters.
    pub const fn for_self() -> Self {
        Self::new(CachePolicy::PerCharacter, CachePolicy::None)
    }

    /// Creates a new tracker caching data for own characters and other players' account.
    pub const fn for_all() -> Self {
        Self::new(CachePolicy::PerCharacter, CachePolicy::PerAccount)
    }

    /// Returns the applicable cache policy.
    fn cache_policy(&self, is_self: bool) -> CachePolicy {
        if is_self {
            self.self_policy
        } else {
            self.others_policy
        }
    }

    /// Searches the cache for an entry, removing it when found.
    fn search_cache(
        &mut self,
        predicate: impl FnMut(&CacheEntry<T>) -> bool,
    ) -> Option<CacheEntry<T>> {
        let index = self.cache.iter().position(predicate)?;
        Some(self.cache.remove(index))
    }

    /// Adds a new tracked player, returning `true` if cached data was used.
    pub fn add_player(&mut self, player: Player, data: T) -> bool {
        let cached = match self.cache_policy(player.is_self) {
            CachePolicy::None => None,
            CachePolicy::PerAccount => self
                .search_cache(|entry| entry.account == player.account)
                .and_then(|entry| {
                    if entry.character == player.character {
                        Some(entry.data)
                    } else {
                        None
                    }
                }),
            CachePolicy::PerCharacter => self
                .search_cache(|entry| entry.character == player.character)
                .map(|entry| entry.data),
        };
        let found = cached.is_some();

        self.tracker.add_player(player, cached.unwrap_or(data));

        found
    }

    /// Adds a new tracked player with default data, returning `true` if cached data was used.
    pub fn add_player_default(&mut self, player: Player) -> bool
    where
        T: Default,
    {
        self.add_player(player, T::default())
    }

    /// Removes a tracked player, returning `true` if the player was tracked.
    pub fn remove_player(&mut self, id: usize) -> bool {
        self.tracker
            .remove_player(id)
            .map(|entry| self.cache_entry(entry))
            .is_some()
    }

    /// Removes a tracked player, returning the removed entry if the player was tracked.
    pub fn take_player(&mut self, id: usize) -> Option<Entry<T>>
    where
        T: Clone,
    {
        self.tracker.remove_player(id).map(|entry| {
            self.cache_entry(entry.clone());
            entry
        })
    }

    /// Returns a reference to the local player (self).
    pub fn get_self(&self) -> Option<&Entry<T>> {
        self.tracker.get_self()
    }

    /// Returns a mutable reference to the local player (self).
    pub fn get_self_mut(&mut self) -> Option<&mut Entry<T>> {
        self.tracker.get_self_mut()
    }

    /// Returns a reference to a tracked player entry.
    pub fn player(&self, id: usize) -> Option<&Entry<T>> {
        self.tracker.player(id)
    }

    /// Returns a mutable reference to a tracked player entry.
    pub fn player_mut(&mut self, id: usize) -> Option<&mut Entry<T>> {
        self.tracker.player_mut(id)
    }

    /// Adds an entry to the cache.
    ///
    /// Caching happens automatically based on the set [`CachePolicy`], so usually this does not have to be called manually.
    pub fn cache_entry(&mut self, entry: Entry<T>) {
        if self.cache_policy(entry.player.is_self).can_cache() {
            self.cache.push(entry.into());
        }
    }

    /// Returns an iterator over the current cache contents.
    pub fn cached(&self) -> impl Iterator<Item = &CacheEntry<T>> {
        self.cache.iter()
    }

    /// Clears the cache.
    pub fn clear_cache(&mut self) {
        self.cache.clear()
    }
}

impl<T> Default for CachedTracker<T> {
    fn default() -> Self {
        Self::for_self()
    }
}

impl<T> ops::Deref for CachedTracker<T> {
    type Target = [Entry<T>];

    fn deref(&self) -> &Self::Target {
        self.tracker.entries.as_slice()
    }
}

impl<T> ops::DerefMut for CachedTracker<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.tracker.entries.as_mut_slice()
    }
}

/// Cache entry.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CacheEntry<T> {
    pub character: String,
    pub account: String,
    pub profession: Profession,
    pub data: T,
}

impl<T> From<Entry<T>> for CacheEntry<T> {
    fn from(entry: Entry<T>) -> Self {
        let Entry { player, data } = entry;
        Self {
            character: player.character,
            account: player.account,
            profession: player.profession,
            data,
        }
    }
}

/// How data should be cached.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CachePolicy {
    /// Do not cache anything.
    None,

    /// Cache one entry per account.
    PerAccount,

    /// Cache one entry per character.
    PerCharacter,
}

impl CachePolicy {
    /// Whether caching is allowed.
    pub const fn can_cache(&self) -> bool {
        matches!(self, Self::PerAccount | Self::PerCharacter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arcdps::{Profession, Specialization};

    #[test]
    fn caching() {
        let mut tracker = CachedTracker::for_self();

        let player = Player::new(
            123,
            "Char",
            "Acc",
            true,
            Profession::Unknown,
            Specialization::Unknown,
            1,
        );

        tracker.add_player_default(player.clone());
        assert_eq!(1, tracker.len());

        tracker.player_mut(123).unwrap().data = 42;
        assert_eq!(42, tracker.player(123).unwrap().data);

        tracker.remove_player(123);
        assert!(tracker.is_empty());

        tracker.add_player_default(player);
        assert_eq!(1, tracker.len());
        assert_eq!(42, tracker.player(123).unwrap().data);
    }

    #[test]
    fn cache_policy() {
        let mut tracker = CachedTracker::for_all();

        let char1 = Player::new(
            123,
            "Char1",
            "Account1",
            false,
            Profession::Unknown,
            Specialization::Unknown,
            1,
        );

        let char2 = Player::new(
            456,
            "Char2",
            "Account1",
            false,
            Profession::Unknown,
            Specialization::Unknown,
            1,
        );

        let char3 = Player::new(
            789,
            "Char3",
            "Account2",
            false,
            Profession::Unknown,
            Specialization::Unknown,
            1,
        );

        tracker.add_player(char1.clone(), 42);

        tracker.remove_player(123);
        assert_eq!(1, tracker.cache.len());

        let cached = tracker.add_player(char2.clone(), 100);
        assert!(!cached);
        assert_eq!(100, tracker.player(456).unwrap().data);
        assert!(tracker.cache.is_empty());

        tracker.remove_player(456);
        assert!(tracker.is_empty());
        assert_eq!(1, tracker.cache.len());

        let cached = tracker.add_player_default(char2);
        assert!(cached);
        assert_eq!(100, tracker.player(456).unwrap().data);
        assert!(tracker.cache.is_empty());

        tracker.remove_player(456);
        let cached = tracker.add_player_default(char3);
        assert!(!cached);

        tracker.remove_player(789);
        assert!(tracker.is_empty());
        assert_eq!(2, tracker.cache.len());
    }
}
