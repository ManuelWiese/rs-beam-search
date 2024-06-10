use std::collections::HashMap;

use crate::beam_entry::{BeamEntry, ProbabilityT};
use crate::sorting::{top_n_elements, ScoredValue};

pub struct BeamState {
    pub entries: HashMap<String, BeamEntry>,
    pub pruning: bool,
    pub pruning_threshold: ProbabilityT,
}

impl BeamState {
    pub fn new(pruning: bool, pruning_threshold: ProbabilityT) -> BeamState {
        BeamState {
            entries: HashMap::new(),
            pruning: pruning,
            pruning_threshold: pruning_threshold,
        }
    }

    pub fn get_probabilities(&self, labeling: &str) -> Option<&BeamEntry> {
        self.entries.get(labeling)
    }

    pub fn update(&mut self, labeling: String, pr_non_blank: ProbabilityT, pr_blank: ProbabilityT) {
        let entry = self.entries.entry(labeling).or_insert(BeamEntry::default());
        entry.update_probabilities(pr_non_blank, pr_blank);
    }

    pub fn sort(&mut self) -> Vec<(String, ProbabilityT)> {
        if self.pruning {
            self.prune();
        }

        let mut entries: Vec<(String, ProbabilityT)> = self
            .entries
            .iter()
            .map(|(key, entry)| (key.clone(), entry.pr_total))
            .collect();

        // Sort the entries by the second entry (pr_total) in descending order
        entries.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        entries
    }

    pub fn sort_top_n(&mut self, n: usize) -> Vec<(String, ProbabilityT)> {
        if self.pruning {
            self.prune();
        }

        let mut entries = self
            .entries
            .iter()
            .map(|(key, beam_entry)| {
                ScoredValue::new((key.clone(), beam_entry), beam_entry.pr_total)
            })
            .collect::<Vec<_>>();

        entries = top_n_elements(entries, n);

        let results: Vec<(String, ProbabilityT)> = entries
            .iter()
            .map(|scored_value| (scored_value.value.0.clone(), scored_value.score))
            .collect::<Vec<_>>();

        results
    }

    pub fn prune(&mut self) {
        self.entries
            .retain(|_, beam_entry| beam_entry.pr_total > self.pruning_threshold);
    }
}

impl Default for BeamState {
    fn default() -> Self {
        BeamState::new(true, 1e-5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beam_state_default() {
        let beam_state = BeamState::default();
        assert_eq!(beam_state.entries.len(), 0)
    }

    #[test]
    fn test_beam_state_update_and_get() {
        let mut beam_state = BeamState::default();
        let key = String::from("test");

        beam_state.update(String::from("test"), 0.1, 0.1);

        assert_eq!(beam_state.entries.len(), 1);
        assert_eq!(beam_state.get_probabilities(&key).unwrap().pr_total, 0.2);
    }

    #[test]
    fn test_beam_state_prune() {
        let mut beam_state = BeamState::new(true, 0.1);

        beam_state.update(String::from("a"), 0.01, 0.08);
        beam_state.update(String::from("b"), 0.05, 0.04);
        beam_state.update(String::from("c"), 0.1, 0.2);

        assert_eq!(beam_state.entries.len(), 3);

        beam_state.prune();

        assert_eq!(beam_state.entries.len(), 1);
    }

    #[test]
    fn test_beam_state_sort() {
        let mut beam_state = BeamState::default();

        beam_state.update(String::from("a"), 0.1, 0.0);
        beam_state.update(String::from("b"), 0.3, 0.0);
        beam_state.update(String::from("c"), 0.2, 0.0);

        assert_eq!(beam_state.entries.len(), 3);

        let entries = beam_state.sort();
        assert_eq!(beam_state.entries.len(), entries.len());

        assert_eq!(entries[0].0, "b");
        assert_eq!(entries[1].0, "c");
        assert_eq!(entries[2].0, "a");
    }

    #[test]
    fn test_beam_state_sort_top_n() {
        let mut beam_state = BeamState::default();

        beam_state.update(String::from("a"), 0.1, 0.0);
        beam_state.update(String::from("b"), 0.3, 0.0);
        beam_state.update(String::from("c"), 0.2, 0.0);
        beam_state.update(String::from("d"), 0.0, 0.05);

        assert_eq!(beam_state.entries.len(), 4);

        let n = 3;
        let entries = beam_state.sort_top_n(n);
        assert_eq!(n, entries.len());

        assert_eq!(entries[0].0, "b");
        assert_eq!(entries[1].0, "c");
        assert_eq!(entries[2].0, "a");

        println!("{:?}", entries);
    }
}
