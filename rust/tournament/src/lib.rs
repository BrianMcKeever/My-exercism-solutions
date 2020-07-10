use std::collections::HashMap;

#[derive(Debug)]
struct TeamRecord {
    wins: u32,
    losses: u32,
    draws: u32,
}

fn update_team_record(
    records: &mut HashMap<String, TeamRecord>,
    team: &str,
    wins: u32,
    losses: u32,
    draws: u32,
) {
    let name = team.to_string();
    match records.get_mut(&name) {
        None => {
            records.insert(
                name,
                TeamRecord {
                    wins,
                    losses,
                    draws,
                },
            );
        }
        Some(record) => {
            record.wins += wins;
            record.losses += losses;
            record.draws += draws;
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let records: &mut HashMap<String, TeamRecord> = &mut HashMap::new();
    for line in match_results.lines() {
        let mut split = line.split(";");
        let team1 = split.next().unwrap();
        let team2 = split.next().unwrap();
        let result = split.next().unwrap();
        match result {
            "win" => {
                update_team_record(records, team1, 1, 0, 0);
                update_team_record(records, team2, 0, 1, 0);
            }
            "loss" => {
                update_team_record(records, team1, 0, 1, 0);
                update_team_record(records, team2, 1, 0, 0);
            }
            "draw" => {
                update_team_record(records, team1, 0, 0, 1);
                update_team_record(records, team2, 0, 0, 1);
            }
            _ => panic!("Invalid argument"),
        }
    }
    let mut stats: Vec<(&String, u32, u32, u32, u32, u32)> = records.iter().map(|(team, record)| {
        let mp = record.wins + record.losses + record.draws;
        let points = record.wins * 3 + record.draws;
        (team, mp, record.wins, record.draws, record.losses, points)
    }).collect();
    stats.sort_unstable_by_key(|(b, _, _, _, _, a)| (-*a, *b));
    let table = stats.iter().map(|(team, mp, wins, draws, losses, points)| {
        format!(
            "\n{:30} |{:3} |{:3} |{:3} |{:3} |{:3}",
            team, mp, wins, draws, losses, points
        )
    }).collect::<String>();
    let mut output = "Team                           | MP |  W |  D |  L |  P".to_string();
    output += &table;
    output
}
