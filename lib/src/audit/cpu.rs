#[derive(Debug)]
pub struct Cpu {
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub frequency: Option<u64>,
}

pub fn get_cpu_info() -> Vec<Cpu> {
    let mut result = Vec::new();
    result
}
