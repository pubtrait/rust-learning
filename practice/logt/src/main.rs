use chrono::Local;
use env_logger::Builder;
use log::*;
use std::io::Write;

mod other;

fn main() {
    // env_logger::init();
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {}:{} {}] {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    debug!("a debug log");
    info!("hello info");
    warn!("a warn log");
    error!("a error log");

    using_log();

    other::hello::world();
}

fn using_log() {
    info!("using a info log");
}
