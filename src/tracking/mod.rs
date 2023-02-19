//! Player tracking utilities.

mod cache;
mod entry;
mod player;

pub use cache::*;
pub use entry::*;
pub use player::*;

use std::ops;

/// Player tracker.
#[derive(Debug, Clone)]
pub struct Tracker<T> {
    /// Currently tracked players.
    entries: Vec<Entry<T>>,
}

impl<T> Tracker<T> {
    /// Creates a new tracker.
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Adds a new tracked player.
    pub fn add_player(&mut self, player: Player, data: T) {
        self.entries.push(Entry::new(player, data))
    }

    /// Adds a new tracked player with default data.
    pub fn add_player_default(&mut self, player: Player)
    where
        T: Default,
    {
        self.add_player(player, T::default())
    }

    /// Removes a tracked player, returning the [`Entry`] if they were tracked.
    pub fn remove_player(&mut self, id: usize) -> Option<Entry<T>> {
        self.entries
            .iter()
            .position(|entry| entry.player.id == id)
            .map(|index| self.entries.remove(index))
    }

    /// Returns a reference to the local player (self).
    pub fn get_self(&self) -> Option<&Entry<T>> {
        self.entries.iter().find(|entry| entry.player.is_self)
    }

    /// Returns a mutable reference to the local player (self).
    pub fn get_self_mut(&mut self) -> Option<&mut Entry<T>> {
        self.entries.iter_mut().find(|entry| entry.player.is_self)
    }

    /// Returns a reference to a tracked player entry.
    pub fn player(&self, id: usize) -> Option<&Entry<T>> {
        self.entries.iter().find(|entry| entry.player.id == id)
    }

    /// Returns a mutable reference to a tracked player entry.
    pub fn player_mut(&mut self, id: usize) -> Option<&mut Entry<T>> {
        self.entries.iter_mut().find(|entry| entry.player.id == id)
    }
}

impl<T> Default for Tracker<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ops::Deref for Tracker<T> {
    type Target = [Entry<T>];

    fn deref(&self) -> &Self::Target {
        self.entries.as_slice()
    }
}

impl<T> ops::DerefMut for Tracker<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.entries.as_mut_slice()
    }
}
