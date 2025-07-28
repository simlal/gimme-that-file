mod tcp_ip;

use tcp_ip::{SocketAddr, establish_tcp_conn};

fn main() {
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
