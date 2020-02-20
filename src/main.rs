use async_std::task;
use flydb;

fn main() {
    let reader_task = task::spawn(flydb::run());
    println!("Started task!");
    task::block_on(reader_task);
    println!("Stopped task!");
}
