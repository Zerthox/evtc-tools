use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

/// Buff granted by a Relic.
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
pub enum Relic {
    #[serde(rename = "Relic of Agony")]
    Agony = 76234,

    #[serde(rename = "Relic of Akeem")]
    Akeem = 70452,

    #[serde(rename = "Relic of Altruism")]
    Altruism = 75614,

    #[serde(rename = "Relic of Antitoxin")]
    Antitoxin = 70735,

    #[serde(rename = "Relic of Atrocity")]
    Atrocity = 74275,

    #[serde(rename = "Relic of Bava Nisos")]
    BavaNisos = 76187,

    #[serde(rename = "Relic of Bloodstone")]
    Bloodstone = 76336,

    #[serde(rename = "Relic of Castora")]
    Castora = 78087,

    #[serde(rename = "Relic of Cerus")]
    Cerus = 70615,

    #[serde(rename = "Relic of Dagda")]
    Dagda = 69626,

    #[serde(rename = "Relic of Durability")]
    Durability = 70943,

    #[serde(rename = "Relic of Dwayna")]
    Dwayna = 71277,

    #[serde(rename = "Relic of Evasion")]
    Evasion = 69689,

    #[serde(rename = "Relic of Febe")]
    Febe = 71640,

    #[serde(rename = "Relic of Fire")]
    Fire = 75978,

    #[serde(rename = "Relic of Fireworks")]
    Fireworks = 71259,

    #[serde(rename = "Relic of Geysers")]
    Geysers = 74959,

    #[serde(rename = "Relic of Isgarren")]
    Isgarren = 69740,

    #[serde(rename = "Relic of Karakosa")]
    Karakosa = 71356,

    #[serde(rename = "Relic of Leadership")]
    Leadership = 70871,

    #[serde(rename = "Relic of Lyhr")]
    Lyhr = 70447,

    #[serde(rename = "Relic of Mabon")]
    Mabon = 70960,

    #[serde(rename = "Relic of Mercy")]
    Mercy = 69559,

    #[serde(rename = "Relic of Mistburn")]
    Mistburn = 76254,

    #[serde(rename = "Relic of Mosyn")]
    Mosyn = 72845,

    #[serde(rename = "Relic of Mount Balrior")]
    MountBalrior = 75186,

    #[serde(rename = "Relic of Nayos")]
    Nayos = 71382,

    #[serde(rename = "Relic of Nourys")]
    Nourys = 71505,

    #[serde(rename = "Relic of Peitha")]
    Peitha = 70254,

    #[serde(rename = "Relic of Resistance")]
    Resistance = 70688,

    #[serde(rename = "Relic of Reunification")]
    Reunification = 75303,

    #[serde(rename = "Relic of Rivers")]
    Rivers = 73660,

    #[serde(rename = "Relic of Sorrow")]
    Sorrow = 73689,

    #[serde(rename = "Relic of Speed")]
    Speed = 70126,

    #[serde(rename = "Relic of Surging")]
    Surging = 75940,

    #[serde(rename = "Relic of the Adventurer")]
    Adventurer = 70628,

    #[serde(rename = "Relic of the Afflicted")]
    Afflicted = 69523,

    #[serde(rename = "Relic of the Aristocracy")]
    Aristocracy = 71257,

    #[serde(rename = "Relic of the Astral Ward")]
    AstralWard = 70448,

    #[serde(rename = "Relic of the Beehive")]
    Beehive = 74880,

    #[serde(rename = "Relic of the Biomancer")]
    Biomancer = 78041,

    #[serde(rename = "Relic of the Blightbringer")]
    Blightbringer = 74494,

    #[serde(rename = "Relic of the Brawler")]
    Brawler = 69971,

    #[serde(rename = "Relic of the Cavalier")]
    Cavalier = 69672,

    #[serde(rename = "Relic of the Centaur")]
    Centaur = 70060,

    #[serde(rename = "Relic of the Chronomancer")]
    Chronomancer = 69864,

    #[serde(rename = "Relic of the Citadel")]
    Citadel = 70388,

    #[serde(rename = "Relic of the Claw")]
    Claw = 74181,

    #[serde(rename = "Relic of the Daredevil")]
    Daredevil = 70744,

    #[serde(rename = "Relic of the Deadeye")]
    Deadeye = 71112,

    #[serde(rename = "Relic of the Defender")]
    Defender = 70010,

    #[serde(rename = "Relic of the Demon Queen")]
    DemonQueen = 71678,

    #[serde(rename = "Relic of the Dragonhunter")]
    Dragonhunter = 70680,

    #[serde(rename = "Relic of the Eagle")]
    Eagle = 75656,

    #[serde(rename = "Relic of the Earth")]
    Earth = 76028,

    #[serde(rename = "Relic of the Firebrand")]
    Firebrand = 70599,

    #[serde(rename = "Relic of the First Revenant")]
    FirstRevenant = 78005,

    #[serde(rename = "Relic of the Flock")]
    Flock = 70992,

    #[serde(rename = "Relic of the Founding")]
    Founding = 72548,

    #[serde(rename = "Relic of the Fractal")]
    Fractal = 70047,

    #[serde(rename = "Relic of the Golemancer")]
    Golemancer = 75474,

    #[serde(rename = "Relic of the Herald")]
    Herald = 69898,

    #[serde(rename = "Relic of the Holosmith")]
    Holosmith = 75584,

    #[serde(rename = "Relic of the Ice")]
    Ice = 70989,

    #[serde(rename = "Relic of the Krait")]
    Krait = 69772,

    #[serde(rename = "Relic of the Lich")]
    Lich = 75975,

    #[serde(rename = "Relic of the Living City")]
    LivingCity = 76284,

    #[serde(rename = "Relic of the Midnight King")]
    MidnightKing = 71384,

    #[serde(rename = "Relic of the Mirage")]
    Mirage = 70216,

    #[serde(rename = "Relic of the Mist Stranger")]
    MistStranger = 78806,

    #[serde(rename = "Relic of the Mists Tide")]
    MistsTide = 75356,

    #[serde(rename = "Relic of the Monk")]
    Monk = 69912,

    #[serde(rename = "Relic of the Necromancer")]
    Necromancer = 69915,

    #[serde(rename = "Relic of the Nightmare")]
    Nightmare = 69720,

    #[serde(rename = "Relic of the Ogre")]
    Ogre = 75992,

    #[serde(rename = "Relic of the Pack")]
    Pack = 70438,

    #[serde(rename = "Relic of the Phenom")]
    Phenom = 76433,

    #[serde(rename = "Relic of the Pirate Queen")]
    PirateQueen = 78328,

    #[serde(rename = "Relic of the Privateer")]
    Privateer = 75724,

    #[serde(rename = "Relic of the Reaper")]
    Reaper = 71287,

    #[serde(rename = "Relic of the Scoundrel")]
    Scoundrel = 78083,

    #[serde(rename = "Relic of the Scourge")]
    Scourge = 70532,

    #[serde(rename = "Relic of the Sorcerer")]
    Sorcerer = 72447,

    #[serde(rename = "Relic of the Steamshrieker")]
    Steamshrieker = 75053,

    #[serde(rename = "Relic of the Stormsinger")]
    Stormsinger = 74029,

    #[serde(rename = "Relic of the Sunless")]
    Sunless = 71058,

    #[serde(rename = "Relic of the Thief")]
    Thief = 69914,

    #[serde(rename = "Relic of the Trooper")]
    Trooper = 70224,

    #[serde(rename = "Relic of the Twin Generals")]
    TwinGenerals = 72475,

    #[serde(rename = "Relic of the Unseen Invasion")]
    UnseenInvasion = 70596,

    #[serde(rename = "Relic of the Warrior")]
    Warrior = 71070,

    #[serde(rename = "Relic of the Water")]
    Water = 70295,

    #[serde(rename = "Relic of the Wayfinder")]
    Wayfinder = 72768,

    #[serde(rename = "Relic of the Weaver")]
    Weaver = 71004,

    #[serde(rename = "Relic of the Wizard's Tower")]
    WizardsTower = 70777,

    #[serde(rename = "Relic of the Zephyrite")]
    Zephyrite = 71014,

    #[serde(rename = "Relic of Thorns")]
    Thorns = 75453,

    #[serde(rename = "Relic of Vampirism")]
    Vampirism = 75846,

    #[serde(rename = "Relic of Vass")]
    Vass = 70697,

    #[serde(rename = "Relic of Zakiros")]
    Zakiros = 72510,
}
