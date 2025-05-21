#[derive(Debug)]
pub struct Cpu {
    vendor: Option<String>,
    model: Option<String>,
}

impl Cpu {
    pub fn get_cpu_info() -> Vec<Cpu> {
        let mut result = Vec::new();
    }
}
