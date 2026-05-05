use crate::{KodaNode, KodaType};
use std::fs;
use std::collections::HashMap;

pub struct Resolver {
    pub variables: HashMap<String, String>,
}

impl Resolver {
    pub fn new() -> Self {
        Self { variables: HashMap::new() }
    }

    pub fn collect_variables(&mut self, nodes: &[KodaNode]) {
        for node in nodes {
            if let KodaType::Variable = node.k_type {
                self.variables.insert(node.label.clone(), node.content.clone());
            }
            self.collect_variables(&node.children);
        }
    }

    pub fn resolve(&self, node: &mut KodaNode) {
        if node.content.contains('@') {
            for (key, val) in &self.variables {
                let placeholder = format!("@{}", key);
                if node.content.contains(&placeholder) {
                    node.content = node.content.replace(&placeholder, val);
                }
            }
        }

        if let KodaType::Pointer = node.k_type {
            if let Ok(content) = fs::read_to_string(&node.content) {
                node.content = content;
                node.k_type = KodaType::Data;
            }
        }

        for child in &mut node.children {
            self.resolve(child);
        }
    }
}
