pub mod css;
pub mod dom;
pub mod html;
pub mod parser;
pub mod style;

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

fn style_tree_maker(n: &style::StyledNode) -> Tree<String> {
    let result = n
        .children
        .iter()
        .fold(Tree::new(n.to_string()), |mut root, entry| {
            match entry.node.node_type {
                dom::NodeType::Element(_) => {
                    root.push(style_tree_maker(entry));
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
        "<html>
    <head>
      <title>Test</title>
    </head>
    <div class=\"outer\">
      <p class=\"inner\">Hello, <span id=\"name\">world!</span></p>
      <p><!-- comment --></p>
      <p class=\"inner\" id=\"bye\">Goodbye!</p>
    </div> 
  </html>
  ",
    );

    let root_node = html::parse(s);

    println!("{}", tree_maker(&root_node));

    let q = String::from(
        "span {
        display: flex;
    }
    
    .inner {
        color: red;
        width: 20px;
    }
    ",
    );

    let stylesheet = css::parse(q);

    println!("{} ", stylesheet);

    let style_root = style::style_tree(&root_node, &stylesheet);

    println!("{}", style_tree_maker(&style_root));
}
