use std::process::{Command, Output};

pub fn run_command(command: &str) -> String {
    let output: Output = match Command::new("sh").arg("-c").arg(command).output() {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
            return String::new();
        }
    };

    if output.status.success() {
        // Remove extra chars from output
        let mut result = String::from_utf8_lossy(&output.stdout).to_string();
        result.retain(|c| !c.is_control()); // Remove control characters
        result.trim().to_string() // Remove blank spaces on start and end
    } else {
        eprintln!(
            "Command failed with error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::new()
    }
}
