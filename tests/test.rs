extern crate snowflake;

use snowflake::SnowFlakeWorker;

#[test]
fn it_works() {
    let worker = SnowFlakeWorker::new(0);
    assert!(worker.next_id().is_ok())
}