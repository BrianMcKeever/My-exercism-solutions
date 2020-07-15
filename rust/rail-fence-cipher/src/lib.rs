pub struct RailFence {
    num_rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence {
            num_rails: rails as usize,
        }
    }

    fn to_rails(&self, text: &str) -> Vec<Vec<char>> {
        let rails = vec![vec!['.'; self.num_rails as usize]; text.len()];
        let mut go_down = true;
        let mut i = 0;
        rails
            .iter()
            .zip(text.chars())
            .map(|(rail, letter)| {
                let mut r = rail.clone();
                r[i] = letter;
                if go_down {
                    if i == self.num_rails - 1 {
                        go_down = false;
                        i -= 1;
                    } else {
                        i += 1;
                    }
                } else {
                    if i == 0 {
                        go_down = true;
                        i += 1;
                    } else {
                        i -= 1;
                    }
                }
                r
            })
            .collect()
    }

    pub fn encode(&self, text: &str) -> String {
        let encoded_rails = self.to_rails(text);

        let mut result = String::new();
        for i in 0..self.num_rails {
            for rail in &encoded_rails {
                if rail[i] != '.' {
                    result.push(rail[i]);
                }
            }
        }
        result
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut encoded_rails = self.to_rails(
            "?".chars()
                .cycle()
                .take(cipher.len())
                .collect::<String>()
                .as_str(),
        );

        let mut letters = cipher.chars();
        for j in 0..self.num_rails {
            for rail in encoded_rails.iter_mut() {
                if rail[j] == '?' {
                    rail[j] = letters.next().unwrap();
                }
            }
        }

        encoded_rails
            .iter()
            .flat_map(|c| c.iter().copied().filter(|l| *l != '.').collect::<Vec<_>>())
            .collect()
    }
}
