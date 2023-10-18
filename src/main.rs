extern crate rand;
extern crate enum_map;
#[macro_use] extern crate enum_map_derive;

use enum_map::enum_map;
use rand::Rng;

mod structs;

use crate::structs::{Game, Player, Card, CardInfo};




fn main() {
	println!("Hello, world!");
	
	let mut random = rand::thread_rng();
	
	let mut game = Game::new(Box::new([
		Player::new("Ben"),
		Player::new("Michael")
	]));
	
	for i in 0..game.players.len() {
		for _ in 0..10 {
			game.players[i].library.push(Card {
				name: "enemy guy".into(),
				owner: i,
				info: CardInfo::Creature {
					cost: random.gen_range(1..=10),
					power: random.gen_range(1..=10),
					toughness: random.gen_range(1..=10)
				}
			});
		}
		for _ in 0..10 {
			game.players[i].library.push(Card {
				name: "cool land".into(),
				owner: i,
				info: CardInfo::Land
			})
		}
	}
	
	
	for player in game.players.iter_mut() {
		for _ in 0..5 {
			player.hand.push(player.library.pop().unwrap());
		}
	}
	
	
	loop {
		for permanent in &mut game.battlefield {
			permanent.tapped = false;
		}
		game.ap().draw(1);
		
		
	}
	
	
}
