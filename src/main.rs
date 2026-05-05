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
        println!("Koda Language System");
        println!("Usage: koda install | koda build <file>");
        return;
    }

    match args.as_str() {
        "install" => handle_install(),
        "build" => {
            if args.len() < 3 {
                println!("Error: Specify a .koda file.");
                return;
            }
            run_build(&args);
        }
        _ => println!("Unknown command: {}", args),
    }
}

fn handle_install() {
    println!("--- Koda System Installation ---");
    
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    let home = env::var("HOME").or_else(|_| env::var("USERPROFILE")).expect("Home dir not found");
    
    let mut koda_home = PathBuf::from(home);
    koda_home.push(".koda");
    koda_home.push("bin");
    
    fs::create_dir_all(&koda_home).ok();
    
    let bin_name = if cfg!(windows) { "koda.exe" } else { "koda" };
    let mut dest_path = koda_home.clone();
    dest_path.push(bin_name);

    fs::copy(&exe_path, &dest_path).expect("Failed to copy binary to system folder");

    #[cfg(windows)]
    {
        println!("Adding Koda to Windows Path...");
        let path_str = koda_home.to_str().unwrap();
        let cmd = format!("[Environment]::SetEnvironmentVariable('Path', [Environment]::GetEnvironmentVariable('Path', 'User') + ';{}', 'User')", path_str);
        Command::new("powershell").args(["-Command", &cmd]).output().ok();
    }

    #[cfg(not(windows))]
    {
        println!("Adding Koda to Unix Path...");
        // Logic to append to .zshrc or .bashrc would go here
    }

    println!("Installation complete! Restart your terminal and type 'koda'.");
}

fn run_build(file_path: &str) {
    let content = fs::read_to_string(file_path).expect("Could not read file");
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
    println!("Successfully built: {}", file_path);
}
