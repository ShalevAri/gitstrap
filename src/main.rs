use clap::Parser;
use dialoguer::{Confirm, Input};
use std::process::{Command, exit};

/// Simple CLI tool to initialize a Git repository and push to a remote origin.
#[derive(Parser)]
#[command(name = "gitstrap")]
#[command(about = "Initializes a Git repo, adds initial commit, sets main branch, and pushes to the remote.", long_about = None)]
struct Args {
    /// Automatically confirm all prompts
    #[arg(short, long)]
    yes: bool,
}

fn run_command(cmd: &str, auto_yes: bool) -> bool {
    // If auto confirmation isn't enabled, ask for user confirmation.
    if !auto_yes {
        let confirmed = Confirm::new()
            .with_prompt(format!("Do you want to run: \"{}\"?", cmd))
            .default(false)
            .interact()
            .unwrap();
        if !confirmed {
            println!("Skipping: {}", cmd);
            return true; // Skip this command, treat as success.
        }
    }

    println!("Running: {}", cmd);
    // Execute the command using the shell.
    let status = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .status();

    match status {
        Ok(status) if status.success() => true,
        Ok(status) => {
            eprintln!("Command exited with non-zero status: {}", status);
            false
        }
        Err(e) => {
            eprintln!("Failed to execute command '{}': {}", cmd, e);
            false
        }
    }
}

fn main() {
    let args = Args::parse();
    let auto_yes = args.yes;

    // Prompt the user for the remote origin URL.
    let remote_url: String = Input::new()
        .with_prompt("Enter the remote origin URL (e.g., https://github.com/user/repo.git)")
        .interact_text()
        .unwrap();

    // Define the sequence of Git commands.
    let commands = vec![
        "git init".to_string(),
        "git add .".to_string(),
        "git commit -m \"initial commit\"".to_string(),
        "git branch -M main".to_string(),
        format!("git remote add origin {}", remote_url.trim()),
        "git push -u origin main".to_string(),
    ];

    // Execute each command in order.
    for cmd in commands {
        if !run_command(&cmd, auto_yes) {
            eprintln!("Error executing command. Exiting.");
            exit(1);
        }
    }
}