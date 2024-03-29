use crate::{constants::*, preprocessor::preprocessor};

pub type NodeList = Vec<Node>;

#[derive(Debug, Clone)]
pub struct Node {
    name: String,
    comments: Vec<String>,
    keys: Vec<KeyValuePair>,
    children: NodeList,
}

impl Node {
    pub fn new(name: String) -> Self {
        Self {
            name,
            comments: Vec::new(),
            keys: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, new_child: Node) {
        match self
            .children
            .iter_mut()
            .find(|p| *p.name == new_child.name.to_owned())
        {
            Some(_) => {}
            None => {
                self.children.push(new_child);
            }
        }
    }

    pub fn add_kv_pair(&mut self, kv_pair: KeyValuePair) {
        match self
            .keys
            .iter_mut()
            .find(|p| *p.key == kv_pair.key.to_owned())
        {
            Some(pair) => {
                pair.value = kv_pair.value.to_owned();
            }
            None => {
                self.keys.push(kv_pair);
            }
        }
    }

    /// Convert node and it's children to string
    pub fn convert_to_string(&self) -> String {
        unimplemented!();

        let result = String::new();

        // TODO: Push root node comments
        // TODO: Push root node keys and there comments
        // TODO: Do the same recursively for root node children

        result
    }
}

#[derive(Debug, Clone)]
pub struct KeyValuePair {
    key: String,
    value: String,
    comments: Vec<String>,
}

pub fn parse(file: Option<String>, input: String) -> Node {
    // Note: Find a way to add comments to root node
    let mut root_node = Node::new(ROOT_SECTION.into());
    let mut sections: Vec<String> = Vec::new();
    let mut comments: Vec<String> = Vec::new();

    let processed_input = preprocessor(file, input);

    for (index, mut line) in processed_input.lines().enumerate() {
        let line_number = index + 1;
        line = line.trim();

        if line.is_empty() {
            continue;
        }

        if line.starts_with(COMMENT_PREFIX) {
            comments.push(remove_first_chars(
                &mut line.to_string(),
                COMMENT_PREFIX.len(),
            ));
            continue;
        }

        if line.starts_with(SECTION_PREFIX) {
            let s: Vec<&str> = line.split_whitespace().collect();

            let current_section = {
                if s[0] == SECTION_PREFIX {
                    s[1].to_string()
                } else {
                    remove_first_chars(&mut s[0].to_string(), SECTION_PREFIX.len())
                }
            };

            sections.push(current_section);

            assert!(
                sections.is_empty() == false,
                "line {}: No section name is provided",
                line_number
            );
        }

        if line.starts_with(SECTION_END) || line.ends_with(SECTION_END) {
            sections.pop();
        }

        if line.contains(KEY_VALUE_SEP) {
            let (mut key, mut value) = line
                .split_once(KEY_VALUE_SEP)
                .expect("Corrupt config file!");

            key = key.trim();
            value = value.trim();

            if sections.is_empty() {
                let kv_pair = KeyValuePair {
                    key: key.into(),
                    value: value.into(),
                    comments: comments.clone(),
                };

                root_node.add_kv_pair(kv_pair);
            } else {
                let parent: &mut Node = &mut root_node;
                let kv_pair = KeyValuePair {
                    key: key.into(),
                    value: value.into(),
                    comments: comments.clone(),
                };

                create_nodes(kv_pair, &mut sections.clone(), parent);
            }
        }

        comments.clear();
    }

    println!("{:#?}", root_node);
    return root_node;
}

fn create_nodes(kv_pair: KeyValuePair, sections: &mut Vec<String>, parent: &mut Node) {
    let section_name = sections.get(0);

    if section_name.is_some() {
        let node = Node::new(section_name.unwrap().to_owned());

        parent.add_child(node);

        sections.remove(0);

        let tnode_index = parent.children.len() - 1;

        create_nodes(kv_pair, sections, &mut parent.children[tnode_index]);
    } else {
        // add kv pair to last node
        parent.add_kv_pair(kv_pair)
    }
}

fn remove_first_chars(s: &mut String, n: usize) -> String {
    for _ in 0..n {
        s.remove(0);
    }

    s.trim_start().to_string()
}
