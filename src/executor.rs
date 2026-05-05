use std::process::Command;
use crate::{KodaNode, KodaType};

pub struct Executor;

impl Executor {
    pub fn run(node: &mut KodaNode) {
        if let KodaType::Execution = node.k_type {
            let output = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .args(["/C", &node.content])
                    .output()
            } else {
                Command::new("sh")
                    .args(["-c", &node.content])
                    .output()
            };

            if let Ok(out) = output {
                let result = String::from_utf8_lossy(&out.stdout).trim().to_string();
                node.content = result;
                node.k_type = KodaType::Data;
            } else {
                node.content = String::from("EXECUTION_ERROR");
            }
        }

        for child in &mut node.children {
            Self::run(child);
        }
    }
}
