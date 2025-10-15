use std::ops::{Deref, DerefMut};

use rand::Rng as _;

pub type ModifierValue = i32;

#[derive(Debug, Clone, Copy)]
pub struct Modifier {
    pub kind: ModifierKind,
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

    #[allow(unused, reason = "Debug function")]
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
pub enum ModifierKind {
    /// Base amount of wood added to the stockyard per second when the player uses Gather Wood in hundredths
    WoodBase,
    /// Adds to this item's base wood gain by hundreds of the modifier's value
    WoodBaseGain,
    WoodMultiplier,
    WoodAffinityChanceMultiplier, // UNIMPLED
    WoodAffinityMultiplier, // UNIMPLED
    WoodAffinityTimeMultiplier, // UNIMPLED

    /// Base amount of stone added to the stockyard per second when the player uses Gather Stone in hundredths
    StoneBase,
    StoneBaseGain,
    StoneMultiplier,
    StoneAffinityChanceMultiplier, // UNIMPLED
    StoneAffinityMultiplier, // UNIMPLED
    StoneAffinityTimeMultiplier, // UNIMPLED

    ToolAffinityChanceMultiplier, // UNIMPLED
}

impl ModifierKind {
    pub fn display_actual(&self, actual: i32) -> String {
        fn sign(n: i32) -> char {
            if n > 0 { '+' } else { '-' }
        }

        fn percent(n: i32) -> f32 {
            n as f32 / 100.0
        }

        match *self {
            ModifierKind::WoodBase => format!("Chopping wood gives {} wood per second", percent(actual)),
            ModifierKind::WoodBaseGain => format!("{}{} Wood chopped per second", sign(actual), percent(actual)),
            ModifierKind::WoodMultiplier => format!("{}{}% Wood chopped per second", sign(actual), actual),
            ModifierKind::WoodAffinityChanceMultiplier => format!("{}{}% Wood affinity chance", sign(actual), actual),
            ModifierKind::WoodAffinityMultiplier => format!("{}{}% Wood affinity gain", sign(actual), actual),
            ModifierKind::WoodAffinityTimeMultiplier => format!("{}{}% Wood affinity time", sign(actual), actual),

            ModifierKind::StoneBase => format!("Mining stone gives {} stone per second", percent(actual)),
            ModifierKind::StoneBaseGain => format!("{}{} Stone mined per second", sign(actual), actual),
            ModifierKind::StoneMultiplier => format!("{}{}% Stone mined per second", sign(actual), actual),
            ModifierKind::StoneAffinityChanceMultiplier => format!("{}{} Stone affinity chance", sign(actual), actual),
            ModifierKind::StoneAffinityMultiplier => format!("{}{}% Stone affinity gain", sign(actual), actual),
            ModifierKind::StoneAffinityTimeMultiplier => format!("{}{}% Sone affinity time", sign(actual), actual),

            ModifierKind::ToolAffinityChanceMultiplier => format!("{}{} Tool action affinity chance", sign(actual), percent(actual)),
        }
    }
}

pub(crate) fn initialize_implicits() -> Vec<Implicit> {
    let mods = vec![
        Affix::new("Tier0ToolsGatherWoodBase".to_string(), Modifier { kind: ModifierKind::WoodBase, min: 50, max: 50 }),
        Affix::new("Tier0ToolsGatherStoneBase".to_string(), Modifier { kind: ModifierKind::StoneBase, min: 50, max: 50 }),
        Affix::new("Tier1ToolsGatherWoodBase".to_string(), Modifier { kind: ModifierKind::WoodBase, min: 80, max: 120 }),
        Affix::new("Tier1ToolsGatherStoneBase".to_string(), Modifier { kind: ModifierKind::StoneBase, min: 80, max: 120 }),
    ];

    mods.into_iter().map(Implicit).collect()
}

pub(crate) fn initialize_prefixes() -> Vec<Prefix> {
    let mods = vec![
        Affix::new("Lumberjack's".to_string(), Modifier { kind: ModifierKind::WoodBaseGain, min: 10, max: 20 }),
    ];

    mods.into_iter().map(Prefix).collect()
}

pub(crate) fn initialize_suffixes() -> Vec<Suffix> {
    let mods = vec![
        Affix::new("ingenuity".to_string(), Modifier { kind: ModifierKind::ToolAffinityChanceMultiplier, min: 50, max: 100 }),
    ];

    mods.into_iter().map(Suffix).collect()
}