use std::cmp::Reverse;
use std::{cmp::Ordering, collections::BinaryHeap};

/// A struct to hold a value and its associated score.
#[derive(Debug, Clone)]
pub struct ScoredValue<T> {
    pub value: T,
    pub score: f32,
}

impl<T> ScoredValue<T> {
    /// Creates a new `ScoredValue` with the given value and score.
    pub fn new(value: T, score: f32) -> ScoredValue<T> {
        ScoredValue { value, score }
    }
}

impl<T> PartialOrd for ScoredValue<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl<T> Ord for ScoredValue<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("Scores should be comparable")
    }
}

impl<T> PartialEq for ScoredValue<T> {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl<T> Eq for ScoredValue<T> {}

/// Returns the top `n` elements with the highest scores from the given vector.
pub fn top_n_elements<T>(v: Vec<ScoredValue<T>>, n: usize) -> Vec<ScoredValue<T>> {
    if v.len() == 0 {
        return v;
    }

    let mut min_heap = BinaryHeap::with_capacity(n);

    for scored_value in v {
        if min_heap.len() < n {
            min_heap.push(Reverse(scored_value));
        } else if let Some(Reverse(min_entry)) = min_heap.peek() {
            if scored_value.score > min_entry.score {
                min_heap.pop();
                min_heap.push(Reverse(scored_value));
            }
        }
    }

    let mut entries: Vec<ScoredValue<T>> = min_heap
        .into_iter()
        .map(|Reverse(scored_value)| scored_value)
        .collect::<Vec<_>>();

    entries.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scored_value_new() {
        let _ = ScoredValue::new((String::from("abc"), 0.1), 0.1);
    }

    #[test]
    fn test_cmp() {
        let a = ScoredValue::new(String::from("abc"), 0.1);
        let b = ScoredValue::new(String::from("def"), 0.2);

        assert_ne!(a, b);
        assert!(a > b);
    }

    #[test]
    fn test_top_n_elements() {
        let values = vec![
            ScoredValue::new("c", 0.7),
            ScoredValue::new("b", 0.5),
            ScoredValue::new("a", 1.0),
        ];
        let sorted = top_n_elements(values, 2);

        assert_eq!(sorted.len(), 2);
        assert_eq!("a", sorted[0].value);
        assert_eq!("c", sorted[1].value);
    }
}
