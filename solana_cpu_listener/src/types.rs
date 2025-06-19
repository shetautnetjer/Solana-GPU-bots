// -------- src/types.rs -----------------------------------------
pub mod types {
    use serde::Serialize;

    #[derive(Serialize, Debug, Clone)]
    pub struct LogRow {
        pub timestamp: u64,
        pub slot: Option<u64>,
        pub account: String,
        pub mint: String,
        pub owner: String,
        pub ui_amount: f64,
        pub delta: f64,
        pub rolling_delta: f64,
    }
}
