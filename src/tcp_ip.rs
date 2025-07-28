use std::env;
use std::error::Error;
use std::fmt;
use std::io;
use std::net::{TcpListener, TcpStream};
use std::num::ParseIntError;

//
// IP Address
//
#[derive(Debug)]
pub enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpAddr::V4(a, b, c, d) => write!(f, "{a}.{b}.{c}.{d}"),
            IpAddr::V6(addr) => write!(f, "{addr}"),
        }
    }
}

impl IpAddr {
    pub fn from_string(input: String) -> Result<Self, IpParseError> {
        let parts: Vec<&str> = input.split('.').collect();
        if parts.len() == 4 {
            let nums: Result<Vec<u8>, _> = parts.iter().map(|p| p.parse()).collect();
            let nums = nums?;
            return Ok(IpAddr::V4(nums[0], nums[1], nums[2], nums[3]));
        }

        if input.contains(':') {
            return Ok(IpAddr::V6(input));
        }

        Err(IpParseError::InvalidFormat)
    }
}

//
// Socket Address + Error
//
#[derive(Debug)]
pub struct SocketAddr {
    pub ip_addr: IpAddr,
    pub port: u16,
}

impl fmt::Display for SocketAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.ip_addr, self.port)
    }
}

impl<'a> SocketAddr {
    pub fn from_env() -> Result<Self, SocketAddrError<'a>> {
        let ip = match env::var("IP_ADDR") {
            Ok(val) => val,
            Err(e) => {
                return Err(SocketAddrError::Env(e, "IP_ADDR"));
            }
        };
        let ip_addr = IpAddr::from_string(ip)?; // IpParseError → SocketAddrError

        let port_str = match env::var("PORT") {
            Ok(val) => val,
            Err(e) => {
                eprintln!("Missing 'PORT' in environment variables.");
                return Err(SocketAddrError::Env(e, "PORT"));
            }
        };
        let port = port_str.parse::<u16>()?;

        Ok(Self { ip_addr, port })
    }
}

#[derive(Debug)]
pub enum IpParseError {
    InvalidFormat,
    ParseError(ParseIntError),
}

impl fmt::Display for IpParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpParseError::InvalidFormat => write!(f, "Invalid IP address format"),
            IpParseError::ParseError(e) => write!(f, "Failed to parse IP component: {e}"),
        }
    }
}
impl Error for IpParseError {}
impl From<ParseIntError> for IpParseError {
    fn from(e: ParseIntError) -> Self {
        IpParseError::ParseError(e)
    }
}

#[derive(Debug)]
pub enum SocketAddrError<'a> {
    Env(env::VarError, &'a str),
    Port(ParseIntError),
    Ip(IpParseError),
}

impl<'a> fmt::Display for SocketAddrError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SocketAddrError::Env(e, v) => {
                write!(f, "Env var error: {e}. Missing environment variable: '{v}'")
            }
            SocketAddrError::Port(e) => write!(f, "Invalid port: {e}"),
            SocketAddrError::Ip(e) => write!(f, "IP parse error: {e}"),
        }
    }
}

impl<'a> Error for SocketAddrError<'a> {}

impl<'a> From<ParseIntError> for SocketAddrError<'a> {
    fn from(e: ParseIntError) -> Self {
        SocketAddrError::Port(e)
    }
}
impl<'a> From<IpParseError> for SocketAddrError<'a> {
    fn from(e: IpParseError) -> Self {
        SocketAddrError::Ip(e)
    }
}

//
// TCP Server
//
pub fn establish_tcp_conn(socket_addr: &SocketAddr) -> io::Result<()> {
    let socket_addr_str = socket_addr.to_string();

    let listener = TcpListener::bind(&socket_addr_str)?;
    println!("🚀 Listening on {socket_addr_str}");
    for stream in listener.incoming() {
        match stream {
            Ok(s) => println!("✅ Connection from {s:?}"),
            Err(e) => eprintln!("⚠️ Connection failed: {e}"),
        }
    }
    Ok(())
}
