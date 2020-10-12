//! The `snowflake` crate is an implement of [twitter's snowflake algorithm](https://github.com/twitter/snowflake)
//! written in rust. The bits are organized as follows:  
//! 
//! - 1 -> Future use
//! - 41 -> Epoch
//! - 10 -> Worker ID
//! - 12 -> Sequence counter, 0 through 4096
//! 
//! Author by [h_ang!(J27);](mailto:hunagjj.27@qq.com)

// TODO(huangjj.27@qq.com): make the EPOCH can be set by configurations
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use std::sync::Mutex;

use anyhow::{anyhow, Result};

const WORKER_ID_BITS: u8 = 10;
const SEQUENCE_BITS: u8 = 12;

// use bit operations to get max number of the item
const MAX_WORKER_ID: u16 = (-1 ^ (-1 << WORKER_ID_BITS)) as u16;
const MAX_SEQUENCE: u16 = (-1 ^ (-1 << SEQUENCE_BITS)) as u16;

// shift bits
const WORKER_ID_SHIFT: u8 = SEQUENCE_BITS;
const TIMESTAMP_LEFT_SHIFT: u8 = SEQUENCE_BITS + WORKER_ID_BITS;

#[derive(Debug)]
pub struct SnowFlakeWorker {
    // this worker id
    worker_id: u16,
    // the current millisecond sequence
    sequence: Mutex<u16>,
    // last generation timestamp
    last_timestamp: Mutex<SystemTime>,
}

impl SnowFlakeWorker {
    /// Creates a new SnowFlakeWorker instance.
    pub fn new(worker_id: u16) -> Self {
        assert!(worker_id <= MAX_WORKER_ID);
        
        SnowFlakeWorker {
            worker_id,
            sequence: Mutex::new(0),
            last_timestamp: Mutex::new(SystemTime::now()),
        }
    }

    /// Generates a new id and increments the current sequence counter
    pub fn next_id(&self) -> Result<u64> {
        let mut timestamp = SystemTime::now();
        let mut last_timestamp = self.last_timestamp.lock().map_err(|_| anyhow!("last_timestamp is poisoned"))?;

        if *last_timestamp > timestamp {
            return Err(anyhow!("Time went backwards!"));
        }

        let mut sequence = self.sequence.lock().map_err(|_| anyhow!("sequence is poisoned"))?;
        
        let difference = timestamp.duration_since(*last_timestamp)?;

        if difference.as_millis() == 0 {
            *sequence += 1;
            if *sequence > MAX_SEQUENCE {
                timestamp = block_until(timestamp);
                *sequence = 0;
            }
        } else {
            *sequence = 0;            
        }

        *last_timestamp = timestamp;

        let duration = timestamp
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::new(0, 0))
            .as_millis() as u64;

        Ok(
            (duration << TIMESTAMP_LEFT_SHIFT) as u64 |
            (self.worker_id << WORKER_ID_SHIFT) as u64 |
            *sequence as u64
        )
    }
}

fn block_until(until: SystemTime) -> SystemTime {
    let mut now;
    loop {
        now = SystemTime::now();
        if now > until {
            return now;
        }
    }
}