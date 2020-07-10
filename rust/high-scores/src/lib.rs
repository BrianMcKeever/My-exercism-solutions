use std::cmp;
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        if self.scores.len() < 4 {
            let mut result: Vec<u32> = self.scores.to_vec();
            result.sort_unstable_by_key(|x| cmp::Reverse(*x));
            return result;
        }
        let mut first = u32::MIN;
        let mut second = u32::MIN;
        let mut third = u32::MIN;
        for score in self.scores.iter() {
            if *score > first {
                third = second;
                second = first;
                first = *score;
            } else if *score > second {
                third = second;
                second = *score;
            } else if *score > third {
                third = *score;
            }
        }
        vec![first, second, third]
    }
}
