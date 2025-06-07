use clap::Parser;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{Duration, sleep};

/// tcp-echo-benchmark
#[derive(Parser, Debug)]
struct Args {
    /// address
    #[arg(short = 'a', default_value_t = String::from("127.0.0.1:12345"))]
    address: String,

    /// length
    #[arg(short = 'l', default_value_t = 512)]
    length: usize,

    /// number
    #[arg(short = 'c', default_value_t = 50)]
    number: u32,

    /// duration
    #[arg(short = 't', default_value_t = 60)]
    duration: u64,
}

struct Count {
    i: u64,
    o: u64,
}

#[tokio::main]
async fn main() {
    let Args {
        address,
        length,
        number,
        duration,
    } = Args::parse();

    let stop = Arc::new(AtomicBool::new(false));
    let mut handles = Vec::with_capacity(number as usize);

    for _ in 0..number {
        let address = address.clone();
        let stop = stop.clone();
        let length = length;

        let handle = tokio::spawn(async move {
            let mut sum = Count { i: 0, o: 0 };
            let mut buf = vec![0; length];
            buf[length - 1] = b'\n';
            let mut stream = TcpStream::connect(&*address).await.unwrap();
            stream.set_nodelay(true).unwrap();

            loop {
                if stop.load(Ordering::Relaxed) {
                    break;
                }

                // Write data
                match stream.write_all(&buf).await {
                    Err(_) => {
                        eprintln!("Write error!");
                        break;
                    }
                    Ok(_) => sum.o += 1,
                }

                if stop.load(Ordering::Relaxed) {
                    break;
                }

                // Read echoed data
                match stream.read_exact(&mut buf).await {
                    Err(_) => break,
                    Ok(_) => sum.i += 1,
                }
            }
            sum
        });
        handles.push(handle);
    }

    sleep(Duration::from_secs(duration)).await;
    stop.store(true, Ordering::Relaxed);

    let mut total = Count { i: 0, o: 0 };
    for handle in handles {
        let count = handle.await.unwrap();
        total.i += count.i;
        total.o += count.o;
    }

    println!(
        "Benchmarking {} with {} clients, {} bytes, {} seconds",
        address, number, length, duration
    );
    println!(
        "Throughput: {:.0} request/sec, {:.0} response/sec",
        total.o / duration,
        total.i / duration
    );
    println!("Total: {} requests, {} responses", total.o, total.i);
}
