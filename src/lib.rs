use std::collections::{BTreeSet, HashSet};

pub mod calc;
pub mod lecture;

pub fn get_basic_data_sets(basic_data: Vec<Vec<usize>>) -> Option<HashSet<BTreeSet<usize>>> {
    let valid_sets: Vec<BTreeSet<usize>> =
        basic_data.into_iter().map(BTreeSet::from_iter).collect();
    let valid_sets: HashSet<BTreeSet<usize>> = HashSet::from_iter(valid_sets);
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

    fn generate_basic_data() -> Vec<Vec<usize>> {
        [vec![2, 3, 4], vec![3], vec![0], vec![1, 4], vec![3]].to_vec()
    }

    fn generate_basic_data_2() -> Vec<Vec<usize>> {
        // [vec![1, 2, 3, 4], vec![0, 3, 4], vec![0, 3, 4, 5], vec![0, 1, 2, 5], vec![0, 1, 2, 5], vec![2, 3, 4]].to_vec()
        [
            vec![1, 2, 3, 4, 6],
            vec![0, 3, 4, 6],
            vec![0, 3, 4, 5, 6],
            vec![0, 1, 2, 5, 6],
            vec![0, 1, 2, 5, 6],
            vec![2, 3, 4, 6],
            vec![0, 1, 2, 3, 4, 5],
        ]
        .to_vec()
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

    #[test]
    fn test_generate_table_with_basic_data() {
        let data = generate_basic_data_2();
        let input: Vec<HashSet<usize>> = data.into_iter().map(HashSet::from_iter).collect();
        let mut output: Vec<BTreeSet<usize>> = Vec::new();
        calc::generate_table(input, 4, &mut output);
    }
}
