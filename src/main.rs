const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
use rand::RngExt;

fn main() {
	let mut highs : Vec<i32> = vec![72, 68, 75, 81, 79];
	
	for i in 0..highs.len() {
		println!("{}: {}", DAYS[i], highs[i])
		}
	
	for i in 1..=2 {
        let mut rng = rand::rng();
	let temp : i32 = rng.random_range(60..=100);
        highs.push(temp);
	}

//	for i in 0..DAYS.len() {
//		print!("{}: {}", DAYS[i], highs[i])
//		};


	fn average_temp(log: &Vec<i32>) -> f64 {
		let mut average : f64 = 0.0; 
		let mut total : i32 = 0;
		for i in 0..highs.len() {
			total = total + highs[i]
			}
		average = (total / highs.len()).into()
	}

	println!("The average is: {}", average_temp)
}
