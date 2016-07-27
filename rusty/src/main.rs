extern crate rand;
use rand::Rng;

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
   wins: u32,
   losses: u32,
   ties: u32,
   rocks: u32,
   papers: u32,
   scissors: u32
}

fn empty_stats(mut stat: &mut Stats) {
   stat.wins = 0;
   stat.losses = 0;
   stat.ties = 0;
   stat.rocks = 0;
   stat.papers = 0;
   stat.scissors = 0;
}
static mut hist: [[Stats; 3]; 3] = [
	[
		Stats {
			wins: 0,
			losses: 0,
			ties: 0,
			rocks: 0,
			papers: 0,
			scissors: 0,
		},
		Stats {
			wins: 0,
			losses: 0,
			ties: 0,
			rocks: 0,
			papers: 0,
			scissors: 0,
		},
		Stats {
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
			wins: 0,
			losses: 0,
			ties: 0,
			rocks: 0,
			papers: 0,
			scissors: 0,
		},
		Stats {
			wins: 0,
			losses: 0,
			ties: 0,
			rocks: 0,
			papers: 0,
			scissors: 0,
		},
		Stats {
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
			wins: 0,
			losses: 0,
			ties: 0,
			rocks: 0,
			papers: 0,
			scissors: 0,
		},
		Stats {
			wins: 0,
			losses: 0,
			ties: 0,
			rocks: 0,
			papers: 0,
			scissors: 0,
		},
		Stats {
			wins: 0,
			losses: 0,
			ties: 0,
			rocks: 0,
			papers: 0,
			scissors: 0,
		},
	],
];
fn main() {

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
}