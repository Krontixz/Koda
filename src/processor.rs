use crate::{KodaNode, KodaType};

pub struct KodaTree {
    pub root: Vec<KodaNode>,
}

impl KodaTree {
    pub fn build(lines: Vec<KodaNode>) -> Self {
        let mut root = Vec::new();
        let mut stack: Vec<(usize, *mut KodaNode)> = Vec::new();

        let mut nodes = lines;

        for mut node in nodes {
            if let KodaType::Empty = node.k_type {
                continue;
            }

            while let Some(&(indent, _)) = stack.last() {
                if node.indent <= indent {
                    stack.pop();
                } else {
                    break;
                }
            }

            if stack.is_empty() {
                root.push(node);
                let last_idx = root.len() - 1;
                stack.push((0, &mut root[last_idx] as *mut KodaNode));
            } else {
                let (_, parent_ptr) = stack.last().unwrap();
                unsafe {
                    let parent = &mut **parent_ptr;
                    parent.children.push(node);
                    let last_idx = parent.children.len() - 1;
                    let new_node = &mut parent.children[last_idx];
                    stack.push((new_node.indent, new_node as *mut KodaNode));
                }
            }
        }
        KodaTree { root }
    }
}
