use super::Player;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Struct representing a tracker entry.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Entry<T> {
    /// Player this entry corresponds to.
    pub player: Player,

    /// Data associated with the player.
    pub data: T,
}

impl<T> Entry<T> {
    /// Creates a new entry.
    pub fn new(player: Player, data: T) -> Self {
        Self { player, data }
    }
}

impl<T> From<Player> for Entry<T>
where
    T: Default,
{
    fn from(player: Player) -> Self {
        Self::new(player, T::default())
    }
}
