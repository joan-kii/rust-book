use std::collections::HashMap;

fn main() {
    let team_1: String = String::from("Blue");
    let score: String = String::from("50");

    let mut teams: HashMap<String, String> = HashMap::new();
    teams.insert(team_1, score);
}
