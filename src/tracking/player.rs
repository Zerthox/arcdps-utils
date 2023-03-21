use arcdps::{strip_account_prefix, Agent, Profession, Specialization};
use std::cmp;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Struct representing a player.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Player {
    /// Player id given by the game.
    pub id: usize,

    // Player instance id on map
    pub instance_id: usize,

    /// Player character name.
    pub character: String,

    /// Player account name.
    pub account: String,

    /// Whether the player is the local player.
    pub is_self: bool,

    /// Profession (class) of the player character.
    pub profession: Profession,

    /// Current elite specialization the player has equipped.
    pub elite: Specialization,

    /// Current squad subgroup the player is in.
    pub subgroup: usize,

    /// Whether the player is currently in combat.
    pub combat: bool,
}

impl Player {
    /// Creates a new player.
    pub fn new(
        id: usize,
        instance_id: usize,
        character: impl Into<String>,
        account: impl Into<String>,
        is_self: bool,
        profession: Profession,
        elite: Specialization,
        subgroup: usize,
    ) -> Self {
        Self {
            id,
            instance_id,
            character: character.into(),
            account: account.into(),
            is_self,
            profession,
            elite,
            subgroup,
            combat: false,
        }
    }

    /// Creates a new player from tracking change agents.
    pub fn from_tracking_change(src: Agent, dst: Agent) -> Option<Self> {
        debug_assert!(src.elite == 0 && src.prof != 0);

        let acc_name = dst.name?;
        Some(Self::new(
            src.id,
            dst.id,
            src.name?,
            strip_account_prefix(acc_name),
            dst.is_self != 0,
            dst.prof.into(),
            dst.elite.into(),
            dst.team as usize,
        ))
    }

    /// Enters the player into combat.
    pub fn enter_combat(&mut self, new_subgroup: Option<usize>) {
        self.combat = true;
        if let Some(sub) = new_subgroup {
            self.subgroup = sub;
        }
    }

    /// Exits the player from combat.
    pub fn exit_combat(&mut self) {
        self.combat = false;
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Player {}

impl cmp::PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl cmp::Ord for Player {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.id.cmp(&other.id)
    }
}
