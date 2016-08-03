extern crate rand;
use rand::Rng;
use std::io;
use std::io::Error;

#[derive(Copy, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
	Empty
}

struct Previous {
    one: Hand,
    two: Hand
}
fn iterate_last(mut prev: &mut Previous, new_hand: Hand) {
    prev.two = prev.one;
    prev.one = new_hand;
}
struct Stats {
    games: u32,
    wins: u32,
    losses: u32,
    ties: u32,
    rocks: u32,
    papers: u32,
    scissors: u32
}
fn get_a_string() -> Result<String, Error> {
    println!("Enter choice (r,p,s) or q to quit >");
    let mut to_return = String::new();
    io::stdin().read_line(&mut to_return).expect("FAIL");
    Ok(to_return)
}


fn empty_stats(mut stat: &mut Stats) {
	stat.games =0;
    stat.wins = 0;
    stat.losses = 0;
    stat.ties = 0;
    stat.rocks = 0;
    stat.papers = 0;
    stat.scissors = 0;
}

fn main() {
	let mut hist: [[Stats; 3]; 3] = [
		[
			Stats {
				games: 0,
				wins: 0,
				losses: 0,
				ties: 0,
				rocks: 0,
				papers: 0,
				scissors: 0,
			},
			Stats {
				games: 0,
				wins: 0,
				losses: 0,
				ties: 0,
				rocks: 0,
				papers: 0,
				scissors: 0,
			},
			Stats {
				games: 0,
				wins: 0,
				losses: 0,
				ties: 0,
				rocks: 0,
				papers: 0,
				scissors: 0,
			},
		],
		[
			Stats {
				games: 0,
				wins: 0,
				losses: 0,
				ties: 0,
				rocks: 0,
				papers: 0,
				scissors: 0,
			},
			Stats {
				games: 0,
				wins: 0,
				losses: 0,
				ties: 0,
				rocks: 0,
				papers: 0,
				scissors: 0,
			},
			Stats {
				games: 0,
				wins: 0,
				losses: 0,
				ties: 0,
				rocks: 0,
				papers: 0,
				scissors: 0,
			},
		],
		[
			Stats {
				games: 0,
				wins: 0,
				losses: 0,
				ties: 0,
				rocks: 0,
				papers: 0,
				scissors: 0,
			},
			Stats {
				games: 0,
				wins: 0,
				losses: 0,
				ties: 0,
				rocks: 0,
				papers: 0,
				scissors: 0,
			},
			Stats {
				games: 0,
				wins: 0,
				losses: 0,
				ties: 0,
				rocks: 0,
				papers: 0,
				scissors: 0,
			},
		],
	];
	let game_stats: Stats = empty_stats(game_stats);
	//Gnome { height: 100, weight: 200 };

	//loop start?
	let mut menu_loop=true;
	let mut current_hand: Hand = Hand::Empty;
	let mut last_hand: Hand = Hand::Empty;
	let mut two_hands_ago: Hand = Hand::Empty;
	let mut computer_hand: Hand = Hand::Empty;
	while menu_loop{
		let mut valid_input=false;
		let mut s;
		match get_a_string() {
			Ok(n) => s = n,
			Err(n) => {
			    println!("ERROR COULD NOT READ!");
				s = String::from("ERROR");
			}
		}
		if s.trim()=="q"{
			println!("Quitting!");
			menu_loop=false;
			//break;
		}else if s.trim()=="r"{
			valid_input=true;
			current_hand = Hand::Rock;
			println!("rock!");
		}else if s.trim()=="p"{
			valid_input=true;
			println!("paper!");
			current_hand = Hand::Paper;
		}else if s.trim()=="s"{
			valid_input=true;
			println!("scissors!");
			current_hand = Hand::Scissors;
		}else{
			println!("invalid input!");
		}
		if valid_input{
			let mut first_index=-1;
			let mut second_index=-1;
			let mut current_index=-1;
			let mut computer_index=-1;
			match last_hand{
				Hand::Rock=>second_index=0,
				Hand::Paper=>second_index=1,
				Hand::Scissors=>second_index=2,
				Hand::Empty=>second_index=-1,
			}
			match two_hands_ago{
				Hand::Rock=>first_index=0,
				Hand::Paper=>first_index=1,
				Hand::Scissors=>first_index=2,
				Hand::Empty=>first_index=-1,
			}
			two_hands_ago=last_hand;
			last_hand=current_hand;
			computer_hand = Hand::Empty;
			if second_index>-1 && first_index>-1{
				//need to get and set history
			}else{
				//rand w/o tracking
				let mut rand_hand;
				rand_hand = rand::thread_rng().gen_range(0, 3);
				if rand_hand==0{
					computer_hand = Hand::Rock;
				}else if rand_hand==1{
					computer_hand = Hand::Paper;
				}else if rand_hand==2{
					computer_hand = Hand::Scissors;
				}

			}
			match current_hand{
				Hand::Rock=>{
					println!("Player chose: Rock");
					current_index=0;
				}
				Hand::Paper=>{
					println!("Player chose: Paper");
					current_index=1;
				}
				Hand::Scissors=>{
					println!("Player chose: Scissors");
					current_index=2;
				}
				Hand::Empty=>{
					println!("Something Broke");
					current_index=-1;
				}
			}
			match computer_hand{
				Hand::Rock=>{
					println!("Opponent chose: Rock");
					computer_index=0;
				}
				Hand::Paper=>{
					println!("Opponent chose: Paper");
					computer_index=1;
				}
				Hand::Scissors=>{
					println!("Opponent chose: Scissors");
					computer_index=2;
				}
				Hand::Empty=>{
					println!("Something Broke");
					computer_index=-1;
				}
			}
			
			if current_index==computer_index{
				//if there is a tie
				println!("It's a tie!");
			}else if (current_index==0&&computer_index==2)||(current_index==1&&computer_index==0)||(current_index==2&&computer_index==1){
				//if the player wins
				println!("You win!");
			}else if(current_index==0&&computer_index==1)||(current_index==1&&computer_index==2)||(current_index==2&&computer_index==0){
				//if the player loses
				println!("You lose!");
			}else{
				//something went horribly wrong!!!
				println!("Something went horribly wrong!!!!");
			}
		}
		//match s{
			//String::from("r") => println!("rock!"),
		//}
		//println!("s = {}", s);
	
	
	}//loop end
	
	/*let mut h=[[Stats{wins=0, losses=0, ties=0, rocks=0, papers=0, scissors=0}; 3]; 3];
	for j in 0..3 {
		for k in 0..3 {
			empty_stats(&mut h[j][k])
		}
	}
	
	
	*/
	//= History { past: [[Stats; 3]; 3]; };

    // 3x3 array of all Empty
    /*
	let mut b = [[Hand::Empty; 3]; 3];
	let mut rand_row;
	let mut rand_col;
    for j in 0..100 {
		rand_row = rand::thread_rng().gen_range(0, 3);
		rand_col = rand::thread_rng().gen_range(0, 3);
		if j%2==0{
			b[rand_row][rand_col] = Hand::X;
		}else{
			b[rand_row][rand_col] = Hand::O;
		}
		for l in 0..3 {
			for k in 0..3 {
				match b[l][k] {
					Hand::Empty => print!("."), 
					Hand::X => print!("X"), 
					Hand::O => print!("O"), 
				}
			}
			println!("");
		}
		println!("");
	}*/
    println!("pop");
	let a=hist[0][0].wins;
	println!("a = {}",a);
}