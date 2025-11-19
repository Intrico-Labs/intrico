#[derive(Debug)]
pub struct BackendConfig {
    pub optimization_level: u8
}

impl Default for BackendConfig {
    fn default() -> Self {
        Self {
            optimization_level: 0
        }
    }
}