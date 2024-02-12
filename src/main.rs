pub mod dom;
pub mod html;

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
    let s = String::from(
        "qq<html>
    <head>
      <title>Test</title>
    </head>
    <div class=\"outer\">
      <p class=\"inner\">Hello, <span id=\"name\">world!</span></p>
      <p></p>
      <p class=\"inner\" id=\"bye\">Goodbye!</p>
    </div>
  </html>
  ",
    );

    let node = html::parse(s);

    println!("{}", tree_maker(&node));
}
