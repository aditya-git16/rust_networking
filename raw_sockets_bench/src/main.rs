use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::time::Instant;
use std::thread;

const MSG_SIZE: usize = 1024;
const MSG_COUNT: usize = 100_000;

fn raw_socket_throughput() -> std::io::Result<f64> {
    let listener = TcpListener::bind("127.0.0.1:8001")?;
    
    thread::spawn(move || {
        if let Ok((mut socket, _)) = listener.accept() {
            let mut buffer = vec![0u8; MSG_SIZE];
            let mut total_bytes = 0;
            while total_bytes < MSG_SIZE * MSG_COUNT {
                if let Ok(n) = socket.read(&mut buffer) {
                    total_bytes += n;
                }
            }
        }
    });

    let mut stream = TcpStream::connect("127.0.0.1:8001")?;
    let message = vec![1u8; MSG_SIZE];
    let start = Instant::now();
    
    for _ in 0..MSG_COUNT {
        stream.write_all(&message)?;
    }
    stream.flush()?;
    
    let duration = start.elapsed();
    let throughput = (MSG_SIZE * MSG_COUNT) as f64 / duration.as_secs_f64();
    Ok(throughput / 1_000_000.0) // MB/s
}

fn main() -> std::io::Result<()> {
    let throughput = raw_socket_throughput()?;
    println!("Throughput: {:.2} MB/s", throughput);
    Ok(())
}