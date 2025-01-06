use zeromq::{Socket, PushSocket, PullSocket, SocketSend, SocketRecv};
use std::time::Instant;

const MSG_SIZE: usize = 1024;
const MSG_COUNT: usize = 100_000;

async fn zmq_throughput() -> zeromq::ZmqResult<f64> {
    let mut receiver = PullSocket::new();
    let mut sender = PushSocket::new();

    receiver.bind("tcp://127.0.0.1:5555").await?;
    sender.connect("tcp://127.0.0.1:5555").await?;

    let message = vec![1u8; MSG_SIZE];
    let start = Instant::now();
    
    let send_task = tokio::spawn(async move {
        for _ in 0..MSG_COUNT {
            sender.send(message.clone().into()).await?;
        }
        Ok::<_, zeromq::ZmqError>(())
    });

    let mut received = 0;
    while received < MSG_COUNT {
        receiver.recv().await?;
        received += 1;
    }

    send_task.await.unwrap()?;

    let duration = start.elapsed();
    Ok((MSG_SIZE * MSG_COUNT) as f64 / duration.as_secs_f64() / 1_000_000.0)
}

#[tokio::main]
async fn main() -> zeromq::ZmqResult<()> {
    let throughput = zmq_throughput().await?;
    println!("ZMQ Throughput: {:.2} MB/s", throughput);
    Ok(())
}