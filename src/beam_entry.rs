pub type ProbabilityT = f32;

#[derive(Debug, Default)]
pub struct BeamEntry {
    pub pr_total: ProbabilityT,
    pub pr_non_blank: ProbabilityT,
    pub pr_blank: ProbabilityT,
}

impl BeamEntry {
    pub fn new(pr_non_blank: ProbabilityT, pr_blank: ProbabilityT) -> BeamEntry {
        BeamEntry {
            pr_total: pr_non_blank + pr_blank,
            pr_non_blank,
            pr_blank,
        }
    }

    pub fn update_probabilities(&mut self, pr_non_blank: ProbabilityT, pr_blank: ProbabilityT) {
        self.pr_non_blank += pr_non_blank;
        self.pr_blank += pr_blank;
        self.pr_total += pr_blank + pr_non_blank;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beam_entry_default() {
        let entry = BeamEntry::default();
        assert_eq!(entry.pr_total, 0.0);
        assert_eq!(entry.pr_non_blank, 0.0);
        assert_eq!(entry.pr_blank, 0.0);
    }

    #[test]
    fn test_beam_entry_new() {
        let pr_non_blank = 0.3;
        let pr_blank = 0.7;
        let entry = BeamEntry::new(pr_non_blank, pr_blank);
        assert_eq!(entry.pr_total, pr_non_blank + pr_blank);
        assert_eq!(entry.pr_non_blank, pr_non_blank);
        assert_eq!(entry.pr_blank, pr_blank);
    }

    #[test]
    fn test_update_probabilities() {
        let mut entry = BeamEntry::new(0.2, 0.3);
        entry.update_probabilities(0.1, 0.1);
        assert_eq!(entry.pr_total, 0.7);
        assert_eq!(entry.pr_non_blank, 0.3);
        assert_eq!(entry.pr_blank, 0.4);
    }
}
