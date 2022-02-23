use std::collections::HashSet;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = Command::new("git")
        .args(["reflog"])
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;
    let mut branch_set = HashSet::new();
    let mut branches = Vec::new();
    let stdout = output.stdout.take().unwrap();
    for line in BufReader::new(stdout).lines().filter_map(|r| r.ok()) {
        if !line.contains("checkout") || line.contains("to origin/master") {
            continue;
        }
        let branch = String::from(line.split(' ').last().expect("Bad reflog format"));
        if branch_set.insert(branch.clone()) {
            branches.push(branch)
        }
        if branches.len() >= 10 {
            break;
        }
    }
    let _ = output.kill();
    use terminal_menu::{menu, button, run, mut_menu};
    let menu = menu(
        branches.iter().map(|b| button(b)).collect()
    );
    run(&menu);
    let guard = mut_menu(&menu);
    if !guard.canceled() {
        checkout(guard.selected_item_name())?;
    }
    Ok(())
}

fn checkout(branch: &str) -> Result<(), Box<dyn Error>> {
    let out = Command::new("git")
        .args(["checkout", branch])
        .stdout(Stdio::piped())
        .output()?;
    println!("{}", String::from_utf8_lossy(out.stdout.as_slice()));
    Ok(())
}
