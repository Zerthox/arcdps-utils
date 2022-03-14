//! General GW2 related data.
//!
//! This is also used in the ArcDPS API, but may be useful outside.

mod player;

pub use player::*;

use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};
use strum_macros::{Display, IntoStaticStr};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// GW2 client language.
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
    TryFromPrimitive,
    Display,
    IntoStaticStr,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum Language {
    English = 0,
    French = 2,
    German = 3,
    Spanish = 4,
    Chinese = 5,
}

// TODO: document unclear attributes
/// Buff formula attributes.
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u16)]
pub enum Attribute {
    None,

    Power,
    Precision,
    Toughness,
    Vitality,
    Ferocity,
    Healing,
    Condition,
    Concentration,
    Expertise,

    Armor,
    Agony,
    StatInc,
    FlatInc,
    PhysInc,
    CondInc,
    PhysRec,
    CondRec,
    Attackspeed,
    SiphonInc,
    SiphonRec,

    /// Unknown or invalid.
    #[num_enum(default)]
    Unknown = 65535,
}
