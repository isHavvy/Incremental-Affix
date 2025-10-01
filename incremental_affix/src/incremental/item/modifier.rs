use std::ops::{Deref, DerefMut};

use rand::Rng as _;

pub type ModifierValue = i32;

#[derive(Debug, Clone, Copy)]
pub struct Modifier {
    pub kind: super::Modifiers,
    pub min: ModifierValue,
    pub max: ModifierValue,
}

impl Modifier {
    fn display_actual(&self, actual: ModifierValue) -> String {
        self.kind.display_actual(actual)
    }

    fn random_modifier_value(&self) -> ModifierValue {
        rand::thread_rng().gen_range(self.min..self.max + 1)
    }
}

#[derive(Debug, Clone)]
pub struct Affix {
    #[expect(unused)] // Have not implemented deeper inspection.
    pub name: String,
    pub modifier: Modifier,
    pub modifier_actual: ModifierValue,
    pub hybrid_modifier: Option<Modifier>,
    pub hybrid_modifier_actual: ModifierValue,
}

impl Affix {
    /// Construct a new affix with the given modifier. Sets the hybrid to `None`.
    pub fn new(name: String, modifier: Modifier) -> Self {
        Self {
            name,
            modifier,
            modifier_actual: if modifier.min == modifier.max { modifier.min } else { 0 },
            hybrid_modifier: None,
            hybrid_modifier_actual: 0,
        }
    }

    pub fn display(&self) -> String {
        let mut output = String::new();

        output.push_str(&self.modifier.display_actual(self.modifier_actual));
        if let Some(hybrid_modifier) = self.hybrid_modifier {
            output.push('\n');
            output.push_str(&hybrid_modifier.display_actual(self.hybrid_modifier_actual));
        }

        output
    }

    pub fn randomize_actual(&mut self) {
        self.modifier_actual = self.modifier.random_modifier_value();
        if let Some(modifier) = self.hybrid_modifier {
            self.hybrid_modifier_actual = modifier.random_modifier_value();
        }
    }

    pub fn modifiers(&self) -> impl Iterator<Item=(&Modifier, ModifierValue)> {
        [
            Some((&self.modifier, self.modifier_actual)),
            (self.hybrid_modifier.as_ref().map(|hybrid| (hybrid, self.hybrid_modifier_actual)))
        ].into_iter().filter_map(|x| x) }
}

#[derive(Debug, Clone)]
pub(crate) struct Implicit(pub Affix);

impl Deref for Implicit {
    type Target = Affix;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Implicit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Prefix(pub Affix);

impl Deref for Prefix {
    type Target = Affix;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Prefix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Suffix(pub Affix);

impl Deref for Suffix {
    type Target = Affix;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Suffix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// --- Actual modifiers below --- //

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[expect(unused)]
pub enum Modifiers {
    CanChopWood,
    CanMineStone,

    WoodBaseGain,
    WoodMultiplier,
    WoodAffinityChanceMultiplier,
    WoodAffinityMultiplier,
    WoodAffinityTimeMultiplier,

    StoneBaseGain,
    StoneMultiplier,
    StoneAffinityChanceMultiplier,
    StoneAffinityMultiplier,
    StoneAffinityTimeMultiplier,
}

impl Modifiers {
    pub fn display_actual(&self, actual: i32) -> String {
        fn sign(n: i32) -> char {
            if n > 0 { '+' } else { '-' }
        }

        match *self {
            Modifiers::CanChopWood => "You can chop wood".to_string(),
            Modifiers::CanMineStone => "You can mine stone".to_string(),

            Modifiers::WoodBaseGain => format!("{}{} Wood chopped per second", sign(actual), actual),
            Modifiers::WoodMultiplier => format!("{}{}% Wood chopped per second", sign(actual), actual),
            Modifiers::WoodAffinityChanceMultiplier => format!("{}{}% Wood affinity chance", sign(actual), actual),
            Modifiers::WoodAffinityMultiplier => format!("{}{}% Wood affinity gain", sign(actual), actual),
            Modifiers::WoodAffinityTimeMultiplier => format!("{}{}% Wood affinity time", sign(actual), actual),

            Modifiers::StoneBaseGain => format!("{}{} Stone mined per second", sign(actual), actual),
            Modifiers::StoneMultiplier => format!("{}{}% Stone mined per second", sign(actual), actual),
            Modifiers::StoneAffinityChanceMultiplier => format!("{}{} Stone affinity chance", sign(actual), actual),
            Modifiers::StoneAffinityMultiplier => format!("{}{}% Stone affinity gain", sign(actual), actual),
            Modifiers::StoneAffinityTimeMultiplier => format!("{}{}% Sone affinity time", sign(actual), actual),
        }
    }
}

pub(crate) fn initialize_implicits() -> Vec<Implicit> {
    let mods = vec![
        Affix::new("CanChopWood".to_string(), Modifier { kind: Modifiers::CanChopWood, min: 1, max: 1 }),
        Affix::new("CanMineStone".to_string(), Modifier { kind: Modifiers::CanMineStone, min: 1, max: 1 })
    ];

    mods.into_iter().map(Implicit).collect()
}