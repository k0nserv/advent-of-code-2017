use std::fmt;
use std::collections::{HashMap, HashSet};

struct Program {
    name: String,
    weight: u32,
    children: Vec<Box<Program>>,
}

impl Program {
    fn new(name: String, weight: u32, children: Vec<Box<Program>>) -> Self {
        Self {
            name: name,
            weight: weight,
            children: children,
        }
    }

    fn build(
        data: &(&str, u32, HashSet<&str>),
        treeData: &HashMap<String, (&str, u32, HashSet<&str>)>,
    ) -> Box<Program> {
        Box::new(Program::new(
            data.0.to_owned(),
            data.1,
            data.2
                .iter()
                .map(|name| Program::build(treeData.get(name.to_owned()).unwrap(), treeData))
                .collect(),
        ))
    }

    fn weight(&self, include_self: bool) -> u32 {
        let mut sum = 0;
        if include_self {
            sum += self.weight;
        }

        sum
            + self.children
                .iter()
                .fold(0, |acc, child| acc + child.weight(include_self))
    }

    fn weight_required(&self, target_weight: u32) -> u32 {
        let child_weights = self.children
            .iter()
            .map(|child| child.weight(true))
            .collect::<Vec<u32>>();
        let balanced = child_weights.iter().all(|x| *x == child_weights[0]);

        if balanced {
            let child_sum: u32 = child_weights.iter().sum();
            return ((target_weight as i32 - child_sum as i32)) as u32;
        }

        let mut counts = HashMap::<u32, u32>::new();

        for weight in child_weights.iter() {
            *counts.entry(*weight).or_insert(0) += 1;
        }

        let mut count_vec: Vec<_> = counts.iter().collect();
        count_vec.sort_by(|a, b| b.1.cmp(a.1));
        let index_of_unbalanced_child = child_weights
            .clone()
            .iter()
            .position(|&v| v == *count_vec.iter().last().unwrap().0)
            .unwrap();

        return self.children[index_of_unbalanced_child].weight_required(*count_vec[0].0);
    }

    fn write_tree(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
        write!(
            f,
            "{}{} ({} - {})\n",
            "  ".repeat(depth),
            self.name,
            self.weight,
            self.weight(true)
        );

        self.children.iter().for_each(|child| {
            child.write_tree(f, depth + 1);
        });

        Ok(())
    }
}

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_tree(f, 0)
    }
}

fn parse<'a>(input: &'a str) -> Box<Program> {
    let tree_map = input
        .trim()
        .lines()
        .map(|row| {
            let words = row.trim().split_whitespace().collect::<Vec<&str>>();
            let name = words[0];
            let weight = words[1]
                .trim_matches(|c: char| !c.is_numeric())
                .parse::<u32>()
                .expect("Expected numeric weight");
            let mut children = HashSet::new();

            if words.len() > 3 {
                children = words[3..]
                    .iter()
                    .map(|program| program.trim_matches(','))
                    .collect::<HashSet<&str>>();
            }


            (name.to_owned(), (name, weight, children))
        })
        .collect::<HashMap<_, _>>();
    let mut root = tree_map.keys().collect::<HashSet<_>>();
    let childs_of_other = tree_map
        .values()
        .flat_map(|&(_, _, ref children)| children.clone())
        .collect::<HashSet<&str>>();

    for node in tree_map.values() {
        if childs_of_other.contains(node.0) {
            root.remove(&node.0.to_owned());
        }
    }

    assert!(root.len() == 1, "There can only be one");

    let root_vec = root.iter().collect::<Vec<_>>();
    let root_data = tree_map.get(root_vec[0].to_owned()).unwrap();

    Program::build(root_data, &tree_map)
}

pub fn solve(input: &str) -> (String, u32) {
    let tree = parse(input);
    (tree.name.to_owned(), tree.weight_required(0))
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_cases() {
        let input = "
            pbga (66)
            xhth (57)
            ebii (61)
            havc (66)
            ktlj (57)
            fwft (72) -> ktlj, cntj, xhth
            qoyq (66)
            padx (45) -> pbga, havc, qoyq
            tknk (41) -> ugml, padx, fwft
            jptl (61)
            ugml (68) -> gyxo, ebii, jptl
            gyxo (61)
            cntj (57)
        ";
        assert_eq!(solve(input), ("tknk".to_owned(), 60));
    }
}
