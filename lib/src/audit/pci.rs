use pci_info::{PciInfo, pci_enums::PciDeviceClass};
use pciid_parser::{Database, find_device_name, find_vendor_name};

pub struct Gpu {
    vendor: Option<String>,
    name: Option<String>,
}

pub fn get_gpu_info() -> Vec<Gpu> {
    let db = Database::read().unwrap();
    let info = PciInfo::enumerate_pci().unwrap();
    let mut result = Vec::new();
    for i in info {
        let device = i.unwrap();
        if matches!(
            device.device_class().unwrap(),
            PciDeviceClass::DisplayController
        ) {
            let vendor_id = device.vendor_id();
            let device_id = device.device_id();
            let vendor = find_vendor_name(vendor_id).unwrap_or_default();
            let name = find_device_name(vendor_id, device_id).unwrap_or_default();
            result.push(Gpu { vendor, name })
        }
    }
    result
}
