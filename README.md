## TCP Echo Benchmark Tool
A command-line utility written in Rust for benchmarking TCP echo server performance by simulating multiple clients sending and receiving data asynchronously.
#### Installation
```
cargo install tcp-echo-benchmark
```
#### Usage
```
tcp-echo-benchmark [-a <address>] [-l <length>] [-c <number>] [-t <duration>]

tcp-echo-benchmark

Options:
  -a, --address     address
  -l, --length      length
  -c, --number      number
  -t, --duration    duration
  --help, help      display usage information
```
#### Output example
```
Benchmarking localhost:8080 with 1 clients, 1 bytes, 5 seconds
Throughput: 69539 request/sec, 69539 response/sec
Total: 347696 requests, 347696 responses
```
