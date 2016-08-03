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
struct MinStats {
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
fn new_stats()->Stats {
	//stat.games =0;
    //stat.wins = 0;
    //stat.losses = 0;
    //stat.ties = 0;
    //stat.rocks = 0;
    //stat.papers = 0;
    //stat.scissors = 0;
	let stat: Stats = Stats { games: 0, wins: 0 , losses: 0 , ties: 0 , rocks: 0 , papers: 0 , scissors: 0 };
	return stat;
}

fn main() {
	let mut hist: [[MinStats; 3]; 3] = [
		[
			MinStats {
				rocks: 0,
				papers: 0,
				scissors: 0,
			},
			MinStats {
				rocks: 0,
				papers: 0,
				scissors: 0,
			},
			MinStats {
				rocks: 0,
				papers: 0,
				scissors: 0,
			},
		],
		[
			MinStats {
				rocks: 0,
				papers: 0,
				scissors: 0,
			},
			MinStats {
				rocks: 0,
				papers: 0,
				scissors: 0,
			},
			MinStats {
				rocks: 0,
				papers: 0,
				scissors: 0,
			},
		],
		[
			MinStats {
				rocks: 0,
				papers: 0,
				scissors: 0,
			},
			MinStats {
				rocks: 0,
				papers: 0,
				scissors: 0,
			},
			MinStats {
				rocks: 0,
				papers: 0,
				scissors: 0,
			},
		],
	];
	//let mut game_stats: Stats = empty_stats(game_stats);
	let mut game_stats=new_stats();
	let mut past_rocks=0;
	let mut past_papers=0;
	let mut past_scissors=01;
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
			game_stats.games += 1;
			let mut first_index=0;
			let mut second_index=0;
			let mut current_index;
			let mut computer_index;
			let mut need_history=true;
			match last_hand{
				Hand::Rock=>second_index=0,
				Hand::Paper=>second_index=1,
				Hand::Scissors=>second_index=2,
				Hand::Empty=>need_history=false,
			}
			match two_hands_ago{
				Hand::Rock=>first_index=0,
				Hand::Paper=>first_index=1,
				Hand::Scissors=>first_index=2,
				Hand::Empty=>need_history=false,
			}
			two_hands_ago=last_hand;
			last_hand=current_hand;
			computer_hand = Hand::Empty;
			if need_history{
				//need to get and set history
				//first need to check history to see past actions
				//let mut rand_hand;
				past_rocks=hist[first_index][second_index].rocks;
				past_papers=hist[first_index][second_index].papers;
				past_scissors=hist[first_index][second_index].scissors;
				if past_rocks==past_papers && past_rocks==past_scissors{
					//do fully random
					let mut rand_hand;
					rand_hand = rand::thread_rng().gen_range(0, 3);
					if rand_hand==0{
						computer_hand = Hand::Rock;
					}else if rand_hand==1{
						computer_hand = Hand::Paper;
					}else if rand_hand==2{
						computer_hand = Hand::Scissors;
					}
				}else if past_rocks>past_papers&&past_rocks>past_scissors{
					//the player mostly chose rock following the last two hands
					//try to give it paper cuts!
					computer_hand = Hand::Paper;
				}else if past_papers>past_rocks&&past_papers>past_scissors{
					//the player mostly chose paper following the last two hands
					//try to slice it up!
					computer_hand = Hand::Scissors;
				}else if past_scissors>past_rocks&&past_scissors>past_papers{
					//the player mostly chose scissors following the last two hands
					//try to smash it with a rock!
					computer_hand = Hand::Rock;
				}else if past_rocks>past_papers&&past_rocks==past_scissors{
					//rocks and scissors are the majority
					//choose rock, because it would either win or tie
					computer_hand = Hand::Rock;
				}else if past_rocks>past_scissors&&past_rocks==past_papers{
					//rocks and papers are the majority
					//choose paper, because it would either win or tie
					computer_hand = Hand::Paper;
				}else if past_scissors>past_rocks&&past_scissors==past_papers{
					//scissors and papers are the majority
					//choose scissors, because it would either win or tie
					computer_hand = Hand::Scissors;
				}else{
					println!("Something is very wrong with the AI...(run)");
					computer_hand = Hand::Empty;
				}
								
				//set history,
				match current_hand{
					Hand::Rock=>hist[first_index][second_index].rocks+=1,
					Hand::Paper=>hist[first_index][second_index].papers+=1,
					Hand::Scissors=>hist[first_index][second_index].scissors+=1,
					Hand::Empty=>println!("Run, run, as fast as you can, you can't escape from the homocidal AI"),
				}
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
					game_stats.rocks+=1;
				}
				Hand::Paper=>{
					println!("Player chose: Paper");
					current_index=1;
					game_stats.papers+=1;
				}
				Hand::Scissors=>{
					println!("Player chose: Scissors");
					current_index=2;
					game_stats.scissors+=1;
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
				game_stats.ties += 1;
			}else if (current_index==0&&computer_index==2)||(current_index==1&&computer_index==0)||(current_index==2&&computer_index==1){
				//if the player wins
				println!("You win!");
				game_stats.wins += 1;
			}else if(current_index==0&&computer_index==1)||(current_index==1&&computer_index==2)||(current_index==2&&computer_index==0){
				//if the player loses
				println!("You lose!");
				game_stats.losses += 1;
			}else{
				//something went horribly wrong!!!
				println!("Something went horribly wrong!!!!");
			}
		}
	
	
	}//loop end
	
	println!("Player Stats:");
	if game_stats.games==0{
		//println!("Wins: 0 (0.00%)\nWins: 0 (0.00%)");
		println!("Wins: 0 (0.00%)");
		println!("Ties: 0 (0.00%)");
		println!("Losses: 0 (0.00%)");
		println!("Rocks: 0");
		println!("Papers: 0");
		println!("Scissors: 0");
	}else{
		//not dividing by zero!!
		let win_percent: f32 = 100.0*((game_stats.wins as f32)/ (game_stats.games as f32));
		let loss_percent: f32 = 100.0*((game_stats.losses as f32)/ (game_stats.games as f32));
		let tie_percent: f32 = 100.0*((game_stats.ties as f32)/ (game_stats.games as f32));
		println!("Wins: {} ({:.2}%)",game_stats.wins,win_percent);
		println!("Ties: {} ({:.2}%)",game_stats.ties,tie_percent);
		println!("Losses: {} ({:.2}%)",game_stats.losses,loss_percent);
		println!("Rocks: {}",game_stats.rocks);
		println!("Papers: {}",game_stats.papers);
		println!("Scissors: {}",game_stats.scissors);
	}
}