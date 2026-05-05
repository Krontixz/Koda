mod processor;
mod resolver;
mod executor;

use std::env;
use std::fs;
use std::process::Command;
use std::path::PathBuf;
use crate::processor::KodaTree;
use crate::resolver::Resolver;
use crate::executor::Executor;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Koda Language System\nUsage: koda install | koda build <file>");
        return;
    }

    match args.as_str() {
        "install" => {
            let exe_path = match env::current_exe() {
                Ok(p) => p,
                Err(_) => { println!("Error: Could not find current executable."); return; }
            };

            let home = env::var("HOME").or_else(|_| env::var("USERPROFILE"))
                .unwrap_or_else(|_| ".".to_string());
            
            let mut koda_bin = PathBuf::from(home);
            koda_bin.push(".koda");
            koda_bin.push("bin");
            
            if let Err(e) = fs::create_dir_all(&koda_bin) {
                println!("Error: Could not create directory {:?}: {}", koda_bin, e);
                return;
            }
            
            let bin_name = if cfg!(windows) { "koda.exe" } else { "koda" };
            let mut dest = koda_bin.clone();
            dest.push(bin_name);

            if let Err(e) = fs::copy(&exe_path, &dest) {
                println!("Error: Failed to copy binary: {}", e);
                return;
            }

            #[cfg(windows)]
            {
                let path_val = koda_bin.to_str().unwrap_or("");
                if !path_val.is_empty() {
                    let script = format!(
                        "[Environment]::SetEnvironmentVariable('Path', [Environment]::GetEnvironmentVariable('Path', 'User') + ';{}', 'User')",
                        path_val
                    );
                    let _ = Command::new("powershell").args(["-Command", &script]).output();
                }
            }

            println!("Koda installed successfully to {:?}", dest);
        }
        "build" => {
            if args.len() < 3 { 
                println!("Error: Provide a filename.");
                return; 
            }
            let file_path = &args;
            let content = match fs::read_to_string(file_path) {
                Ok(c) => c,
                Err(_) => { println!("Error: Could not read file {}", file_path); return; }
            };

            let mut tokens = Vec::new();
            for line in content.lines() {
                tokens.push(koda::tokenize(line));
            }

            let mut tree = KodaTree::build(tokens);
            let mut res = Resolver::new();
            res.collect_variables(&tree.root);

            for node in &mut tree.root {
                res.resolve(node);
                Executor::run(node);
            }
            println!("Build successful: {}", file_path);
        }
        _ => println!("Unknown command."),
    }
}
