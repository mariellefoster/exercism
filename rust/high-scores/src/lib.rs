#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores}
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
        let mut score_cp = self.scores.to_vec();

        score_cp.sort();
        score_cp.into_iter().rev().take(3).collect()
    }
}
