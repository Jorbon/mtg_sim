

pub struct Game {
	pub players: Box<[Player]>,
	pub battlefield: Vec<Permanent>,
	pub active_player: usize
}

impl Game {
	pub fn new(players: Box<[Player]>) -> Self {
		Self {
			players: players,
			battlefield: vec![],
			active_player: 0
		}
	}
}

impl Game {
	pub fn ap(&self) -> &Player {
		&self.players[self.active_player]
	}
}

pub struct Player {
	pub name: Box<str>,
	pub hand: Vec<Card>,
	pub library: Vec<Card>,
	pub graveyard: Vec<Card>,
	pub mana_pool: u32
}

impl Player {
	pub fn new(name: &str) -> Self {
		Self {
			name: name.into(),
			hand: vec![],
			library: vec![],
			graveyard: vec![],
			mana_pool: 0
		}
	}
	pub fn draw(&mut self, n: u32) {
		for _ in 0..n {
			self.hand.push(self.library.pop().unwrap());
		}
	}
}

pub struct Card {
	pub name: Box<str>,
	pub owner: usize,
	pub info: CardInfo
}

impl Card {
	pub fn 
}

pub struct Permanent {
	pub name: Box<str>,
	pub controller: usize,
	pub info: PermanentInfo,
	pub tapped: bool
}



pub enum CardInfo {
	Creature {
		cost: u32,
		power: u32,
		toughness: u32
	},
	Land
}

pub enum PermanentInfo {
	Creature {
		mana_value: u32,
		power: u32,
		toughness: u32,
		damage: u32
	},
	Land
}




