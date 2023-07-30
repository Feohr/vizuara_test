use std::process::Command;

fn main() {
    let port = std::fs::read_to_string("SERVE_PORT").unwrap();
    let _ = Command::new("python3")
        .args([
            "-m",
            "http.server",
            &format!("{}", port.trim()),
            "-d",
            "data/Push_force",
        ])
        .spawn()
        .unwrap();
}
