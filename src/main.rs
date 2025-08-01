mod tcp_ip;

use tcp_ip::{SocketAddr, run_http_server};

fn main() {
    // Read IP-port from environment and/or .env file
    match dotenv::dotenv() {
        Ok(dotenv_file) => println!(
            "Found and loaded environment variables from {:?}.",
            dotenv_file
                .file_name()
                .and_then(|fname| fname.to_str())
                .unwrap_or("<unknown>")
        ),
        Err(_) => {
            println!("Could not found and load vars from .env file. Using current environment")
        }
    }

    match SocketAddr::from_env() {
        Ok(socket) => {
            if let Err(e) = run_http_server(&socket) {
                eprintln!("❌ Connection error: {e}");
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to parse socket address: {e}");
        }
    }
}
