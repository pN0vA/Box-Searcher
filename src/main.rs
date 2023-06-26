use std::io::Result;
use std::io;
use std::process::Command;

fn main() -> Result<()> {
    println!("What is the command you want:");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    choice = choice.trim_end().to_string();
    println!("What is the extra you want:");
    let mut argu = String::new();
    io::stdin().read_line(&mut argu).expect("Failed to read line");
    argu = argu.trim_end().to_string();
    let output = Command::new(choice)
    .arg(argu)
    .output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed: {}", stderr);
    }

    Ok(())
}
