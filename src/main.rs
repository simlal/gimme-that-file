mod tcp_ip;

use tcp_ip::{SocketAddr, establish_tcp_conn};

fn main() {
    // Read IP-port from environment and/or .env file
    let _ = dotenv::dotenv();

    match SocketAddr::from_env() {
        Ok(socket) => {
            if let Err(e) = establish_tcp_conn(&socket) {
                eprintln!("❌ Connection error: {e}");
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to parse socket address: {e}");
        }
    }
}
