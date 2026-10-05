use std::path::Path;
use sysinfo::{Disks, Networks, System};

pub struct App {
    pub data: Vec<(&'static str, u64)>,
    pub sys: System,
    pub disks: Disks,
    pub networks: Networks,
}

impl App {
    pub fn default() -> Self {
        let sys = System::new_all();
        let disks = Disks::new_with_refreshed_list();
        let networks = Networks::new_with_refreshed_list();
        App {
            data: vec![("RAM", 0), ("CPU", 0), ("F DISK", 0), ("NET_TX", 0)],
            sys,
            disks,
            networks,
        }
    }
    pub fn update_all(&mut self) {
        self.update_disk_usage();
        self.update_network_usage();
        self.update_ram();
        self.update_cpu();
    }
    fn update_ram(&mut self) {
        self.sys.refresh_memory();

        let used_ram = self.sys.used_memory() / 1024 / 1024;
        if let Some(ram_entry) = self.data.iter_mut().find(|(label, _)| *label == "RAM") {
            ram_entry.1 = used_ram;
        }
    }

    fn update_cpu(&mut self) {
        self.sys.refresh_cpu_usage();

        let used_cpu = self.sys.global_cpu_usage().round() as u64;

        if let Some(entry) = self.data.iter_mut().find(|(label, _)| *label == "CPU") {
            entry.1 = used_cpu;
        }
    }
    fn update_disk_usage(&mut self) {
        self.disks.refresh(true);

        let mut available_space = 0;

        for disk in &self.disks {
            if disk.mount_point() == Path::new("/") {
                available_space = disk.available_space();
                break;
            }
        }

        let free_gb = available_space / 1_000_000_000;

        if let Some(entry) = self.data.iter_mut().find(|(label, _)| *label == "F DISK") {
            entry.1 = free_gb;
        }
    }

    fn update_network_usage(&mut self) {
        self.networks.refresh(true);

        let mut total_transmitted_bytes = 0;

        for (_name, network) in &self.networks {
            total_transmitted_bytes += network.transmitted();
        }

        let tx_kb = total_transmitted_bytes / 1024;

        if let Some(entry) = self.data.iter_mut().find(|(label, _)| *label == "NET_TX") {
            entry.1 = tx_kb;
        }
    }
}
