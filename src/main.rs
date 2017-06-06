use std::io::{stdin};

extern crate rand;
extern crate tictactoe;

use tictactoe::tictactoe::*;

use rand::random;

fn main() {
	println!("Player 1: Pick a number from 1 to 100");
	let c1 = get_input();
	
	println!("Player 2: Pick a number from 1 to 100");
	let c2 = get_input();
	
	let invalid_input = |c: char| !c.is_numeric();
	
	if c1.chars().any(&invalid_input) || c2.chars().any(&invalid_input) {
		println!("Invalid input!");
	}
	
	let random_number = (random::<u32>() % 100) + 1;
	
	let random_number = random_number as i32;
	
	println!("The victory number is {}", random_number);
	
	let c1 = convert_to_int(&c1);
	let c2 = convert_to_int(&c2);
	
	let mut players: [Player; 2] = [Player::new('X'), Player::new('O')];
	
	if (c1 - random_number).abs() > (c2 - random_number).abs() {
		println!("{} is closer to {} than {}", c2, random_number, c1);
	} else {
		println!("{} is closer to {} than {}", c1, random_number, c2);
	}
	
	println!("Winner, please enter the symbol you want");
	
	let sym = get_input().pop().unwrap();
	
	if sym == 'O' {
		players = swap(players);
	}
	
	let mut board = Board::new();
	
	loop {
		println!("{:?}'s turn! Choose a square to mark.", players[0].symbol);
		
		println!("{}", board);
		
		loop {
			let p1_choice = get_input();
			
			let y = convert_to_int(&p1_choice);
			
			if !(y > 0 && y < 10) {
				println!("That square doesn't exist!");
				continue;
			}
			
			if !board.mark(y as usize, players[0].symbol) {
				println!("That square is already marked!");
				continue;
			}
			
			break;
		}
		
		if board.check_for_winner() {
			println!("Player 1 won!");
			break;
		}
		
		if board.check_for_unwinnable() {
			println!("This game is unwinnable!");
			break;
		}
		
		println!("{:?}'s turn! Choose a square to mark.", players[1].symbol);
		
		println!("{}", board);
		
		loop {
			let p2_choice = get_input();
			
			let y = convert_to_int(&p2_choice);
			
			if !(y > 0 && y < 10) {
				println!("That square doesn't exist!");
				continue;
			}
			
			if !board.mark(y as usize, players[1].symbol) {
				println!("That square is already marked!");
				continue;
			}
			
			break;
		}
		
		if board.check_for_winner() {
			println!("Player 2 won!");
			break;
		}
		
		if board.check_for_unwinnable() {
			println!("This game is unwinnable!");
			break;
		}
	}
	
	println!("{}", board);
}

fn convert_to_int(s: &String) -> i32 {
	let n: u32 = s.chars().rev().enumerate().map(|(i, x)| (i, x.to_digit(10).unwrap())).fold(0, |acc, (i, x)| acc + (10u32.pow(i as u32) * x));
	
	n as i32
}

fn get_input() -> String {
	let mut s = String::new();
	
	stdin().read_line(&mut s).expect("Failed to read input");
	
	s = s.lines().next().unwrap().to_string();
	
	s
}