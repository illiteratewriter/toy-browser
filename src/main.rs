pub mod dom;

fn main() {
    // let node = dom::Node(vec![], dom::NodeType::Text(String::from("value")))

    let node = dom::Node {
      children: vec![],
      node_type: dom::NodeType::Text(String::from("value"))
    };

    println!("{:?}", node);
}