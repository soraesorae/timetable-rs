use std::collections::{BTreeSet, HashSet};

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
