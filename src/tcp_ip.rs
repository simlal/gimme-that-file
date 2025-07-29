use std::{
    env,
    error::Error,
    fmt, fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    num::ParseIntError,
};

//
// Basic IP Address
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
// Basic Socket Address + Error
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
// HACK: Remove instead of impl a constructor for From trait/func
// impl<'a> From<env::VarError> for SocketAddrError<'a> {
//     fn from(e: env::VarError, v: &'a str) -> Self {
//         SocketAddrError::Env(e, v)
//     }
// }

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
// Simple TCP Server
//
pub fn run_http_server(socket_addr: &SocketAddr) -> std::io::Result<()> {
    let socket_addr_str = socket_addr.to_string();

    let listener = TcpListener::bind(&socket_addr_str)?;
    println!("🚀 Listening on {socket_addr_str}");
    loop {
        let tcp_stream = accept_loop(&listener)?;
        handle_connection(&tcp_stream)?;
    }
}

fn accept_loop(listener: &TcpListener) -> Result<TcpStream, std::io::Error> {
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                println!("✅ Connection from {s:?}");
                return Ok(s);
            }
            Err(e) => eprintln!("⚠️ Connection failed: {e}"),
        }
    }
    // Listener gets closed if loop ends
    Err(std::io::Error::other("Listener closed unexpectedly"))
}

fn handle_connection(mut stream: &TcpStream) -> Result<(), std::io::Error> {
    let buf_reader = BufReader::new(stream);
    // let mut request_line = String::new();
    // let _ = buf_reader.read_line(&mut request_line);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map_while(Result::ok)
        .take_while(|line| !line.is_empty())
        .collect();

    // TEST: err missing req
    // let http_request: Vec<u8> = vec![];

    if let Some(request_line) = http_request.first() {
        // TODO: handle routing
        println!("{request_line}");

        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("index.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
        Ok(())
    } else {
        Err(std::io::Error::other(
            "Malformed or empty HTTP request: {http_request:}",
        ))
    }
}
