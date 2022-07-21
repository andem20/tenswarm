pub const NUM_REQUESTS: usize = 100_000;
pub const REQS_PR_SECOND: u64 = 10_000;
pub const DELAY_NANOS: u64 = 1_000_000_000 / REQS_PR_SECOND;
pub const URL: &'static str = "http://localhost:9090";