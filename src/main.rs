const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
use rand::Rng;

fn main() {
	let mut highs : Vec<i32> = vec![72, 68, 75, 81, 79];
	let mut rng = rand::rng();
	
	for i in 0..DAYS.len() {
		println!("{}: {}", DAYS[i], highs[i])
		}
	
	for i in 1..=2 {
        let temp = rand::rng();
        highs.push(temp.try_into().unwrap());
        }

}
