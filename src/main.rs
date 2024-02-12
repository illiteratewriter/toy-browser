pub mod dom;

use std::collections::HashMap;
use termtree::Tree;

fn tree_maker(n: &dom::Node) -> Tree<String> {
    let result = n
        .children
        .iter()
        .fold(Tree::new(n.to_string()), |mut root, entry| {
            match entry.node_type {
                dom::NodeType::Element(_) => {
                    root.push(tree_maker(entry));
                }
                dom::NodeType::Comment(_) => {
                    root.push(Tree::new(entry.to_string()));
                }
                dom::NodeType::Text(_) => {
                    root.push(Tree::new(entry.to_string()));
                }
            }

            root
        });
    result
}

fn main() {
    let h1 = dom::text(String::from("h1"));
    let h2 = dom::text(String::from("h2"));
    let div = dom::elem(String::from("div"), HashMap::new(), vec![h2]);
    let node = dom::elem(String::from("div"), HashMap::new(), vec![h1, div]);

    println!("{}", tree_maker(&node));
}
