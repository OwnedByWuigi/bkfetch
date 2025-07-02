#[cfg(target_os = "linux")]
use {
    crate::helpers::file::file_open,
    std::process::Command,
    std::fs::File,
    std::io::Read,
    std::rc::Rc,
};

#[cfg(target_os = "macos")]
pub fn get_arch() -> String {
    std::env::consts::ARCH.to_string()
}


#[cfg(target_os = "windows")]
pub fn get_cpu() -> String {
    use std::process::Command;

    let mut cpu = String::new();

    let output = Command::new("wmic")
        .args(&["cpu", "get", "name"])
        .output()
        .expect("Failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);

    for line in output.lines() {
        if !line.contains("Name") && line.trim().len() > 0 {
            cpu = line.to_string();
        }
    }

    cpu
}

#[cfg(target_os = "windows")]
pub fn get_ram_used() -> String {
    use std::process::Command;

    let mut ram_used = String::new();
    let mut ram_total = String::new();

    let output = Command::new("wmic")
        .args(&["OS", "get", "FreePhysicalMemory,TotalVisibleMemorySize"])
        .output()
        .expect("Failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);

    for line in output.lines() {
        if line.contains("Memory") || line.trim().len() == 0 {continue};

        let mut split = line.split_whitespace();

        ram_used = split.next().unwrap().to_string();
        ram_total = split.next().unwrap().to_string();
    }

    format!(
        "{}MB / {}MB",
        (ram_total.parse::<u64>().unwrap() - ram_used.parse::<u64>().unwrap()) / 1024,
        ram_total.parse::<u64>().unwrap() / 1024
    )
}

#[cfg(target_os = "windows")]
pub fn get_kernel() -> String {
    use std::process::Command;

    let mut kernel = String::new();

    let output = Command::new("wmic")
        .args(&["OS", "get", "Caption"])
        .output()
        .expect("Failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);

    for line in output.lines() {
        if !line.contains("Caption") && line.trim().len() > 0 {
            kernel = line.to_string();
        }
    }

    kernel
}

#[cfg(target_os = "windows")]
pub fn get_disk_usage() -> String {
    use std::process::Command;

    let mut disk = String::new();

    let output = Command::new("wmic")
        .args(&["logicaldisk", "get", "size,freespace,caption"])
        .output()
        .expect("Failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);

    for line in output.lines() {
        if line.contains("Caption") || line.trim().len() == 0 {continue};

        let mut split = line.split_whitespace();

        let name = split.next().unwrap().trim().to_string();
        let free = split.next().unwrap().trim().to_string();
        let size = split.next().unwrap().trim().to_string();

        disk.push_str(&format!(
            "{} {}GB / {}GB",
            name,
            (
                size.parse::<u64>().unwrap() - 
                free.parse::<u64>().unwrap()
            ) 
            / 1024 / 1024 / 1024,

            size.parse::<u64>().unwrap() 
            / 1024 / 1024 / 1024
        ));
    }

    disk
}

#[cfg(target_os = "linux")]
pub fn get_arch() -> String {
    Command::new("uname")
        .arg("-m")
        .output()
        .expect("Failed to execute process")
        .stdout
        .iter()
        .map(|&c| c as char)
        .collect::<String>()
        .trim().to_string()
}

#[cfg(target_os = "windows")]
pub fn get_arch() -> String {
    use std::process::Command;

    let mut uptime = String::new();

    let output = Command::new("wmic")
        .args(&["path", "Win32_OperatingSystem", "get", "OSArchitecture"])
        .output()
        .expect("Failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);

    for line in output.lines() {
        if !line.contains("OSArchitecture") && line.trim().len() > 0 {
            uptime = line.trim().to_string()
        }
    };

    uptime
}

#[cfg(target_os = "linux")]
pub fn get_cpu() -> String {
    let mut cpu: Rc<String> = Rc::new(String::new());
    let mut temp_buf: String = String::new();

    let mut file = File::open("/proc/cpuinfo").unwrap();
    file.read_to_string(&mut temp_buf).unwrap();

    let lines: &Vec<&str> = &temp_buf.lines().collect();

    lines.into_iter().for_each(|line| {
        if line.contains("model name") {
            cpu = Rc::new(
                line.split(":")
                    .collect::<Vec<&str>>()[1].to_string()
                    .replace("\t", "")
            );
            cpu = Rc::new(cpu.replacen(" ", "", 1));
        }
    });

    cpu.to_string()
}

#[cfg(target_os = "linux")]
pub fn get_ram_used() -> String {
    let temp_buf: String = file_open("/proc/meminfo");

    let lines: &Vec<&str> = &temp_buf.lines().collect();

    let mut total: u128 = 0;
    let mut available: u128 = 0;

    lines.into_iter().for_each(|line| {
        if line.contains("MemTotal") {
            total = eval_ram(line.to_string());
        } else if line.contains("MemAvailable") {
            available = eval_ram(line.to_string());
        }
    });

    format!(
        "{}M / {}M",
        total - available,
        total
    )
}

#[cfg(target_os = "linux")]
fn eval_ram(line: String) -> u128 {
    let kbs: u128 = line.split(":")
        .collect::<Vec<&str>>()[1].to_string()
        .replace("\t", "")
        .replace("kB", "")
        .replace(" ", "")
        .parse::<u128>()
        .unwrap();

    kbs / 1000
}

#[cfg(target_os = "windows")]
pub fn get_gpu() -> String {
    use std::process::Command;

    let mut gpu = String::new();

    let output = Command::new("wmic")
        .args(&["path", "win32_VideoController", "get", "Name"])
        .output()
        .expect("Failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);

    for line in output.lines() {
        if !line.contains("Name") && line.trim().len() > 0 {
            gpu = line.to_string();
        }
    }

    gpu
}

#[cfg(target_os = "macos")]
pub fn get_cpu() -> String {
    use std::process::Command;

    let output = Command::new("sysctl")
        .arg("-n")
        .arg("machdep.cpu.brand_string")
        .output();

    if let Ok(output) = output {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        "Unknown CPU".to_string()
    }
}
#[cfg(target_os = "macos")]
pub fn get_ram_used() -> String {
    use std::process::Command;

    // Get total memory
    let total_output = Command::new("sysctl")
        .arg("-n")
        .arg("hw.memsize")
        .output();

    // Get vm_stat output for active memory
    let vm_output = Command::new("vm_stat")
        .output();

    if let (Ok(total), Ok(vm)) = (total_output, vm_output) {
        let total_mem_bytes: f64 = String::from_utf8_lossy(&total.stdout)
            .trim()
            .parse::<f64>()
            .unwrap_or(0.0);

        let vm_output_str = String::from_utf8_lossy(&vm.stdout);
        let mut used_pages = 0.0;
        for line in vm_output_str.lines() {
            if line.contains("Pages active") || line.contains("Pages wired down") {
                if let Some(num) = line.split(':').nth(1) {
                    used_pages += num.trim().replace(".", "").parse::<f64>().unwrap_or(0.0);
                }
            }
        }

        let page_size = 4096.0; // default macOS page size
        let used_mem = used_pages * page_size;

        format!(
            "{:.1} GiB / {:.1} GiB",
            used_mem / (1024.0 * 1024.0 * 1024.0),
            total_mem_bytes / (1024.0 * 1024.0 * 1024.0)
        )
    } else {
        "Unknown RAM usage".to_string()
    }
}
