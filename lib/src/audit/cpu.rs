use dmidecode::EntryPoint;

#[derive(Debug)]
pub struct Cpu {
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub frequency: Option<u64>,
}

const DMIDECODE_BIN: &[u8] = include_bytes!("/usr/bin/dmidecode");

pub fn get_cpu_info() -> String {
    let entry_point = EntryPoint::search(DMIDECODE_BIN).unwrap();
    let structures =
        entry_point.structures(&DMIDECODE_BIN[entry_point.smbios_address() as usize..]);
    format!("{:#?}", structures)
}
