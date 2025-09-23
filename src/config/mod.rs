pub struct AddrConfig {
    pub target: String,
    pub ports: Vec<u16>,
    pub timeout: u64,
    pub concurrency: u16,
}

pub struct ScanConfig {
    pub target: String,
    pub port: u16,
    pub timeout: u64,
}

pub enum DisplayFormat {
    Text,
    Json,
    Csv,
}

pub struct DisplayConfig {
    pub format: DisplayFormat,
    pub quiet: bool,
}
