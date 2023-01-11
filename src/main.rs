use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    net::UdpSocket,
    time::SystemTime
};

macro_rules! log {
    () => {
        println!()
    };
    ($($arg:tt)*) => {{
        println!("[{}] {}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis(), format!($($arg)*));
    }};
}


const BUFFER_SIZE_IN: usize = 1024;


fn print_map(map: &HashMap<String, String>) {
    log!("HashMap (Length {})", map.len());
    let fallback_str = String::from("[ INVALID ]");

    for key in map.keys() {
        let val = map
            .get(key)
            .unwrap_or(&fallback_str);

        println!("| {key}: {val}");
    }
}


fn load_config(path: &str) -> Result<HashMap<String, String>, std::io::Error> {
    let mut cfg: HashMap<String, String> = HashMap::new();

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    reader
        .lines()
        .enumerate()
        .for_each(|(i, l)| {
            if l.is_err() {
                eprintln!("Error reading config [Ln {i}]");
                return;
            }

            let line = l.expect("Internal Error -- assertion failed when checking if a line returned an error.");
            let entry = line.split_once("=");
            match entry {
                Some((a, b)) => { cfg.insert(String::from(a).to_lowercase(), String::from(b)); },
                None => {}
            }
        });

    return Ok(cfg);
}


fn main() -> std::io::Result<()> {
    log!("Hello, world!");

    let config = load_config("./serv_config.txt")
        .expect("Config should be defined in ./serv_config.txt");

    print_map(&config);

    let ip = &config.get("ip").expect("Missing field 'ip' from configuration.");
    let port = &config.get("port").expect("Missing field 'port' from configuration.");
    let addr = format!("{}:{}", ip, port);

    log!("Binding server to address: {}", addr);
    let server: UdpSocket = UdpSocket::bind(addr)
        .expect("Failed to bind to that ip/port combination.");
    let _r = server.set_read_timeout(None)
        .expect("Failed to setup UDP socked: Unable to disable read timeout.");

    let mut buffer_in: [u8; BUFFER_SIZE_IN] = [0; BUFFER_SIZE_IN];
    let write_buf = buffer_in.as_mut_slice();

    loop {
        let data_length: usize;
        let origin: std::net::SocketAddr;

        match server.recv_from(write_buf) {
            Ok((d, o)) => {
                data_length = d;
                origin = o;
            }

            Err(_err) => {
                eprintln!("Error while reading: {}", _err.to_string());
                break;
            }
        }

        let message_buf: Vec<u8> = write_buf
            .iter()
            .take(data_length)
            .map(|l| l.clone())
            .collect();

        let message = String::from_utf8(message_buf);
        let msg;

        match message {
            Ok(m) => { msg = m; }
            Err(err) => {
                log!("{}:{} ->! SERV | {} - {}", origin.ip(), origin.port(), "Err: Invalid message!", err.to_string());
                continue;
            }
        }

        log!("{}:{} --> SERV | {}", origin.ip(), origin.port(), msg);

        if msg == "exit" {
            println!("Recieved 'exit' keyword - Exiting");
            break;
        }
    }

    log!("Session Terminated.");
    return Ok(())
}
