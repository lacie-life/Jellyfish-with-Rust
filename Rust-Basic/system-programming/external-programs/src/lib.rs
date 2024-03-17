use std::process::Command;
use serde_json;
use log::{error, debug};

fn run_command(command: &str) -> String {
    debug!("Raw command: {command}");
    let args: Vec<&str> = command.split(" ").collect();
    debug!("Raw command split: {args:?}");
    let output = Command::new(args[0])
        .args(&args[1..])
        .output();
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.to_string()
        },
        Err(error) => {
            println!("Command failed: {command}");
            error!("error: {}", error);
            "".to_string()
        }
    }

}

// make this json serializable
#[derive(Debug, serde::Serialize)]
pub struct Filesystem {
    pub filesystem: String,
    pub size: String,
    pub used: String,
    pub available: String,
    pub use_percent: String,
    pub mounted_on: String,
}

impl Filesystem {
    pub fn new(filesystem: String, size: String, used: String, available: String, use_percent: String, mounted_on: String) -> Filesystem {
        Filesystem {
            filesystem,
            size,
            used,
            available,
            use_percent,
            mounted_on,
        }
    }
}

pub fn parse_df_output(input: &str) -> Vec<Filesystem> {
  // Parse output of df command:
  // Filesystem      1K-blocks       Used  Available Use% Mounted on
  // tmpfs             3260284       2628    3257656   1% /run
  // /dev/nvme0n1p5  504591304  251393624  227492416  53% /
  // tmpfs            16301416     316652   15984764   2% /dev/shm
  // tmpfs                5120          4       5116   1% /run/lock
  // efivarfs              256        134        118  54% /sys/firmware/efi/efivars
  // /dev/nvme0n1p2      98304      33052      65252  34% /boot/efi
  // /dev/sda4      3727672592 2993917620  544324804  85% /mnt/Data
  // /dev/sda2      2071704572  868010264 1203694308  42% /mnt/Data-2
  // tmpfs             3260280       6896    3253384   1% /run/user/1000
  // /dev/sdb1      7814006784 6252931072 1561075712  81% /media/lacie/NAS

  let mut devices: Vec<Filesystem> = Vec::new();

  for line in input.lines() {
    if line.starts_with("Filesystem") {
        debug!("Skipping header line: {line}");
      continue;
    }
    if line.len() == 0 {
        debug!("skipping that is empty");
      continue;
    }
    let mut parts = line.split_whitespace();
    let filesystem = parts.next().unwrap().to_string();
    let size = parts.next().unwrap().to_string();
    let used = parts.next().unwrap().to_string();
    let available = parts.next().unwrap().to_string();
    let use_percent = parts.next().unwrap().to_string();
    let mounted_on = parts.next().unwrap().to_string();
    let device = Filesystem::new(filesystem, size, used, available, use_percent, mounted_on);
    devices.push(device);
  }
devices
}

pub fn which_executable(command: &str) -> String {
    // find in different system paths the executable
    let acceptable_paths = vec!["/bin", "/usr/bin", "/usr/local/bin"];
    for path in acceptable_paths {
        let full_path = format!("{}/{}", path, command);
        // if the path exists then return it
        if std::path::Path::new(&full_path).exists() {
            return full_path;
        }
    }
    return command.to_string();
}

pub fn run_df(path: &str) -> serde_json::Value {
    let command = "idf";
    let output = run_command(command);
    if output.is_empty() {
        error!("No output from command: {command}");
        return serde_json::json!({});
    }

    // serialize the result of the parsing and return the JSON array
    let devices = parse_df_output(&output);
    if path.len() == 0 {
        return serde_json::json!(devices);
    } else {
        for device in devices {
            if device.mounted_on == path {
                return serde_json::json!(device);
            }
        }.into()
    }
}
