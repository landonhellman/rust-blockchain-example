extern crate chrono;
extern crate csv;
extern crate serde;

use csv::{ReaderBuilder, Writer};
use serde::Deserialize;
use std::collections::VecDeque;
use std::error::Error;
use chrono::Local;
use std::fs::File;

#[derive(Debug, Deserialize, Clone)]
struct Person {
    id: String,
    name: String,
    preference_1: f64,
    preference_2: f64,
    preference_3: f64,
   
    Pronouns: String,                  // 1 for he/him, 2 for she/her, 3 for anything else
    #[serde(default)]PronounsID: f64,
   
    Residential_College: String,    // 
   
    Difficulty: String,                // 1, 2, 3, and 4 for difficulty 
    #[serde(default)]DifficultyID: f64,
   
    Days: String,
    #[serde(default)]DaysID: f64,                 // 1 for no day hikes, 2 for day hikes, 1.1 for indifferent
   
    Arts: String,
    #[serde(default)]ArtsID: f64,                 // same as day hikes
   
    Gender: String, 
    #[serde(default)]GenderID: f64,
    
    Food: String,
    #[serde(default)]FoodID: f64,         // basically true or false
    
    Location: String, 
    #[serde(default)]LocationID: f64,                  // weights ASSIGNED based off of geographic location. use a rust
                                    // library to compute the distance
    
    School: String, 
    #[serde(default)]SchoolID: f64,                   // 1 for public and magnet, 2 for private
}

fn assignPronouns(footie: &mut Person) {
    let difficultyString = &footie.Difficulty;

    if difficultyString == "Easy: a mellow trip, though still some challenges!"{
        footie.DifficultyID = 1.0; 
    }
    else if difficultyString == "Moderate: a few ups and downs, some rough terrain" {
        footie.DifficultyID = 2.0;
    }
    else if difficultyString == "Strenuous: some ups and downs, some rough terrain" {
        footie.DifficultyID = 3.0
    }
    else {
        footie.DifficultyID = 4.0;
    }
}

fn assignDifficulty(footie: &mut Person) {
    let pronounString = &footie.Difficulty;

    if pronounString == "he/him" || pronounString == "he/they" {
        footie.PronounsID = 1.0; 
    }
    else if pronounString == "she/her" || pronounString == "she/they" {
        footie.PronounsID = 2.0;
    }
    else {
        footie.PronounsID = 3.0;
    }
}

fn assignDays(footie: &mut Person) {
    let dayString = &footie.Days;

    if dayString == "Yes, I am interested in day hikes only" {
        footie.DaysID = 1.0; 
    }
    else if dayString == "I am NOT interested in day hikes."  {
        footie.DaysID = 2.0;
    }
    else {
        footie.DaysID = 1.8;
    }
}

fn assignArts(footie: &mut Person) {
    let artString = &footie.Arts;

    if artString == "Yes, I am interested in the arts-focused trips only" {
        footie.ArtsID = 1.0; 
    }
    else if artString == "I am NOT interested in the arts-focused trips."  {
        footie.ArtsID = 2.0;
    }
    else {
        footie.ArtsID = 1.8;
    }
}

fn read_csv(file_path: &str) -> Result<Vec<Person>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;
    let mut people: Vec<Person> = Vec::new();
    
    for result in rdr.deserialize() {
        let mut record: Person = result?;
        assignPronouns(&mut record);
        people.push(record);
    }
    Ok(people)
}

fn calculate_similarity(p1: &Person, p2: &Person) -> f64 {
    let diff_1 = p1.preference_1 - p2.preference_1;
    let diff_2 = p1.preference_2 - p2.preference_2;
    let diff_3 = p1.preference_3 - p2.preference_3;
    let diff_4 = p1.PronounsID - p2.PronounsID;

    (diff_1.powi(2) + diff_2.powi(2) + diff_3.powi(2) + diff_4.powi(2)).sqrt()
}

fn group_people(people: Vec<Person>, group_size: usize) -> Vec<Vec<Person>> {
    let mut people = people;
    people.sort_by(|a, b| {
        let sim_a = a.preference_1.powi(2) + a.preference_2.powi(2) + a.preference_3.powi(2) + a.PronounsID.powi(2);
        let sim_b = b.preference_1.powi(2) + b.preference_2.powi(2) + b.preference_3.powi(2) + b.PronounsID.powi(2);
        sim_a.partial_cmp(&sim_b).unwrap()
    });

    let mut groups: Vec<Vec<Person>> = Vec::new();
    let mut group: VecDeque<Person> = VecDeque::new();

    for person in people {
        group.push_back(person);
        if group.len() == group_size {
            groups.push(group.iter().cloned().collect());
            group.clear();
        }
    }

    if !group.is_empty() {
        groups.push(group.iter().cloned().collect());
    }

    groups
}

fn write_groups_to_csv(groups: Vec<Vec<Person>>, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(file_path)?;

    let mut group_number = 1;
    for group in groups {
        for person in group {
            wtr.serialize((
                group_number,
                person.id,
                person.name,
                person.Pronouns,
                person.Residential_College,
                person.Difficulty,
                person.Days,
                person.Arts,
                person.Gender,
                person.Food,
                person.Location,
                person.School,
                person.preference_1,
                person.preference_2,
                person.preference_3,
            ))?;
        }
        group_number += 1;
    }

    wtr.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = "/Users/landonhellman/Documents/footie-grouper-rust/examples/example.csv"; // CSV input file with people
                                                                                                
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let output_file_name = format!("/Users/landonhellman/Documents/footie-grouper-rust/outputs/exampleOutput_{}.csv", timestamp);

    let output_file_false = File::create(output_file_name.clone())?;
    let output_file: &str = &output_file_name;

    let people = read_csv(input_file)?;

    let groups = group_people(people, 8);

    write_groups_to_csv(groups, output_file)?;

    println!("Grouping complete and saved to {}", output_file);

    Ok(())
}
