#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub mod tictactoe {
	use std::fmt::*;
	use std::cmp::*;
	
	#[derive(Copy, Clone)]
	pub struct Player {
		pub symbol: Symbol,
	}
	
	#[derive(Debug, Copy, Clone)]
	pub struct Board {
		squares: [Square; 9],
	}
	
	impl Board {
		pub fn new() -> Self {
			Board {
				squares: [Square::new(); 9],
			}
		}
		
		pub fn check_for_winner(&self) -> bool {
			let winning_patterns = vec!((0,4,8),(2,4,6),(0,3,6),(1,4,7),(2,5,8),(0,1,2),(3,4,5),(6,7,8));
			
			for i in 0..winning_patterns.len() {
				let winning_pattern = winning_patterns[i];
				
				if self.squares[winning_pattern.0].marked && self.squares[winning_pattern.1].marked && self.squares[winning_pattern.2].marked && (self.squares[winning_pattern.0].mark_symbol == self.squares[winning_pattern.1].mark_symbol) && (self.squares[winning_pattern.1].mark_symbol == self.squares[winning_pattern.2].mark_symbol) {
					return true;
				}
			}
			
			false
		}
		
		pub fn check_for_unwinnable(&self) -> bool {
			false
		}
		
		pub fn mark(&mut self, square_number: usize, symbol: Symbol) -> bool {
			if self.squares[square_number-1].marked {
				return false;
			}
			
			self.squares[square_number-1].marked = true;
			self.squares[square_number-1].mark_symbol = symbol;
			
			true
		}
		
		pub fn get_mark(&self, square_index: usize) -> &str {
			let sq = self.squares[square_index];
			
			if !sq.marked {
				return "";
			}
			
			match sq.mark_symbol {
				Symbol::X => "X",
				Symbol::O => "O",
			}
		}
	}
	
	impl PartialEq for Symbol {
		fn eq(&self, other: &Symbol) -> bool {
			let x = match self {
				&Symbol::X => 0,
				&Symbol::O => 1,
			};
			let y = match other {
				&Symbol::X => 0,
				&Symbol::O => 1,
			};
			
			x == y
		}

		fn ne(&self, other: &Symbol) -> bool {
			let x = match self {
				&Symbol::X => 0,
				&Symbol::O => 1,
			};
			let y = match other {
				&Symbol::X => 0,
				&Symbol::O => 1,
			};
			
			x != y
		}
	}
	
	impl Display for Board {
		fn fmt(&self, f: &mut Formatter) -> Result {
			write!(f, "{symbol:>width$}|{symbol:>width$}|{symbol:>width$}\n", width=6, symbol="");
			write!(f, "{:^width$}|{:^width$}|{:^width$}\n", self.get_mark(0), self.get_mark(1), self.get_mark(2), width=6);
			write!(f, "{:>width$}|{:>width$}|{:>width$}\n", "1", "2", "3", width=6);
			write!(f, "--------------------\n");
			write!(f, "{symbol:>width$}|{symbol:>width$}|{symbol:>width$}\n", width=6, symbol="");
			write!(f, "{:^width$}|{:^width$}|{:^width$}\n", self.get_mark(3), self.get_mark(4), self.get_mark(5), width=6);
			write!(f, "{:>width$}|{:>width$}|{:>width$}\n", "4", "5", "6", width=6);
			write!(f, "--------------------\n");
			write!(f, "{symbol:>width$}|{symbol:>width$}|{symbol:>width$}\n", width=6, symbol="");
			write!(f, "{:^width$}|{:^width$}|{:^width$}\n", self.get_mark(6), self.get_mark(7), self.get_mark(8), width=6);
			write!(f, "{:>width$}|{:>width$}|{:>width$}\n", "7", "8", "9", width=6)
		}
	}
	
	#[derive(Debug, Copy, Clone)]
	pub struct Square {
		marked: bool,
		mark_symbol: Symbol,
	}
	
	impl Square {
		fn new() -> Self {
			Square {
				marked: false,
				mark_symbol: Symbol::X,
			}
		}
	}
	
	#[derive(Debug, Copy, Clone)]
	pub enum Symbol {
		X,
		O,
	}

	impl Player {
		pub fn new(s: char) -> Self {
			let player: Player;
			if s == 'X' {
				player = Player { symbol: Symbol::X };
			} else {
				player = Player { symbol: Symbol::O };
			}
			player
		}
	}
	
	pub fn swap<T: Copy>(array: [T; 2]) -> [T; 2] {
		[array[1], array[0]]
	}
	
}