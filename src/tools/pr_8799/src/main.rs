mod virtcontainers;

use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::process::{exit, Command};
use virtcontainers::get_ch_vcpu_tids_by_path;

fn get_ch_vcpu_tids(pid: u32) -> Result<HashMap<u32, u32>> {
    let proc_path = format!("/proc/{pid}");

    let map = get_ch_vcpu_tids_by_path(&proc_path)?;
    Ok(map)
}

fn real_main() -> Result<()> {
    // Assuming there is a kata container + clh running in the background.
    let output = Command::new("pidof")
        .arg("/opt/kata/bin/cloud-hypervisor")
        .output()
        .expect("Failed to execute the pidof of cloud-hypervisor");

    if !output.status.success() {
        return Err(anyhow!(
            "No process found: {}",
            std::str::from_utf8(&output.stderr)?
        ));
    }

    let lines = std::str::from_utf8(&output.stdout)?
        .lines()
        .collect::<Vec<_>>();

    if lines.is_empty() {
        return Err(anyhow!("No cloud-hypervisor processes found"));
    }

    let pid: u32 = lines[0].parse()?;

    let r = match get_ch_vcpu_tids(pid) {
        Ok(m) => m,
        Err(e) => return Err(anyhow!("{}", e)),
    };

    println!("map: {:?}", r);
    return Ok(());
}

fn main() {
    if let Err(e) = real_main() {
        println!("Error found: {:?}", e);
        exit(1);
    }
}
