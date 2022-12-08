use ::std::fs;
use std::{collections::HashMap, vec};

#[derive(Debug)]
struct Tree {
    visible: bool,
    size: i8,
    trees_to_left: Vec<String>,
    trees_to_right: Vec<String>,
    trees_to_bottom: Vec<String>,
    trees_to_top: Vec<String>,
}

fn main() {
    println!("--- Day 8: Treetop Tree House ---");
    let input: String = fs::read_to_string("./src/day8/input.txt").expect("File should exist");

    let mut trees: HashMap<String, Tree> = HashMap::new();
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut columns: Vec<Vec<String>> = Vec::new();

    input.lines().enumerate().for_each(|(row_i, line)| {
        line.chars().enumerate().for_each(|(column_i, character)| {
            let id: String = row_i.to_string() + "-" + &column_i.to_string();
            let size: i8 = character
                .to_string()
                .parse::<i8>()
                .expect("Should be a number");

            if let Some(row) = rows.get_mut(row_i) {
                row.push(id.clone());
            } else {
                rows.push(vec![id.clone()]);
            }

            if let Some(column) = columns.get_mut(column_i) {
                column.push(id.clone());
            } else {
                columns.push(vec![id.clone()]);
            }

            trees.entry(id).or_insert(Tree {
                visible: false,
                size,
                trees_to_bottom: Vec::new(),
                trees_to_top: Vec::new(),
                trees_to_left: Vec::new(),
                trees_to_right: Vec::new(),
            });
        });
    });

    rows.iter().for_each(|row| {
        // Left to right
        set_visibility(&rows, &columns, &mut trees, row.clone());

        // Right to right
        let rev_row = &mut row.clone();
        rev_row.reverse();
        set_visibility(&rows, &columns, &mut trees, rev_row.clone());
    });

    columns.iter().for_each(|column| {
        // Top to bottom
        set_visibility(&rows, &columns, &mut trees, column.clone());

        // Bottom to top
        let mut_column = &mut column.clone();
        mut_column.reverse();
        set_visibility(&rows, &columns, &mut trees, mut_column.clone());
    });

    // Part One
    let mut visible_trees: usize = 0;
    let _ = &trees.iter_mut().for_each(|(_tree_id, tree)| {
        if tree.visible {
            visible_trees += 1;
        }
    });

    // Part Two
    let mut highest_scenic_score: i128 = 0;
    for (_tree_id, tree) in &trees {
        let scenic_score_right = calc_scenic_score(&trees, &tree, &tree.trees_to_right);
        let scenic_score_left = calc_scenic_score(&trees, &tree, &tree.trees_to_left);
        let scenic_score_top = calc_scenic_score(&trees, &tree, &tree.trees_to_top);
        let scenic_score_bottom = calc_scenic_score(&trees, &tree, &tree.trees_to_bottom);

        let scenic_score: i128 =
            (scenic_score_right * scenic_score_left * scenic_score_top * scenic_score_bottom)
                .into();

        if scenic_score > highest_scenic_score {
            highest_scenic_score = scenic_score;
        }
    }

    println!("Part One: {}", visible_trees);
    println!("Part Two: {}", highest_scenic_score);
}

fn set_visibility(
    rows: &Vec<Vec<String>>,
    columns: &Vec<Vec<String>>,
    trees: &mut HashMap<String, Tree>,
    sequence: Vec<String>,
) {
    let mut largest_size: i8 = 0;

    sequence.iter().enumerate().for_each(|(i, tree_id)| {
        let tree = trees.get_mut(tree_id).expect("Tree should exist");

        if i == 0 {
            // This is an edge tree, it's always visible
            largest_size = tree.size;
            tree.visible = true;
        } else {
            // Tree is visible if its size is bigger than the current
            // biggest size in the sequence
            if tree.size > largest_size {
                largest_size = tree.size;
                tree.visible = true;
            }

            let split: Vec<&str> = tree_id.split("-").collect();
            let row_i: usize = split[0].parse::<usize>().expect("Should be a number");
            let column_i: usize = split[1].parse::<usize>().expect("Should be a number");

            let tree_row = &rows[row_i];
            let tree_column = &columns[column_i];

            if tree.trees_to_left.is_empty() {
                let trees_left = &tree_row[..column_i];
                trees_left.iter().rev().for_each(|tree_id| {
                    tree.trees_to_left.push(tree_id.clone());
                });
            }

            if tree.trees_to_right.is_empty() {
                let trees_right = &tree_row[column_i + 1..];
                trees_right.iter().for_each(|tree_id| {
                    tree.trees_to_right.push(tree_id.clone());
                });
            }

            if tree.trees_to_bottom.is_empty() {
                let trees_bottom = &tree_column[row_i + 1..];
                trees_bottom.iter().for_each(|tree_id| {
                    tree.trees_to_bottom.push(tree_id.clone());
                });
            }

            if tree.trees_to_top.is_empty() {
                let trees_top = &tree_column[..row_i];
                trees_top.iter().rev().for_each(|tree_id| {
                    tree.trees_to_top.push(tree_id.clone());
                });
            }
        }
    });
}

fn calc_scenic_score(trees: &HashMap<String, Tree>, tree: &Tree, sequence: &Vec<String>) -> i32 {
    let mut scenic_score: i32 = 0;
    for tree_id in sequence {
        let next_tree_in_sequence = trees.get(&tree_id.clone()).expect("Tree should exist");
        scenic_score += 1;
        if next_tree_in_sequence.size >= tree.size {
            break;
        }
    }
    scenic_score
}
