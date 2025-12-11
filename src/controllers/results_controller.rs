pub struct ResultInfo {
    pub bout: &'static str,
    pub outcome: &'static str,
}

pub struct ResultsController {
    results: Vec<ResultInfo>,
}

impl ResultsController {
    pub fn new() -> Self {
        Self {
            results: vec![
                ResultInfo {
                    bout: "Santos vs. Cruz",
                    outcome: "TKO • Round 2",
                },
                ResultInfo {
                    bout: "Navarro vs. Dela Rosa",
                    outcome: "Split Decision",
                },
                ResultInfo {
                    bout: "Valdez vs. Luna",
                    outcome: "Unanimous Decision",
                },
                ResultInfo {
                    bout: "Manalo vs. Abad",
                    outcome: "Submission • Round 3",
                },
            ],
        }
    }

    pub fn results(&self) -> &[ResultInfo] {
        &self.results
    }
}
