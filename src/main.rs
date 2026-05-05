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
            let exe_path = env::current_exe().unwrap();
            let home = env::var("HOME").or_else(|_| env::var("USERPROFILE")).unwrap();
            let mut koda_bin = PathBuf::from(home);
            koda_bin.push(".koda");
            koda_bin.push("bin");
            
            fs::create_dir_all(&koda_bin).unwrap();
            
            let bin_name = if cfg!(windows) { "koda.exe" } else { "koda" };
            let mut dest = koda_bin.clone();
            dest.push(bin_name);

            fs::copy(&exe_path, &dest).unwrap();

            #[cfg(windows)]
            {
                let path_val = koda_bin.to_str().unwrap();
                let script = format!(
                    "[Environment]::SetEnvironmentVariable('Path', [Environment]::GetEnvironmentVariable('Path', 'User') + ';{}', 'User')",
                    path_val
                );
                Command::new("powershell").args(["-Command", &script]).output().unwrap();
            }

            println!("Koda installed to {:?}", dest);
        }
        "build" => {
            if args.len() < 3 { return; }
            let content = fs::read_to_string(&args).unwrap();
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
            println!("Build complete for {}", args);
        }
        _ => println!("Unknown command."),
    }
}
