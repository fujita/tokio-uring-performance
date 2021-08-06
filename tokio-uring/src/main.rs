use lib;

fn uring() -> tokio::task::JoinHandle<()> {
    use tokio_uring::fs::File;

    tokio_uring::spawn(async move {
        let buffer = vec![0; 10];
        let dev_urandom = File::open("/dev/urandom").await.unwrap();
        let (_, buffer) = dev_urandom.read_at(buffer, 0).await;

        let dev_null = File::create("/dev/null").await.unwrap();
        let (_, _) = dev_null.write_at(buffer, 0).await;
    })
}

fn main() {
    tokio_uring::start(lib::benchmark(uring));
}
