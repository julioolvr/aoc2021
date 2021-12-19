use std::{env, ops::Add, str::FromStr};

use petgraph::{graph::NodeIndex, visit::EdgeRef, EdgeDirection::Incoming, Graph};

fn main() {
    let file = if env::args()
        .skip(1)
        .next()
        .map_or(false, |flag| flag == "--sample")
    {
        include_str!("../sample.txt")
    } else {
        include_str!("../input.txt")
    };

    let numbers: Vec<Number> = file.lines().map(|line| line.parse().unwrap()).collect();
    let total_sum: Number = numbers
        .into_iter()
        .reduce(|acc, number| acc + number)
        .unwrap();
    println!("Part 1: {}", total_sum.magnitude());
}

#[derive(Debug, PartialEq)]
enum Number {
    Number(usize),
    Pair(Box<Number>, Box<Number>),
}

impl Number {
    fn magnitude(&self) -> usize {
        match self {
            Number::Number(n) => *n,
            Number::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let new_number = Number::Pair(Box::new(self), Box::new(other));
        reduce(&new_number)
    }
}

impl FromStr for Number {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = vec![];

        for char in s.chars() {
            match char {
                ']' => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(Box::new(Number::Pair(left, right)));
                }
                // All numbers in the input are a single digit (otherwise the "split" rule would
                // apply)
                d if d.is_digit(10) => {
                    stack.push(Box::new(Number::Number(d.to_digit(10).unwrap() as usize)))
                }
                '[' | ',' => {}
                c => panic!("Unexpected char {}", c),
            }
        }

        Ok(*stack.pop().unwrap())
    }
}

type TreeNumber = Graph<TreeNode, TreeEdge>;

#[derive(Debug, PartialEq)]
enum TreeNode {
    Number(usize),
    Pair,
}

#[derive(PartialEq, Debug)]
enum TreeEdge {
    Left,
    Right,
}

fn number_to_tree(number: &Number) -> TreeNumber {
    let mut tree = Graph::<TreeNode, TreeEdge>::new();
    insert_node(&mut tree, number);
    tree
}

fn tree_to_number(tree: &TreeNumber) -> Number {
    let root = tree
        .externals(Incoming)
        .next()
        .expect("Did not find tree root");

    node_to_number(root, tree)
}

fn node_to_number(node: NodeIndex, tree: &TreeNumber) -> Number {
    match tree[node] {
        TreeNode::Number(n) => Number::Number(n),
        TreeNode::Pair => {
            let left_node = tree
                .edges(node)
                .find(|edge| *edge.weight() == TreeEdge::Left)
                .expect("Did not find left edge for pair node")
                .target();

            let right_node = tree
                .edges(node)
                .find(|edge| *edge.weight() == TreeEdge::Right)
                .expect("Did not find right edge for pair node")
                .target();

            Number::Pair(
                Box::new(node_to_number(left_node, tree)),
                Box::new(node_to_number(right_node, tree)),
            )
        }
    }
}

fn insert_node(tree: &mut TreeNumber, number: &Number) -> NodeIndex {
    match number {
        Number::Number(value) => tree.add_node(TreeNode::Number(*value)),
        Number::Pair(left, right) => {
            let parent = tree.add_node(TreeNode::Pair);
            let left = insert_node(tree, left);
            let right = insert_node(tree, right);
            tree.add_edge(parent, left, TreeEdge::Left);
            tree.add_edge(parent, right, TreeEdge::Right);
            parent
        }
    }
}

fn reduce(number: &Number) -> Number {
    let mut tree = number_to_tree(number);
    let mut changed = true;

    while changed {
        changed = false;

        if explode(&mut tree) {
            changed = true;
            continue;
        }

        if split(&mut tree) {
            changed = true;
            continue;
        }
    }

    tree_to_number(&tree)
}

fn explode(tree: &mut TreeNumber) -> bool {
    let mut exploded = false;
    let root = tree
        .externals(Incoming)
        .next()
        .expect("Did not find tree root");

    if let Some(to_explode) = find_pair_at_depth(root, &tree, 4) {
        exploded = true;

        let left_node = tree
            .edges(to_explode)
            .find(|edge| *edge.weight() == TreeEdge::Left)
            .expect("Did not find left edge for exploding node")
            .target();

        let right_node = tree
            .edges(to_explode)
            .find(|edge| *edge.weight() == TreeEdge::Right)
            .expect("Did not find right edge for exploding node")
            .target();

        if let Some(left_number) = find_first_on_side(to_explode, &tree, TreeEdge::Left) {
            let a = match tree[left_number] {
                TreeNode::Number(n) => n,
                _ => panic!("Tried to add non-number tree node"),
            };

            let b = match tree[left_node] {
                TreeNode::Number(n) => n,
                _ => panic!("Tried to add non-number tree node"),
            };

            tree[left_number] = TreeNode::Number(a + b);
        }

        if let Some(right_number) = find_first_on_side(to_explode, &tree, TreeEdge::Right) {
            let a = match tree[right_number] {
                TreeNode::Number(n) => n,
                _ => panic!("Tried to add non-number tree node"),
            };

            let b = match tree[right_node] {
                TreeNode::Number(n) => n,
                _ => panic!("Tried to add non-number tree node"),
            };

            tree[right_number] = TreeNode::Number(a + b);
        }

        tree[to_explode] = TreeNode::Number(0);
    }

    exploded
}

fn split(tree: &mut TreeNumber) -> bool {
    let mut split = false;
    let root = tree
        .externals(Incoming)
        .next()
        .expect("Did not find tree root");

    if let Some(to_split) = find_greater_than(root, &tree, 9) {
        split = true;
        let value = match tree[to_split] {
            TreeNode::Number(n) => n,
            _ => panic!("Trying to split a non-number node"),
        };
        tree[to_split] = TreeNode::Pair;
        let left = tree.add_node(TreeNode::Number(value / 2));
        let right = tree.add_node(TreeNode::Number((value + 1) / 2));
        tree.add_edge(to_split, left, TreeEdge::Left);
        tree.add_edge(to_split, right, TreeEdge::Right);
    }

    split
}

fn find_greater_than(from: NodeIndex, tree: &TreeNumber, greater_than: usize) -> Option<NodeIndex> {
    match tree[from] {
        TreeNode::Number(n) if n > greater_than => Some(from),
        TreeNode::Number(_) => None,
        TreeNode::Pair => {
            let left_edge = tree
                .edges(from)
                .find(|edge| *edge.weight() == TreeEdge::Left);
            let right_edge = tree
                .edges(from)
                .find(|edge| *edge.weight() == TreeEdge::Right);

            [left_edge, right_edge]
                .iter()
                .filter_map(|edge| edge.map(|edge| edge.target()))
                .find_map(|target| find_greater_than(target, tree, greater_than))
        }
    }
}

fn find_pair_at_depth(from: NodeIndex, tree: &TreeNumber, depth: usize) -> Option<NodeIndex> {
    if depth == 0 {
        return match tree[from] {
            TreeNode::Pair => Some(from),
            TreeNode::Number(_) => None,
        };
    }

    let left_edge = tree
        .edges(from)
        .find(|edge| *edge.weight() == TreeEdge::Left);
    let right_edge = tree
        .edges(from)
        .find(|edge| *edge.weight() == TreeEdge::Right);

    [left_edge, right_edge]
        .iter()
        .filter_map(|edge| edge.map(|edge| edge.target()))
        .find_map(|target| find_pair_at_depth(target, tree, depth - 1))
}

fn find_first_on_side(from: NodeIndex, tree: &TreeNumber, side: TreeEdge) -> Option<NodeIndex> {
    let mut opposite_edge = tree
        .edges_directed(from, Incoming)
        .find(|edge| *edge.weight() != side);

    let mut parent = tree.neighbors_directed(from, Incoming).next();

    while opposite_edge.is_none() && parent.is_some() {
        opposite_edge = tree
            .edges_directed(parent.unwrap(), Incoming)
            .find(|edge| *edge.weight() != side);
        parent = tree.neighbors_directed(parent.unwrap(), Incoming).next();
    }

    let parent = match parent {
        Some(node) => node,
        None => return None,
    };

    let first_edge = tree
        .edges(parent)
        .find(|edge| *edge.weight() == side)
        .unwrap();
    let mut result = first_edge.target();

    while tree[result] == TreeNode::Pair {
        let edge = tree
            .edges(result)
            .find(|edge| *edge.weight() != side)
            .unwrap();
        result = edge.target();
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! number {
        ( $number:literal ) => {
            Number::Number($number)
        };

        ( [$left:tt, $right:tt] ) => {
            Number::Pair(Box::new(number!($left)), Box::new(number!($right)))
        };
    }

    #[test]
    fn test_explode_reduction() {
        assert_eq!(
            reduce(&number![[[[[[9, 8], 1], 2], 3], 4]]),
            number![[[[[0, 9], 2], 3], 4]]
        );

        assert_eq!(
            reduce(&number![[7, [6, [5, [4, [3, 2]]]]]]),
            number![[7, [6, [5, [7, 0]]]]]
        );

        assert_eq!(
            reduce(&number![[[6, [5, [4, [3, 2]]]], 1]]),
            number![[[6, [5, [7, 0]]], 3]]
        );

        assert_eq!(
            reduce(&number![[[3, [2, [1, [7, 3]]]], [6, [5, [4, [3, 2]]]]]]),
            number![[[3, [2, [8, 0]]], [9, [5, [7, 0]]]]]
        );
    }

    #[test]
    fn test_split() {
        assert_eq!(reduce(&number![[10, 1]]), number![[[5, 5], 1]]);
    }

    #[test]
    fn test_full_reduction() {
        assert_eq!(
            reduce(&number![[[[[[4, 3], 4], 4], [7, [[8, 4], 9]]], [1, 1]]]),
            number![[[[[0, 7], 4], [[7, 8], [6, 0]]], [8, 1]]]
        );
    }

    #[test]
    fn test_addition() {
        assert_eq!(
            number![[1, 2]] + number![[[3, 4], 5]],
            number![[[1, 2], [[3, 4], 5]]]
        );

        assert_eq!(
            number![[[[[4, 3], 4], 4], [7, [[8, 4], 9]]]] + number![[1, 1]],
            number![[[[[0, 7], 4], [[7, 8], [6, 0]]], [8, 1]]]
        );
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(number![[[1, 2], [[3, 4], 5]]].magnitude(), 143);
        assert_eq!(
            number![[[[[0, 7], 4], [[7, 8], [6, 0]]], [8, 1]]].magnitude(),
            1384
        );
        assert_eq!(
            number![[[[[1, 1], [2, 2]], [3, 3]], [4, 4]]].magnitude(),
            445
        );
    }

    #[test]
    fn test_macro_simple_number() {
        assert_eq!(number![42], Number::Number(42))
    }

    #[test]
    fn test_macro_pair() {
        assert_eq!(
            number![[1, 2]],
            Number::Pair(Box::new(Number::Number(1)), Box::new(Number::Number(2)))
        )
    }

    #[test]
    fn test_macro_complex_number() {
        use Number::{Number as N, Pair as P};

        fn bn(n: usize) -> Box<Number> {
            Box::new(N(n))
        }

        fn bp(left: Box<Number>, right: Box<Number>) -> Box<Number> {
            Box::new(P(left, right))
        }

        assert_eq!(
            number![[[[[1, 2], [3, 4]], [[5, 6], [7, 8]]], 9]],
            *bp(
                bp(
                    bp(bp(bn(1), bn(2)), bp(bn(3), bn(4))),
                    bp(bp(bn(5), bn(6)), bp(bn(7), bn(8))),
                ),
                bn(9)
            )
        )
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            number![[[1, 2], [[3, 4], 5]]],
            "[[1,2],[[3,4],5]]".parse().unwrap()
        );

        assert_eq!(
            number![[[[0, [5, 8]], [[1, 7], [9, 6]]], [[4, [1, 2]], [[1, 4], 2]]]],
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]"
                .parse()
                .unwrap()
        );
    }
}
