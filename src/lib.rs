use std::collections::{BTreeSet, HashSet};

enum StackData {
    History(usize, BTreeSet<usize>),
    Restore(HashSet<usize>),
}

pub fn generate_table(v: Vec<HashSet<usize>>) {
    let mut cache: HashSet<BTreeSet<usize>> = HashSet::new();

    for (idx, set) in v.iter().enumerate() {
        let mut valid_set: HashSet<usize> = set.clone();
        let mut stack: Vec<StackData> = Vec::new();
        stack.push(StackData::History(idx, BTreeSet::from([idx])));
        while let Some(class) = stack.pop() {
            match class {
                StackData::History(id, history) => {
                    valid_set = valid_set
                        .intersection(v.get(id).unwrap())
                        .copied()
                        .collect();
                    stack.push(StackData::Restore(valid_set.clone()));
                    for &x in &valid_set {
                        let mut new_history = history.clone();
                        new_history.insert(x);
                        let dup = new_history.clone();
                        if !cache.contains(&new_history) {
                            cache.insert(new_history);
                            stack.push(StackData::History(x, dup));
                        }
                    }
                }
                StackData::Restore(bck) => {
                    valid_set = bck;
                }
            };
        }
    }
}

pub fn get_basic_data_sets(basic_data: Vec<Vec<u64>>) -> Option<HashSet<BTreeSet<u64>>> {
    let valid_sets: Vec<BTreeSet<u64>> = basic_data.into_iter().map(BTreeSet::from_iter).collect();
    let valid_sets: HashSet<BTreeSet<u64>> = HashSet::from_iter(valid_sets);
    if !valid_sets.is_empty() {
        Some(valid_sets)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeSet, HashSet};

    use super::*;

    fn generate_basic_data() -> Vec<Vec<u64>> {
        [vec![2, 3, 4], vec![3], vec![0], vec![1, 4], vec![3]].to_vec()
    }

    #[test]
    fn test_generate_basic_data() {
        let output = get_basic_data_sets(generate_basic_data());

        let expected = HashSet::from([
            BTreeSet::from([2, 3, 4]),
            BTreeSet::from([3]),
            BTreeSet::from([0]),
            BTreeSet::from([1, 4]),
            BTreeSet::from([3]),
        ]);

        assert_eq!(output, Some(expected));
    }
}
