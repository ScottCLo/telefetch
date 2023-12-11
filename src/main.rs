use std::env;
use sysinfo::CpuExt;
use sysinfo::System;

use sysinfo::SystemExt;


fn main() {
    let args: Vec<String> = env::args().collect();
    let sys = System::new_all();
    let (os, distro) = get_os_info(&sys);
    let mut ascii = get_ascii(&os, &distro);
    
    for arg in &args[1..] {
        match arg.as_str() {
            "--arch" => ascii = ASCII_ARCH,
            "--void" => ascii = ASCII_VOID,
            "--linux" => ascii = ASCII_ARCH,
            "--defalt" => ascii = ASCII_DEFAULT,
            _ => ()
        }
    }

    let mut info: Vec<String> = Vec::new();
    info.push(get_user_host(&sys));
    info.push(get_os(&sys));
    info.push(get_kernel(&sys));
    info.push(get_uptime(&sys));
    info.push(get_cpu(&sys));
    info.push(get_memory(&sys));

    for (i, ascii_line) in ascii.lines().enumerate() {
        let mut info_line: &String = &String::from("");
        if i < info.len() {
            info_line = &info[i];
        }
        print!("\r{:<13} {}\n", ascii_line, info_line);
    }
}

fn get_os_info(sys: &System) -> (String, String) {
    let mut os = "";
    let mut distro = "";
    let os_info = sys.long_os_version().unwrap();
    let info: Vec<&str> = os_info.split_whitespace().collect();
    if info.len() > 1 && info[0] == "Linux" || info[0] == "linux" {
        os = info[0];
        distro = info[1];
    }
    (os.to_string(), distro.to_string()) 
}

fn get_user_host(sys: &System) -> String {
    let user: String = env::var_os("USER").unwrap().into_string().unwrap();
    let host: String = sys.host_name().unwrap();
    format!("{user}@{host}")
}


fn get_os(sys: &System) -> String {
    let mut os = sys.long_os_version().unwrap();
    let os_info: Vec<&str> = os.split_whitespace().collect();
    if os_info.len() > 1 && os_info.iter().any(|&i| i == "Linux") {
        os = format!("{} {}", os_info[1], "Linux");
    }

    format!("{:<6} {}", "os", os )
}

fn get_kernel(sys: &System) -> String {
    let kernel = sys.kernel_version().unwrap();
    format!("{:<6} {}", "kernel", kernel)
}

fn get_uptime(sys: &System) -> String {
    let seconds: u64 = sys.uptime();
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minuites = (seconds % 3600) / 60;
    let uptime = format!("{}d {}h {}m", days, hours, minuites);
    format!("{:<6} {}", "uptime", uptime)
}

fn get_memory(sys: &System) -> String {
    let used = sys.used_memory() / 1048576;
    let total = sys.total_memory() / 1048576;
    let memory = format!("{}M / {}M", used, total);
    format!("{:<6} {}", "memory", memory)
}

fn get_cpu(sys: &System) -> String {
    let cpu = sys.global_cpu_info().brand();
    format!("{:<6} {}", "cpu", cpu)
}

fn get_ascii<'a>(os: &str, distro: &str) -> &'a str {
    match os {
        "Linux" | "linux" => {
            match distro {
                "Arch" | "arch" => ASCII_ARCH,
                "Void" | "void" => ASCII_VOID,
                    _ => ASCII_LINUX
            }
        },
        _ => ASCII_DEFAULT
    }
}
static ASCII_DEFAULT: &str =
" ___________ 
|_   _|  ___|
  | | | |
  | | |  _|
  | | | |
  \\_/ \\_|";

static ASCII_LINUX: &str =
"    ---
   (o o:
   (<> :
  / __  \\
 : \\  \\/ )
--.'__/.--
\\_/____\\_/";



static ASCII_ARCH: &str = 
"     /\\    
    /  \\    
   /'.  \\   
  /  __  \\  
 /  :  :'.\\ 
/_--'  '--_\\";

static ASCII_VOID: &str =
"   ______
 _ \\____ \\
: \\  __ \\ \\
: : /  \\ \\ :
: \\ \\__/ : :
 - \\____ \\_:
  \\_____\\";
