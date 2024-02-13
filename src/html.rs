use crate::dom;
use std::collections::HashMap;

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        return result;
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn parse_tagname(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false,
        })
    }

    fn parse_node(&mut self) -> dom::Node {
        if self.starts_with("<!--") {
            self.parse_comment()
        } else if self.starts_with("<") {
            self.parse_element()
        } else {
            self.parse_text()
        }
    }

    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|c| c != '<'))
    }

    fn parse_comment(&mut self) -> dom::Node {
        println!("DID I VENE COME HERE???");
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '!');
        assert!(self.consume_char() == '-');
        assert!(self.consume_char() == '-');

        let mut comment = String::new();

        loop {
            if self.starts_with("-->") {
                break;
            }
            comment.push(self.consume_char());
        }

        assert!(self.consume_char() == '-');
        assert!(self.consume_char() == '-');
        assert!(self.consume_char() == '>');

        dom::comment(comment)
    }

    fn parse_element(&mut self) -> dom::Node {
        // Opening tag.
        assert!(self.consume_char() == '<');
        let tag_name = self.parse_tagname();
        let attrs = self.parse_attributes();
        assert!(self.consume_char() == '>');

        // Contents.
        let children = self.parse_nodes();

        // Closing tag.
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tagname() == tag_name);
        assert!(self.consume_char() == '>');

        return dom::elem(tag_name, attrs, children);
    }

    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tagname();
        assert!(self.consume_char() == '=');
        let value = self.parse_attr_value();
        return (name, value);
    }
    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert!(self.consume_char() == open_quote);
        return value;
    }

    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        return attributes;
    }

    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        return nodes;
    }
}

pub fn parse(source: String) -> dom::Node {
    let mut nodes = Parser {
        pos: 0,
        input: source,
    }
    .parse_nodes();

    // If the document contains a root element, just return it. Otherwise, create one.
    if nodes.len() == 1 {
        nodes.swap_remove(0)
    } else {
        dom::elem("html".to_string(), HashMap::new(), nodes)
    }
}
