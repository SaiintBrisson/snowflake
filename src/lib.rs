use std::time::{SystemTime, Duration, UNIX_EPOCH};
use std::sync::Mutex;

use anyhow::{anyhow, Result};

const IDENTIFIER_BITS: u64 = 10;
const SEQUENCE_BITS: u64 = 12;

const MAX_IDENTIFIER: u64 = (-1 ^ (-1 << IDENTIFIER_BITS)) as u64;
const MAX_SEQUENCE: u64 = (-1 ^ (-1 << SEQUENCE_BITS)) as u64;

const IDENTIFIER_SHIFT: u64 = SEQUENCE_BITS;
const TIMESTAMP_LEFT_SHIFT: u64 = SEQUENCE_BITS + IDENTIFIER_BITS;

#[derive(Debug)]
pub struct SnowflakeGenerator {
    // this worker id
    identifier: u64,
    // the current millisecond sequence
    sequence: Mutex<u64>,
    // last generation timestamp
    last_timestamp: Mutex<u64>,
}

impl SnowflakeGenerator {
    /// Creates a new SnowflakeGenerator instance.
    pub fn new(identifier: u64) -> Self {
        assert!(identifier <= MAX_IDENTIFIER);
        
        SnowflakeGenerator {
            identifier,
            sequence: Mutex::new(0),
            last_timestamp: Mutex::new(Self::get_millis()),
        }
    }

    /// Generates a new id and increments the current sequence counter
    pub fn next_id(&self) -> Result<u64> {
        let mut timestamp = Self::get_millis();
        let mut last_timestamp = self.last_timestamp.lock().map_err(|_| anyhow!("last_timestamp is poisoned"))?;

        if *last_timestamp > timestamp {
            return Err(anyhow!("Time went backwards!"));
        }

        let mut sequence = self.sequence.lock().map_err(|_| anyhow!("sequence is poisoned"))?;
        
        if timestamp - *last_timestamp == 0 {
            *sequence += 1;
            if *sequence > MAX_SEQUENCE {
                timestamp = Self::block_until(timestamp);
                *sequence = 0;
            }
        } else {
            *sequence = 0;            
        }

        *last_timestamp = timestamp;

        Ok(timestamp << TIMESTAMP_LEFT_SHIFT | self.identifier << IDENTIFIER_SHIFT | *sequence)
    }

    fn get_millis() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::new(0, 0))
            .as_millis() as u64
    }

    fn block_until(until: u64) -> u64 {
        let mut now = Self::get_millis();
        while now <= until {
            now = Self::get_millis();
        }
        now
    }
}

impl Default for SnowflakeGenerator {
    fn default() -> Self {
        Self::new(0)
    }
}