use clap::{App, Arg};
use lib;

fn rw() -> tokio::task::JoinHandle<()> {
    use std::{
        fs::File,
        io::{Read, Write},
    };
    tokio::spawn(async move {
        let mut buffer = [0; 10];
        let mut dev_urandom = File::open("/dev/urandom").unwrap();
        dev_urandom.read(&mut buffer).unwrap();

        let mut dev_null = File::create("/dev/null").unwrap();
        dev_null.write(&mut buffer).unwrap();
    })
}

fn main() {
    let args = App::new("tokio-io")
        .arg(
            Arg::with_name("single")
                .long("single")
                .help("use only one CPU core"),
        )
        .get_matches();

    let rt = if args.is_present("single") {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
    } else {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
    };

    rt.block_on(lib::benchmark(rw));
}
