use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq)]
struct Team {
    name: String,
    win: usize,
    draw: usize,
    loss: usize,
}

impl Team {
    pub fn new(name: String) -> Self {
        Team {
            name,
            win: 0,
            draw: 0,
            loss: 0,
        }
    }

    pub fn win_match(&mut self) {
        self.win += 1;
    }

    pub fn draw_match(&mut self) {
        self.draw += 1;
    }

    pub fn loss_match(&mut self) {
        self.loss += 1;
    }

    pub fn scores(&self) -> usize {
        self.win * 3 + self.draw
    }

    pub fn matches(&self) -> usize {
        self.win + self.draw + self.loss
    }
}

impl PartialOrd for Team {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.scores().partial_cmp(&other.scores()).map(|o| {
            if o == Ordering::Equal {
                other.name.partial_cmp(&self.name).unwrap()
            } else {
                o
            }
        })
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:31}| {:>2} |  {} |  {} |  {} |  {}",
            self.name,
            self.matches(),
            self.win,
            self.draw,
            self.loss,
            self.scores()
        )
    }
}

pub fn tally(match_results: &str) -> String {
    let mut teams: HashMap<&str, Team> = HashMap::new();
    let matches: Vec<&str> = match_results
        .split('\n')
        .filter(|m| !m.is_empty())
        .collect();
    for a_match in matches {
        let infos: Vec<&str> = a_match.split(';').collect();
        let host = infos[0];
        let guest = infos[1];
        let result = infos[2];
        let host_team = teams.entry(host).or_insert(Team::new(host.to_string()));
        match result {
            "win" => {
                host_team.win_match();
            }
            "draw" => {
                host_team.draw_match();
            }
            "loss" => {
                host_team.loss_match();
            }
            _ => (),
        }
        // 两个 match 没有合在一起，原因是如果那么做，teams 就会同时 mut 引用两次，无法通过编译
        let guest_team = teams.entry(guest).or_insert(Team::new(guest.to_string()));
        match result {
            "win" => {
                guest_team.loss_match();
            }
            "draw" => {
                guest_team.draw_match();
            }
            "loss" => {
                guest_team.win_match();
            }
            _ => (),
        }
    }

    // 计算最长的 team name (并不需要)
    // let max_len = teams.keys().map(|k| k.len()).max();

    // sort
    let mut teams_vec: Vec<&Team> = teams.values().collect();
    teams_vec.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let mut result_table =
        vec!["Team                           | MP |  W |  D |  L |  P".to_string()];
    for t in teams_vec {
        result_table.push(t.to_string());
    }
    result_table.join("\n")
}
