#[derive(Debug)]
pub struct HighScores<'a> {
    score_list: &'a [u32],
}

impl HighScores<'a> {
    pub fn new(scores: &'a[u32]) -> 'a Self {
        HighScores { score_list<'a> : scores}
    }

    pub fn scores(&self) -> &[u32] {
        unimplemented!("Return all the scores as a slice")
    }

    pub fn latest(&self) -> Option<u32> {
        unimplemented!("Return the latest (last) score")
    }

    pub fn personal_best(&self) -> Option<u32> {
        unimplemented!("Return the highest score")
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        unimplemented!("Return 3 highest scores")
    }
}
