#[derive(Debug, Default)]
pub struct Metrics {
    pub total_files: usize,
    pub total_functions: usize,
    pub block_count: usize,
    pub warn_count: usize,
    pub info_count: usize,
    pub pass_count: usize,
}
