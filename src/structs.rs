use enum_map::EnumMap;



pub struct Game {
	pub players: Vec<Player>,
	pub battlefield: Vec<Permanent>,
	pub exile: Vec<(Card, ExileState)>,
	pub command_zone: Vec<Card>,
	pub stack: Vec<Effect>,
	pub active_player: u32
}

pub struct Player {
	pub name: String,
	pub library: Vec<Card>,
	pub hand: Vec<Card>,
	pub graveyard: Vec<Card>,
	pub counters: Vec<(String, u32)>,
	pub emblems: Vec<Emblem>
}

pub struct Card {
	pub name: String,
	pub owner: u32,
	pub types: EnumMap<CardType, bool>,
	pub supertypes: EnumMap<Supertype, bool>,
	pub cost: Vec<ManaSymbol>,
	pub function: CardFunction,
	pub abilities: Vec<Ability>,
	pub counters: Vec<(String, u32)>
}

impl Card {
	pub fn mana_value(&self) -> u32 {
		self.cost.iter().map(|ms| ms.mana_value()).sum()
	}
}

pub struct Permanent {
	pub name: String,
	pub controller: u32,
	pub types: EnumMap<PermanentType, bool>,
	pub abilities: Vec<Ability>
}

pub struct Effect {
	
}

pub struct Emblem {
	pub name: String,
	pub abilities: Vec<Ability>
}







pub enum TurnPhase {
	Untap,
	Upkeep,
	Draw,
	Main { post_combat: bool },
	DeclareAttackers,
	DeclareBlockers,
	AfterFirstStrikeDamage,
	AfterCombatDamage,
	End
}

pub enum CardFunction {
	Permanent(Permanent),
	Effect(Vec<Effect>)
}

#[derive(Enum, Debug)]
pub enum CardType {
	Creature,
	Instant,
	Sorcery,
	Artifact,
	Enchantment,
	Land,
	Planeswalker,
	Tribal
}

#[derive(Enum, Debug)]
pub enum PermanentType {
	Creature,
	Artifact,
	Enchantment,
	Land,
	Planeswalker
}

#[derive(Enum, Debug)]
pub enum Supertype {
	Basic,
	Legendary,
	Token,
	Snow,
	World,
	Elite
}

// basic legendary elite tribal world snow token enchantment artifact land creature planeswalker instant sorcery

pub enum ManaSymbol {
	Single(ManaSymbolType),
	Hybrid(ManaSymbolType, ManaSymbolType)
}

impl ManaSymbol {
	pub fn mana_value(&self) -> u32 {
		match self {
			ManaSymbol::Single(m) => m.mana_value(),
			ManaSymbol::Hybrid(m1, m2) => u32::max(m1.mana_value(), m2.mana_value())
		}
	}
}

pub enum ManaSymbolType {
	Colored(ManaColor),
	Phyrexian(ManaColor),
	Generic(u32)
}

impl ManaSymbolType {
	pub fn mana_value(&self) -> u32 {
		match self {
			ManaSymbolType::Generic(n) => *n,
			_ => 1
		}
	}
}

pub enum ManaColor {
	White,
	Blue,
	Black,
	Red,
	Green,
	Colorless
}

pub enum Color {
	White,
	Blue,
	Black,
	Red,
	Green
}

pub enum ExileState {
	FaceUp,
	FaceDown(u32)
}

pub enum Ability {
	Activated {
		
	},
	Triggered {
		
	}
}

