use num_enum::{FromPrimitive, IntoPrimitive};
use std::cmp;
use strum_macros::{Display, IntoStaticStr};

#[cfg(feature = "serde")]
use serde_crate::{Deserialize, Serialize};

/// Struct representing a player.
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub struct Player {
    /// Player id given by the game.
    pub id: usize,

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
    pub fn new<C, A>(
        id: usize,
        character: C,
        account: A,
        is_self: bool,
        profession: Profession,
        elite: Specialization,
        subgroup: usize,
    ) -> Self
    where
        C: Into<String>,
        A: Into<String>,
    {
        Self {
            id,
            character: character.into(),
            account: account.into(),
            is_self,
            profession,
            elite,
            subgroup,
            combat: false,
        }
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

/// Player profession.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    IntoPrimitive,
    FromPrimitive,
    Display,
    IntoStaticStr,
)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
#[repr(u32)]
pub enum Profession {
    /// Unknown or invalid.
    #[num_enum(default)]
    Unknown = 0,

    Guardian = 1,
    Warrior = 2,
    Engineer = 3,
    Ranger = 4,
    Thief = 5,
    Elementalist = 6,
    Mesmer = 7,
    Necromancer = 8,
    Revenant = 9,
}

/// Player specialization.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    IntoPrimitive,
    FromPrimitive,
    Display,
    IntoStaticStr,
)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
#[repr(u32)]
pub enum Specialization {
    /// Unknown or invalid.
    #[num_enum(default)]
    Unknown = 0,

    // mesmer
    Dueling = 1,
    Domination = 10,
    Inspiration = 23,
    Illusions = 24,
    Chronomancer = 40,
    Chaos = 45,
    Mirage = 59,
    Virtuoso = 66,

    // necromancer
    DeathMagic = 2,
    BloodMagic = 19,
    Reaper = 34,
    Curses = 39,
    SoulReaping = 50,
    Spite = 53,
    Scourge = 60,
    Harbinger = 64,

    // revenant
    Invocation = 3,
    Retribution = 9,
    Corruption = 14,
    Devastation = 15,
    Salvation = 12,
    Herald = 52,
    Renegade = 63,
    Vindicator = 69,

    // warrior
    Strength = 4,
    Tactics = 11,
    Berserker = 18,
    Defense = 22,
    Arms = 36,
    Discipline = 51,
    Spellbreaker = 61,
    Bladesworn = 68,

    // ranger
    Druid = 5,
    Marksmanship = 8,
    NatureMagic = 25,
    Skirmishing = 30,
    Beastmastery = 32,
    WildernessSurvival = 33,
    Soulbeast = 55,
    Untamed = 72,

    // engineer
    Explosives = 6,
    Tools = 21,
    Alchemy = 29,
    Firearms = 38,
    Scrapper = 43,
    Inventions = 47,
    Holosmith = 57,
    Mechanist = 70,

    // thief
    Daredevil = 7,
    ShadowArts = 20,
    DeadlyArts = 28,
    CriticalStrikes = 35,
    Trickery = 44,
    Acrobatics = 54,
    Deadeye = 58,
    Specter = 71,

    // guardian
    Valor = 13,
    Radiance = 16,
    Dragonhunter = 27,
    Zeal = 42,
    Virtues = 46,
    Honor = 49,
    Firebrand = 62,
    Willbender = 65,

    // elementalist
    Water = 17,
    Earth = 26,
    Fire = 31,
    Arcane = 37,
    Air = 41,
    Tempest = 48,
    Weaver = 56,
    Catalyst = 67,
}
