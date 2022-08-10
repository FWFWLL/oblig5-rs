use std::collections::HashMap;
use std::fs::File;
use serde::Deserialize;

const SUB_LENGTH: usize = 3;
const DATA_PATH: &str = "data/test_data";

#[derive(Deserialize, Debug)]
struct Repertoire {
	filename: String,
	infected: String,
}

fn main() {
	// Vectors containing all immune repertoires
	let mut infected_immune_repertoires: Vec<HashMap<String, i32>> = Vec::new();
	let mut healthy_immune_repertoires: Vec<HashMap<String, i32>> = Vec::new();

	// HashMaps containing all subsequences and in how many repertoires they occur in
	let mut infected_subsequence_counts: HashMap<String, i32> = HashMap::new();
	let mut healthy_subsequence_counts: HashMap<String, i32> = HashMap::new();

	// Just to make it look nice
	let address_infected = &format!("{:p}", &infected_immune_repertoires)[8..];
	let address_healthy = &format!("{:p}", &healthy_immune_repertoires)[8..];

	// Open metadata.csv
	let meta_path = format!("{DATA_PATH}/metadata.csv");
	let meta_file = File::open(meta_path)
		.expect("Failed to open file...");

	// Read records in metadata.csv
	let mut meta_reader = csv::Reader::from_reader(meta_file);
	for result in meta_reader.records() {
		let record = result
			.expect("Expected a valid CSV record...");

		let repertoire: Repertoire = record
			.deserialize(None)
			.expect("Could not deserialize CSV record...");

		let rep_path = format!("{DATA_PATH}/{}", repertoire.filename);
		let rep_file = File::open(rep_path)
			.expect("Failed to open file...");

		let mut temp: HashMap<String, i32> = HashMap::new();

		println!("Reading file {}...", repertoire.filename);

		let mut rep_reader = csv::Reader::from_reader(rep_file);
		for result in rep_reader.records() {
			let record = result
				.expect("Expected a valid CSV record...");

			let sequence: String = record
				.deserialize(None)
				.expect("Could not deserialize CSV record...");


			for i in 0..=sequence.len() - SUB_LENGTH {
				let subsequence = String::from(&sequence[i..i + SUB_LENGTH]);
				temp.insert(subsequence, 1);
			}
		}

		print!("Inserted into HashMap@");
		if repertoire.infected.contains("True") {
			infected_immune_repertoires.push(temp);
			println!("{address_infected}");
		} else {
			healthy_immune_repertoires.push(temp);
			println!("{address_healthy}");
		}
	}

	println!("Done reading files...");

	println!("Merging HashMaps...");

	for hashmap in &infected_immune_repertoires {
		for subsequence in hashmap.keys() {
			*infected_subsequence_counts.entry(subsequence.to_string()).or_insert(0) += 1;
		}
	}

	for hashmap in &healthy_immune_repertoires {
		for subsequence in hashmap.keys() {
			*healthy_subsequence_counts.entry(subsequence.to_string()).or_insert(0) += 1;
		}
	}

	println!("Finished Merging...");

	println!("Displaying results...");

	println!("┏{0:━<13}┳{:━<10}┳{0:━<9}┳{0:━<12}┓", "");
	println!("┃ {:^11} ┃ {:^8} ┃ {:^7} ┃ {:^10} ┃", "Subsequence", "Infected", "Healthy", "Difference");
	println!("┣{0:━<13}╋{:━<10}╋{0:━<9}╋{0:━<12}┫", "");
	for (subsequence, occurences) in infected_subsequence_counts {
		let healthy_occurences: i32 = *healthy_subsequence_counts.get(&subsequence).unwrap_or(&0);
		if occurences - healthy_occurences >= 5 {
			println!("┃ {subsequence:<11} ┃ {occurences:>8} ┃ {healthy_occurences:>7} ┃ {:>10} ┃", occurences - healthy_occurences);
		}
	}
	println!("┗{0:━<13}┻{:━<10}┻{0:━<9}┻{0:━<12}┛", "");
}
