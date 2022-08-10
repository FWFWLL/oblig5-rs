use serde::Deserialize;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::sync::Mutex;
use std::thread;

const SUB_LENGTH: usize = 3;
const DATA_PATH: &str = "data/real_data";

#[derive(Deserialize, Debug)]
struct Repertoire {
	filename: String,
	infected: String,
}

fn main() {
	// Vectors containing all immune repertoires
	let infected_immune_repertoires: Mutex<Vec<HashMap<String, i32>>> = Mutex::new(Vec::new());
	let healthy_immune_repertoires: Mutex<Vec<HashMap<String, i32>>> = Mutex::new(Vec::new());

	// HashMaps containing all subsequences and in how many repertoires they occur in
	let mut infected_subsequence_counts: HashMap<String, i32> = HashMap::new();
	let mut healthy_subsequence_counts: HashMap<String, i32> = HashMap::new();

	// Get repertoires from metadata.csv
	let repertoires: Vec<Repertoire> = csv::Reader::from_reader(File::open(format!("{DATA_PATH}/metadata.csv")).unwrap()).records().map(|rec| {
		rec.unwrap().deserialize(None).unwrap()
	}).collect();

	// Using rayon, iterate over every repertoire and store the results in our Vector of HashMaps
	let _: Vec<()> = repertoires.par_iter().map(|rep| {
		println!("{:>2?} - Receiving file {}", thread::current().id(), rep.filename);

		let sequences: Vec<String> = csv::Reader::from_reader(File::open(format!("{DATA_PATH}/{}", rep.filename)).unwrap()).records().map(|rec| {
			rec.unwrap().deserialize::<String>(None).unwrap()
		}).collect();

		let temp: Mutex<HashMap<String, i32>> = Mutex::new(HashMap::new());

		for sequence in sequences {
			for i in 0..=sequence.len() - SUB_LENGTH {
				temp.lock().unwrap().insert(String::from(&sequence[i..i + SUB_LENGTH]), 1);
			}
		}

		// Check if the repertoire the data came from was from an infected person
		// Place in appropriate corresponding container
		if rep.infected.contains("True") {
			println!("{:>2?} - Inserting into HashMap@{:p}", thread::current().id(), &infected_immune_repertoires);
			infected_immune_repertoires.lock().unwrap().push(temp.into_inner().unwrap());
		} else {
			println!("{:>2?} - Inserting into HashMap@{:p}", thread::current().id(), &healthy_immune_repertoires);
			healthy_immune_repertoires.lock().unwrap().push(temp.into_inner().unwrap());
		}
	}).collect();

	for hashmap in infected_immune_repertoires.into_inner().unwrap().iter() {
		for subsequence in hashmap.keys() {
			*infected_subsequence_counts.entry(subsequence.to_string()).or_insert(0) += 1;
		}
	}

	for hashmap in healthy_immune_repertoires.into_inner().unwrap().iter() {
		for subsequence in hashmap.keys() {
			*healthy_subsequence_counts.entry(subsequence.to_string()).or_insert(0) += 1;
		}
	}

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
