use std::collections::{HashMap};
use crate::css;
use crate::dom;

use dom::{Node, NodeType, ElementData};
use css::{Stylesheet, Rule, Selector, SimpleSelector, Value, Specificity};


type PropertyMap = HashMap<String, Value>;

#[derive(Debug)]
pub struct StyledNode<'a> {
  pub node: &'a Node,
  pub specified_values: PropertyMap,
  pub children: Vec<StyledNode<'a>>
}

fn matches(elem: &ElementData, selector: &Selector) -> bool {
  match *selector {
      Selector::Simple(ref simple_selector) => matches_simple_selector(elem, simple_selector)
  }
}

type MatchedRule<'a> = (Specificity, &'a Rule);

fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
  rule.selectors.iter()
    .find(|selector| matches(elem, *selector))
    .map(|selector| (selector.specificity(), rule))
}

fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
  stylesheet.rules.iter().filter_map(|rule| match_rule(elem, rule)).collect()
}


pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
  StyledNode {
      node: root,
      specified_values: match root.node_type {
          NodeType::Element(ref elem) => specified_values(elem, stylesheet),
          NodeType::Text(_) => HashMap::new(),
          NodeType::Comment(_) => HashMap::new()
      },
      children: root.children.iter().map(|child| style_tree(child, stylesheet)).collect(),
  }
}

fn specified_values(elem: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
  let mut values = HashMap::new();
  let mut rules = matching_rules(elem, stylesheet);

  // Go through the rules from lowest to highest specificity.
  rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
  for (_, rule) in rules {
      for declaration in &rule.declarations {
          values.insert(declaration.name.clone(), declaration.value.clone());
      }
  }
  values
}

fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
  if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
    return false;
  }

  if selector.id.iter().any(|id| elem.id() != Some(id)) {
    return false;
  }

  let elem_classes = elem.classes();
  if selector.class.iter().any(|class| !elem_classes.contains(&**class)) {
    return false;
  }

  true
}