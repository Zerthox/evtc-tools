use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

/// Buff granted by the 6th piece of a Superior Rune.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    IntoPrimitive,
    TryFromPrimitive,
)]
#[repr(u32)]
pub enum Rune {
    #[serde(rename = "Rune of Altruism")]
    Altruism = 16515,

    #[serde(rename = "Rune of Antitoxin")]
    Antitoxin = 21822,

    #[serde(rename = "Rune of Balthazar")]
    Balthazar = 53263,

    #[serde(rename = "Rune of Divinity")]
    Divinity = 24092,

    #[serde(rename = "Rune of Durability")]
    Durability = 33761,

    #[serde(rename = "Rune of Dwayna")]
    Dwayna = 53281,

    #[serde(rename = "Rune of Evasion")]
    Evasion = 25514,

    #[serde(rename = "Rune of Exuberance")]
    Exuberance = 24251,

    #[serde(rename = "Rune of Fireworks")]
    Fireworks = 54858,

    #[serde(rename = "Rune of Grenth")]
    Grenth = 9623,

    #[serde(rename = "Rune of Hoelbrak")]
    Hoelbrak = 9581,

    #[serde(rename = "Rune of Infiltration")]
    Infiltration = 9580,

    #[serde(rename = "Rune of Infiltration (Cooldown)")]
    InfiltrationCooldown = 54865,

    #[serde(rename = "Rune of Leadership")]
    Leadership = 31943,

    #[serde(rename = "Rune of Lyssa")]
    Lyssa = 9625,

    #[serde(rename = "Rune of Melandru")]
    Melandru = 9619,

    #[serde(rename = "Rune of Mercy")]
    Mercy = 9494,

    #[serde(rename = "Rune of Nature's Bounty")]
    NaturesBounty = 38542,

    #[serde(rename = "Rune of Orr")]
    Orr = 9631,

    #[serde(rename = "Rune of Perplexity")]
    Perplexity = 53357,

    #[serde(rename = "Rune of Radiance")]
    Radiance = 25517,

    #[serde(rename = "Rune of Rage")]
    Rage = 9583,

    #[serde(rename = "Rune of RataSum")]
    RataSum = 9594,

    #[serde(rename = "Rune of Resistance")]
    Resistance = 22147,

    #[serde(rename = "Rune of the Renegade")]
    Renegade = 40695,

    #[serde(rename = "Rune of Sanctuary")]
    Sanctuary = 9645,

    #[serde(rename = "Rune of Scavenging")]
    Scavenging = 21049,

    #[serde(rename = "Rune of the Scholar")]
    Scholar = 9620,

    #[serde(rename = "Rune of Snowfall")]
    Snowfall = 26267,

    #[serde(rename = "Rune of Speed")]
    Speed = 53351,

    #[serde(rename = "Rune of Strength")]
    Strength = 9582,

    #[serde(rename = "Rune of Surging")]
    Surging = 33026,

    #[serde(rename = "Rune of Svanir")]
    Svanir = 9637,

    #[serde(rename = "Rune of Water")]
    Water = 9598,

    #[serde(rename = "Rune of Thorns")]
    Thorns = 32456,

    #[serde(rename = "Rune of the Adventurer")]
    Adventurer = 9633,

    #[serde(rename = "Rune of the Afflicted")]
    Afflicted = 9585,

    #[serde(rename = "Rune of the Air")]
    Air = 9640,

    #[serde(rename = "Rune of the Aristocracy")]
    Aristocracy = 21048,

    #[serde(rename = "Rune of the Balefire")]
    Balefire = 53320,

    #[serde(rename = "Rune of the Berserker")]
    Berserker = 33124,

    #[serde(rename = "Rune of the Brawler")]
    Brawler = 9632,

    #[serde(rename = "Rune of the Cavalier =")]
    Cavalier = 40468,

    #[serde(rename = "Rune of the Centaur")]
    Centaur = 9642,

    #[serde(rename = "Rune of the Chronomancer")]
    Chronomancer = 34076,

    #[serde(rename = "Rune of the Citadel")]
    Citadel = 9588,

    #[serde(rename = "Rune of the Daredevil")]
    Daredevil = 32107,

    #[serde(rename = "Rune of the Deadeye")]
    Deadeye = 43961,

    #[serde(rename = "Rune of the Defender")]
    Defender = 53284,

    #[serde(rename = "Rune of the Dolyak")]
    Dolyak = 9586,

    #[serde(rename = "Rune of the Dragonhunter")]
    Dragonhunter = 33949,

    #[serde(rename = "Rune of the Druid")]
    Druid = 32121,

    #[serde(rename = "Rune of the Eagle")]
    Eagle = 9483,

    #[serde(rename = "Rune of the Earth")]
    Earth = 9641,

    #[serde(rename = "Rune of the Elementalist")]
    Elementalist = 9615,

    #[serde(rename = "Rune of the Engineer")]
    Engineer = 9616,

    #[serde(rename = "Rune of the Fire")]
    Fire = 53373,

    #[serde(rename = "Rune of the Firebrand")]
    Firebrand = 41177,

    #[serde(rename = "Rune of the Flame Legion")]
    FlameLegion = 9617,

    #[serde(rename = "Rune of the Flock")]
    Flock = 24192,

    #[serde(rename = "Rune of the Forgeman")]
    Forgeman = 9689,

    #[serde(rename = "Rune of the Golemancer")]
    Golemancer = 9654,

    #[serde(rename = "Rune of the Grove")]
    Grove = 9596,

    #[serde(rename = "Rune of the Guardian")]
    Guardian = 9622,

    #[serde(rename = "Rune of the Herald")]
    Herald = 32545,

    #[serde(rename = "Rune of the Holosmith")]
    Holosmith = 4573,

    #[serde(rename = "Rune of the Ice")]
    Ice = 53361,

    #[serde(rename = "Rune of the Krait")]
    Krait = 24178,

    #[serde(rename = "Rune of the Lich")]
    Lich = 9590,

    #[serde(rename = "Rune of the Mad King")]
    MadKing = 15264,

    #[serde(rename = "Rune of the Mesmer")]
    Mesmer = 9646,

    #[serde(rename = "Rune of the Mirage")]
    Mirage = 45650,

    #[serde(rename = "Rune of the Monk")]
    Monk = 24125,

    #[serde(rename = "Rune of the Necromancer")]
    Necromancer = 9626,

    #[serde(rename = "Rune of the Nightmare")]
    Nightmare = 35543,

    #[serde(rename = "Rune of the Ogre")]
    Ogre = 53349,

    #[serde(rename = "Rune of the Pack")]
    Pack = 24253,

    #[serde(rename = "Rune of the Privateer")]
    Privateer = 53316,

    #[serde(rename = "Rune of the Ranger")]
    Ranger = 9624,

    #[serde(rename = "Rune of the Rebirth")]
    Rebirth = 41444,

    #[serde(rename = "Rune of the Rebirth (Cooldown)")]
    RebirthCooldown = 39944,

    #[serde(rename = "Rune of the Reaper")]
    Reaper = 33424,

    #[serde(rename = "Rune of the Revenant")]
    Revenant = 27710,

    #[serde(rename = "Rune of the Scourge")]
    Scourge = 40404,

    #[serde(rename = "Rune of the Scrapper")]
    Scrapper = 33088,

    #[serde(rename = "Rune of the Soulbeast")]
    Soulbeast = 40264,

    #[serde(rename = "Rune of the Spellbreaker")]
    Spellbreaker = 42674,

    #[serde(rename = "Rune of the Stars")]
    Stars = 47195,

    #[serde(rename = "Rune of the Sunless")]
    Sunless = 21457,

    #[serde(rename = "Rune of the Tempest")]
    Tempest = 31944,

    #[serde(rename = "Rune of the Thief")]
    Thief = 9638,

    #[serde(rename = "Rune of the Trapper")]
    Trapper = 25528,

    #[serde(rename = "Rune of the Traveler")]
    Traveler = 24087,

    #[serde(rename = "Rune of the Trooper")]
    Trooper = 9644,

    #[serde(rename = "Rune of the Undead")]
    Undead = 9711,

    #[serde(rename = "Rune of the Warrior")]
    Warrior = 9647,

    #[serde(rename = "Rune of the Weaver")]
    Weaver = 45737,

    #[serde(rename = "Rune of the Wurm")]
    Wurm = 24096,

    #[serde(rename = "Rune of the Zephyrite")]
    Zephyrite = 51454,

    #[serde(rename = "Rune of Tormenting")]
    Tormenting = 20479,

    #[serde(rename = "Rune of Vampirism")]
    Vampirism = 9592,
}
