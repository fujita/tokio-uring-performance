# The performance of Tokio and Tokio-uring

Here is the result of simple disk I/O benchmark with Rust with io_uring, [Tokio-uring](https://github.com/tokio-rs/tokio-uring). One iteration of the benchmark spawns and awaits 2,000 async tasks. Each task reads 10 bytes from `/dev/urandom` and then writes them to `/dev/null`. The benchmark performs 1,000 iterations. The original Tokio version is taken from [reddit](https://www.reddit.com/r/rust/comments/lg0a7b/benchmarking_tokio_tasks_and_goroutines/) and I added the Tokio-uring version.

The existing application using [Tokio](https://github.com/tokio-rs/tokio) can be easily modified to use io_uring with Tokio-uring. However, you could see a performance degradation. For example, Tokio-uring version is about 8 times slower on EC2 c6gn.8xlarge (32 vCPUs).

![](https://github.com/fujita/tokio-uring-performance/raw/master/.github/assets/performance.png)

A major difference between Tokio and Tokio-uring is that Tokio runs async tasks on multiple CPU cores; Tokio-uring runs tasks on only one CPU core. With Tokio-uring, your application explicitly needs to use multiple CPU cores. The careful design is necessary to pull out the Tokio-uring potential.
