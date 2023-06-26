use std::io::Result;
use std::process::Command;

fn main() -> Result<()> {
    let output = Command::new("cat")
    .arg("/etc/passwd")
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
