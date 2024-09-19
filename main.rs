use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::env;
use std::process::Command;

fn read_cpu_info() -> io::Result<()> {
    let lines = read_specific_lines("/proc/cpuinfo", &["model name", "cpu MHz"])?;
    let mut model_name_printed = false;
    let mut core_count = 1;

    println!("CPU Information:");
    for line in lines {
        if line.contains("model name") && !model_name_printed {
            let cpu_name = line.split(':').nth(1).unwrap().trim(); /
            println!("  Cpu: {}", cpu_name); 
            model_name_printed = true;
        } else if line.contains("cpu MHz") {
            let cpu_mhz = line.split(':').nth(1).unwrap().trim(); 
            println!("  Core {}: {} MHz", core_count, cpu_mhz); 
            core_count += 1;
        }
    }
    println!();
    Ok(())
}

fn read_ram_info() -> io::Result<()> {
    let lines = read_specific_lines("/proc/meminfo", &["MemTotal", "MemFree", "MemAvailable", "SwapTotal", "SwapFree"])?;

    println!("RAM Information:");
    for line in lines {
        if let Some((label, value_kb)) = line.split_once(':') {
            let value_kb = value_kb.trim().split_whitespace().next().unwrap_or("0");
            let value_kb: f64 = value_kb.parse().unwrap_or(0.0);
            let value_mb = value_kb / 1024.0;
            println!("  {}: {:.2} MB", label.trim(), value_mb); 
        }
    }
    println!(); 
    Ok(())
}

fn read_gpu_info() -> io::Result<()> {
    let output = Command::new("lspci")
        .arg("-v")
        .output()?;

    let gpu_info = String::from_utf8_lossy(&output.stdout);

    if gpu_info.contains("VGA") || gpu_info.contains("3D") {
        println!("GPU Information:");
        for line in gpu_info.lines() {
            if line.contains("VGA") || line.contains("3D") {
                println!("  {}", line.trim());
            }
        }
    } else {
        println!("  No GPU Information found using lspci.");
    }

    println!(); 
    Ok(())
}

fn read_specific_lines(path: &str, keywords: &[&str]) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        let line = line?;
        for keyword in keywords {
            if line.contains(keyword) {
                lines.push(line);
                break;
            }
        }
    }

    Ok(lines)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("No argument provided, displaying all system information.\nIf you wanna, use: sysinfo [-cpu | -ram | -gpu]\n");
        read_cpu_info()?;
        read_ram_info()?;
        read_gpu_info()?;
    } else {
        match args[1].as_str() {
            "-cpu" => read_cpu_info()?,
            "-ram" => read_ram_info()?,
            "-gpu" => read_gpu_info()?,
            _ => println!("Invalid option. Usage: sysinfo [-cpu | -ram | -gpu]"),
        }
    }

    Ok(())
}
