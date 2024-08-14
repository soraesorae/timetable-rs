use std::collections::{BTreeSet, HashSet};

enum StackData {
    History(usize, BTreeSet<usize>),
    Restore(HashSet<usize>),
}

// TODO: use Rc<RefCell<_>>
// TODO: index number <-> class time struct
pub fn generate_table(v: Vec<HashSet<usize>>, n: usize, output: &mut Vec<BTreeSet<usize>>) {
    let mut cache: HashSet<BTreeSet<usize>> = HashSet::new();

    for (idx, set) in v.iter().enumerate() {
        let mut valid_set: HashSet<usize> = set.clone();
        let mut stack: Vec<StackData> = Vec::new();
        stack.push(StackData::History(idx, BTreeSet::from([idx])));
        while let Some(class) = stack.pop() {
            match class {
                StackData::History(id, history) => {
                    if history.len() == n {
                        output.push(history);
                        continue;
                    }
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

    dbg!(output);
}
