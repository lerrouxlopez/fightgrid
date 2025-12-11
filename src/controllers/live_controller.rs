#[derive(Clone, Copy, Debug)]
pub struct LiveBlock {
    pub title: &'static str,
    pub status: &'static str,
}

pub struct LiveController {
    blocks: Vec<LiveBlock>,
    pub callouts: &'static str,
}

impl LiveController {
    pub fn new() -> Self {
        Self {
            blocks: vec![
                LiveBlock {
                    title: "Cage Feed",
                    status: "Online • 1080p60",
                },
                LiveBlock {
                    title: "Commentary",
                    status: "Mic check needed",
                },
                LiveBlock {
                    title: "Clock/Scoring",
                    status: "Synced",
                },
            ],
            callouts: "• Keep rounds on 5-minute cadence\n• Push lower-thirds before each walkout\n• Confirm judges table feed",
        }
    }

    pub fn blocks(&self) -> &[LiveBlock] {
        &self.blocks
    }
}
