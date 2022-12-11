use std::{num, vec};

pub fn process_part_1(input: &str) -> usize {
    let mut forest_map = Vec::new();

    for line in input.lines() {
        forest_map.extend(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>(),
        );
    }

    let num_cols = input.lines().next().unwrap().len();
    let num_rows = forest_map.len() / num_cols;

    // List of tree visibles on the row
    let mut visibles: Vec<bool> = vec![];

    for row in forest_map.chunks(num_cols) {
        visibles.extend(mark_visibles(row.to_vec()))
    }

    for j in 0..num_cols {
        let mut col = vec![];
        for i in 0..num_rows {
            col.push(forest_map[i * num_cols + j])
        }

        let visible_col = mark_visibles(col);

        for i in 0..num_rows {
            visibles[i * num_cols + j] = visibles[i * num_cols + j] || visible_col[i];
        }
    }

    visibles.iter().map(|x| *x as usize).sum()
}

// O(n)
fn mark_visibles(line: Vec<usize>) -> Vec<bool> {
    let mut visibles = vec![];

    let mut tree_count_before = vec![0; 10];
    let mut tree_count_after = vec![0; 10];

    // O(n)
    for tree in line.iter() {
        tree_count_after[*tree] += 1;
    }

    // O(n)
    for tree in line.iter() {
        tree_count_after[*tree] -= 1;
        visibles.push(
            // O(n)
            tree_count_before[*tree..].iter().all(|x| *x == 0)
            // O(n)
                || tree_count_after[*tree..].iter().all(|x| *x == 0),
        );

        tree_count_before[*tree] += 1;
    }

    visibles
}

pub fn process_part_2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        const INPUT: &str = "30373
25512
65332
33549
35390";

        assert_eq!(process_part_1(INPUT), 21);
    }

    #[test]
    fn part_2() {
        todo!()
    }
}
