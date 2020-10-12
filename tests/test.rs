extern crate snowflake;

use snowflake::SnowflakeGenerator;

#[test]
fn it_works() {
    let instant = std::time::Instant::now();
    let generator = SnowflakeGenerator::default();

    for _ in 0..50000 {
        assert!(generator.next_id().is_ok())
    }

    let duration: std::time::Duration= std::time::Instant::now() - instant;
    println!("Generating 50000 IDs took: {}ms", duration.as_millis())
}