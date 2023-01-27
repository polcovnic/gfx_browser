use crate::dom::{AttrMap, ElementData, Node, NodeType};

use std::iter::Peekable;
use std::str::Chars;


pub struct HtmlParser<'a> {
    chars: Peekable<Chars<'a>>,
    node_q: Vec<String>,
}

impl<'a> HtmlParser<'a> {
    pub fn new(full_html: &str) -> HtmlParser {
        HtmlParser {
            chars: full_html.chars().peekable(),
            node_q: Vec::new(),
        }
    }

    pub fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();

        while self.chars.peek().is_some() {
            self.consume_while(char::is_whitespace);
            if self.chars.peek().map_or(false, |c| *c == '<') {
                self.chars.next();
                if self.chars.peek().map_or(false, |c| *c == '/') {
                    self.chars.next();
                    self.consume_while(char::is_whitespace);

                    let close_tag_name = self.consume_while(is_valid_tag_name);

                    self.consume_while(|x| x != '>');
                    self.chars.next();

                    self.node_q.push(close_tag_name);
                    break;
                } else if self.chars.peek().map_or(false, |c| *c == '!') {
                    self.chars.next();
                    nodes.push(self.parse_comment_node());
                } else {
                    let mut node = self.parse_node();
                    let insert_index = nodes.len();

                    if let NodeType::Element(ref e) = node.node_type {
                        if self.node_q.len() > 0 {
                            let assumed_tag = self.node_q.remove(0);

                            if e.tag_name != assumed_tag {
                                nodes.append(&mut node.children);
                                self.node_q.insert(0, assumed_tag);
                            }
                        }
                    }

                    nodes.insert(insert_index, node);
                }
            } else {
                nodes.push(self.parse_text_node());
            }
        }
        nodes
    }


    fn parse_node(&mut self) -> Node {
        let tagname = self.consume_while(is_valid_tag_name);
        let attributes = self.parse_attributes();

        let elem = ElementData::new(tagname, attributes);
        let children = self.parse_nodes();
        Node::new(NodeType::Element(elem), children)
    }

    fn parse_text_node(&mut self) -> Node {
        let mut text_content = String::new();

        while self.chars.peek().map_or(false, |c| *c != '<') {
            let whitespace = self.consume_while(char::is_whitespace);
            if whitespace.len() > 0 {
                text_content.push(' ');
            }
            let text_part = self.consume_while(|x| !x.is_whitespace() && x != '<');
            text_content.push_str(&text_part);
        }
        Node::new(NodeType::Text(text_content), Vec::new())
    }

    fn parse_comment_node(&mut self) -> Node {
        let mut comment_content = String::new();

        if self.chars.peek().map_or(false, |c| *c == '-') {
            self.chars.next();
            if self.chars.peek().map_or(false, |c| *c == '-') {
                self.chars.next();
            } else {
                self.consume_while(|c| c != '>');
                return Node::new(NodeType::Comment(comment_content), Vec::new());
            }
        } else {
            self.consume_while(|c| c != '>');
            return Node::new(NodeType::Comment(comment_content), Vec::new());
        }

        if self.chars.peek().map_or(false, |c| *c == '>') {
            self.chars.next();
            return Node::new(NodeType::Comment(comment_content), Vec::new());
        }

        if self.chars.peek().map_or(false, |c| *c == '-') {
            self.chars.next();
            if self.chars.peek().map_or(false, |c| *c == '>') {
                self.chars.next();
                return Node::new(NodeType::Comment(comment_content), Vec::new());
            } else {
                comment_content.push('-');
            }
        }

        while self.chars.peek().is_some() {
            comment_content.push_str(&self.consume_while(|c| c != '<' && c != '-'));
            if self.chars.peek().map_or(false, |c| *c == '<') {
                self.chars.next();
                if self.chars.peek().map_or(false, |c| *c == '!') {
                    self.chars.next();
                    if self.chars.peek().map_or(false, |c| *c == '-') {
                        self.chars.next();
                        if self.chars.peek().map_or(false, |c| *c == '-') {
                            self.consume_while(|c| c != '>');

                            return Node::new(NodeType::Comment(String::from("")), Vec::new());
                        } else {
                            comment_content.push_str("<!-");
                        }
                    } else if self.chars.peek().map_or(false, |c| *c == ' ') {
                        self.chars.next();
                        if self.chars.peek().map_or(false, |c| *c == '-') {
                            self.chars.next();
                            if self.chars.peek().map_or(false, |c| *c == '-') {
                                self.chars.next();
                                if self.chars.peek().map_or(false, |c| *c == '-') {
                                    self.chars.next();
                                    if self.chars.peek().map_or(false, |c| *c == '>') {
                                        self.chars.next();
                                        return Node::new(
                                            NodeType::Comment(String::from("")),
                                            Vec::new(),
                                        );
                                    } else {
                                        comment_content.push_str("<! --");
                                    }
                                } else {
                                    comment_content.push_str("<! -");
                                }
                            } else {
                                comment_content.push_str("<! ");
                            }
                        }
                    } else {
                        comment_content.push_str("<!");
                    }
                } else {
                    comment_content.push('<');
                }
            } else if self.chars.peek().map_or(false, |c| *c == '-') {
                self.chars.next();
                if self.chars.peek().map_or(false, |c| *c == '-') {
                    self.chars.next();
                    if self.chars.peek().map_or(false, |c| *c == '>') {
                        self.chars.next();
                        break;
                    } else {
                        comment_content.push_str("--");
                    }
                } else {
                    comment_content.push('-');
                }
            }
        }

        Node::new(NodeType::Comment(comment_content), Vec::new())
    }

    fn parse_attributes(&mut self) -> AttrMap {
        let mut attributes = AttrMap::new();

        while self.chars.peek().map_or(false, |c| *c != '>') {
            self.consume_while(char::is_whitespace);
            let name = self.consume_while(|c| is_valid_attr_name(c)).to_lowercase();
            self.consume_while(char::is_whitespace);

            let value = if self.chars.peek().map_or(false, |c| *c == '=') {
                self.chars.next();
                self.consume_while(char::is_whitespace);
                let s = self.parse_attr_value();
                self.consume_while(|c| !c.is_whitespace() && c != '>');
                self.consume_while(char::is_whitespace);
                s
            } else {
                "".to_string()
            };
            attributes.insert(name, value);
        }
        self.chars.next();

        attributes
    }

    fn parse_attr_value(&mut self) -> String {
        self.consume_while(char::is_whitespace);

        let result = match self.chars.peek() {
            Some(&c) if c == '"' || c == '\'' => {
                self.chars.next();
                let ret = self.consume_while(|x| x != c);
                self.chars.next();
                ret
            }
            _ => self.consume_while(is_valid_attr_value),
        };

        result
    }


    fn consume_while<F>(&mut self, condition: F) -> String
        where
            F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while self.chars.peek().map_or(false, |c| condition(*c)) {
            result.push(self.chars.next().unwrap());
        }

        result
    }
}

fn is_valid_tag_name(ch: char) -> bool {
    ch.is_digit(36)
}

fn is_valid_attr_name(c: char) -> bool {
    !is_excluded_name(c) && !is_control(c)
}

fn is_control(ch: char) -> bool {
    match ch {
        '\u{007F}' => true,
        c if c >= '\u{0000}' && c <= '\u{001F}' => true,
        c if c >= '\u{0080}' && c <= '\u{009F}' => true,
        _ => false,
    }
}

fn is_excluded_name(c: char) -> bool {
    match c {
        ' ' | '"' | '\'' | '>' | '/' | '=' => true,
        _ => false,
    }
}

fn is_valid_attr_value(c: char) -> bool {
    match c {
        ' ' | '"' | '\'' | '=' | '<' | '>' | '`' => false,
        _ => true,
    }
}


#[test]
fn test_parse_node() {
    let mut parser = HtmlParser::new("div class=\"testc\" id=\"testi\">");
    let node = parser.parse_node();
    let element_data = NodeType::Element(ElementData {
        tag_name: String::from("div"),
        attributes: {
            let mut m = AttrMap::new();
            m.insert(String::from("class"), String::from("testc"));
            m.insert(String::from("id"), String::from("testi"));
            m
        },
    });
    assert_eq!(node.node_type, element_data);
    if let NodeType::Element(data) = node.node_type {
        assert_eq!(data.tag_name, "div");
        assert_eq!(data.attributes.get("class").unwrap(), "testc");
        assert_eq!(data.attributes.get("id").unwrap(), "testi");
    }
}

#[test]
fn test_parse_nodes() {
    let mut parser = HtmlParser::new("<div class=\"testc\" id=\"testi\">");
    let nodes = parser.parse_nodes();
    assert_eq!(nodes.len(), 1);
    if let NodeType::Element(data) = &nodes[0].node_type {
        assert_eq!(data.tag_name, "div");
        assert_eq!(data.attributes.get("class").unwrap(), "testc");
        assert_eq!(data.attributes.get("id").unwrap(), "testi");
    }
}

#[test]
fn test_parse_text_node() {
    let mut parser = HtmlParser::new("test");
    let node = parser.parse_text_node();
    assert_eq!(node.node_type, NodeType::Text(String::from("test")));
    let mut parser = HtmlParser::new("test<");
    let node = parser.parse_text_node();
    assert_eq!(node.node_type, NodeType::Text(String::from("test")));
}

#[test]
fn test_parse_comment_node() {
    let mut parser = HtmlParser::new("--test-->");
    let node = parser.parse_comment_node();
    assert_eq!(node.node_type, NodeType::Comment(String::from("test")));
}

#[test]
fn test_parse_attributes() {
    let mut parser = HtmlParser::new("class=\"testc\" id=\"testi\">");
    let attributes = parser.parse_attributes();
    assert_eq!(attributes.get("class").unwrap(), "testc");
    assert_eq!(attributes.get("id").unwrap(), "testi");
}

#[test]
fn test_parse_attr_value() {
    let mut parser = HtmlParser::new("\"testc\" id=\"testi\">");
    let value = parser.parse_attr_value();
    assert_eq!(value, "testc");
}