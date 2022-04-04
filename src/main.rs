use rand::Rng;

fn main() {
	let mut rng = rand::thread_rng();
	let mood = rng.gen_range(0..5);

	let contents = match mood {
		0 => include_str!("../txt/go-for-it.txt"),
		1 => include_str!("../txt/no-way.txt"),
		2 => include_str!("../txt/too-late.txt"),
		3 => include_str!("../txt/unhelpful.txt"),
		4 => include_str!("../txt/you-sure.txt"),
		_ => panic!("Bad value generated")
	};

	let lines: Vec<&str> = contents
		.split('\n')
		.collect();
	let line = rng.gen_range(0..lines.len());
	println!("{}", lines.get(line).unwrap());
}
