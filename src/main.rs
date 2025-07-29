mod tcp_ip;

use tcp_ip::{SocketAddr, run_http_server};

fn main() {
    // Read IP-port from environment and/or .env file
    let _ = dotenv::dotenv();

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
