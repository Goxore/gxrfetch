use std::env;
use std::fs::{self,DirBuilder};
extern crate termion;
use fsi;
use home;
use nixinfo;
use systemstat::{saturating_sub_bytes, Platform, System};
use termion::{
    color::{self, Fg},
    style,
};

struct Info {
    kernel: String,
    cpu_name: String,
    cpu_cores: String,
    gpu_name: String,
    battery: String,
    memory: String,
    uptime: String,
    user: String,
    host: String,
    os: String,
    distro: String,
    shell: String,
    term: String,
    machine_name: String,
}

type Cnts = (String, String);

impl Default for Info {
    fn default() -> Self {
        Info {
            kernel: String::from(""),
            cpu_name: String::from("not found"),
            cpu_cores: String::from("not found"),
            gpu_name: String::from("not found"),
            battery: String::from("not found"),
            memory: String::from("not found"),
            uptime: String::from("not found"),
            user: String::from("not found"),
            host: String::from("not found"),
            os: String::from("not found"),
            distro: String::from("not found"),
            shell: String::from("not found"),
            term: String::from("not found"),
            machine_name: String::from("not found"),
        }
    }
}


fn get_art(path: &String) -> Vec<String> {
    let art =
        fs::read_to_string(format!("{}{}", path, "ascii")).expect("Error while reading asciiart");
    let artvec = art.lines().map(String::from).collect::<Vec<String>>();
    artvec
}

fn get_config(path: &String) -> Vec<String> {
    let config =
        fs::read_to_string(format!("{}{}", path, "config")).expect("Error while reading asciiart");
    let confvec = config.lines().map(String::from).collect::<Vec<String>>();
    confvec
}

fn get_info() -> Info {
    let mut info = Info::default();
    let cpu_info = fs::read_to_string("/proc/cpuinfo").expect("Error while reading cpu info");

    let mut cpu_name = String::from("not found");
    let mut cpu_cores = String::new();
    let lines = cpu_info.lines().map(String::from).collect::<Vec<String>>();
    for line in lines {
        if line.is_empty() {
            break;
        }
        let line = line.replace('\t', "");
        match line.split_once(':') {
            Some(("model name", s)) => cpu_name = String::from(s.trim()),
            Some(("siblings", s)) => cpu_cores = String::from(s.trim()),
            _ => (),
        }
    }
    info.cpu_name = cpu_name;
    info.cpu_cores = cpu_cores;

    let sys = System::new();

    match sys.battery_life() {
        Ok(battery) => info.battery = (battery.remaining_capacity * 100.0).to_string(),
        Err(x) => print!("\nBattery: error: {}", x),
    }

    match sys.memory() {
        Ok(mem) => {
            info.memory = format!(
                "{} / {}",
                saturating_sub_bytes(mem.total, mem.free),
                mem.total
            )
        }
        Err(x) => println!("\nMemory: error: {}", x),
    }

    info.os = env::consts::OS.to_string();

    match fsi::get_shell() {
        Ok(shell) => info.shell = shell,
        Err(x) => println!("error, {}", x),
    }

    match env::var("USER") {
        Ok(user) => info.user = user,
        Err(x) => println!("\nEnvironment variable $USER error: {}", x),
    }

    match nixinfo::distro() {
        Ok(result) => info.distro = result,
        Err(x) => println!("error, {}", x),
    }

    match nixinfo::hostname() {
        Ok(result) => info.host = result,
        Err(x) => println!("error, {}", x),
    }

    match nixinfo::device() {
        Ok(result) => info.machine_name = result,
        Err(x) => println!("error, {}", x),
    }

    match nixinfo::terminal() {
        Ok(result) => info.term = result,
        Err(x) => println!("error, {}", x),
    }

    match nixinfo::kernel() {
        Ok(result) => info.kernel = result,
        Err(x) => println!("error, {}", x),
    }

    match nixinfo::uptime() {
        Ok(result) => info.uptime = result,
        Err(x) => println!("error, {}", x),
    }

    // match nixinfo::gpu() {
    //     Ok(result) => info.gpu_name = result,
    //     Err(x) => println!("error, {}", x),
    // }

    info
}

fn check_contains(line: &String, contains: Cnts) -> String {
    let newline = line.to_string();

    if newline.contains(&contains.0) {
        newline.replace(&contains.0, &contains.1)
    } else {
        newline
    }
}

fn check_dir_existance(path: &String) -> bool
{
    std::path::Path::new(path).exists() 
}

fn generate_config(config_dir_path: &String) -> std::io::Result<()>
{

let default_ascii = "(y)          ██         
(y) ██      ████      ██
(y) ████    ████    ████
(y) ██ ██    ██    ██ ██
(y) ██ ██    ██    ██ ██
(y) ██  ██   ██   ██  ██
(y) ██  ██   ██   ██  ██
(y) ██  ██  ████  ██  ██
(y) █████   ████   █████
(y) ██  █  ██  ██  █  ██
(y) ██  █████  █████  ██
(y) ██   ██ ████ ██   ██
(y) ████████████████████
(y)      ███ ██ ███     
(y)        ██████       
(y)          ██         

";

let default_config = 
"  (m)<BI>[user](fgl)@(m)[host](fgl)
  (fg)----------------
  (b)<B> (fgl): <N>[name](fgl)
  (b)<B>﬙ (fgl): <N>[cpu](fgl)
  (b)<B> (fgl): <N>[cores] cores(fgl)
  (b)<B> (fgl): <N>[bat]%(fgl)
  (b)<B> (fgl): <N>[mem](fgl)
  (b)<B> (fgl): <N>[uptime](fgl)
  (b)<B> (fgl): <N>[distro](fgl)
  (b)<B> (fgl): <N>[kernel](fgl)
  (b)<B> (fgl): <N>[shell](fgl)
  (b)<B> (fgl): <N>[term](fgl)
";
    DirBuilder::new()
      .recursive(true)
      .create(config_dir_path)?;

    fs::write(format!("{}{}",config_dir_path,"config"), default_config)?;
    fs::write(format!("{}{}",config_dir_path,"ascii"), default_ascii)?;
    Ok(())
}

fn main() {
    

    let info = get_info();

    // let path;

    let path = match home::home_dir() {
        Some(dir) => format!("{}/.config/gxrfetch/", dir.display()),
        None => "./".to_string(),
    };

    if !check_dir_existance(&path) {
        generate_config(&path).unwrap();
    }

    let mut art = get_art(&path);
    let mut conf = get_config(&path);

    let artlen = &art.len();
    let conflen = &conf.len();
    let mut maxlength = artlen;

    if maxlength < conflen {
        maxlength = conflen;
    }

    let linetoinfo: Vec<Cnts> = [
        ("[cpu]".to_string(), info.cpu_name),
        ("[cores]".to_string(), info.cpu_cores),
        ("[bat]".to_string(), info.battery),
        ("[mem]".to_string(), info.memory),
        ("[uptime]".to_string(), info.uptime),
        ("[os]".to_string(), info.os),
        ("[user]".to_string(), info.user),
        ("[host]".to_string(), info.host),
        ("[distro]".to_string(), info.distro),
        ("[shell]".to_string(), info.shell),
        ("[kernel]".to_string(), info.kernel),
        ("[term]".to_string(), info.term),
        ("[name]".to_string(), info.machine_name),
        ("[gpu]".to_string(), info.gpu_name),

        ("(r)".to_string(), Fg(color::Red).to_string()),
        ("(g)".to_string(), Fg(color::Green).to_string()),
        ("(y)".to_string(), Fg(color::Yellow).to_string()),
        ("(b)".to_string(), Fg(color::Blue).to_string()),
        ("(m)".to_string(), Fg(color::Magenta).to_string()),
        ("(c)".to_string(), Fg(color::Cyan).to_string()),
        ("(bg)".to_string(), Fg(color::Black).to_string()),
        ("(fg)".to_string(), Fg(color::White).to_string()),
        ("(rl)".to_string(), Fg(color::LightRed).to_string()),
        ("(gl)".to_string(), Fg(color::LightGreen).to_string()),
        ("(yl)".to_string(), Fg(color::LightYellow).to_string()),
        ("(bl)".to_string(), Fg(color::LightBlue).to_string()),
        ("(ml)".to_string(), Fg(color::LightMagenta).to_string()),
        ("(cl)".to_string(), Fg(color::LightCyan).to_string()),
        ("(bgl)".to_string(), Fg(color::LightBlack).to_string()),
        ("(fgl)".to_string(), Fg(color::LightWhite).to_string()),

        ("<B>".to_string(), style::Bold.to_string()),
        ("<I>".to_string(), style::Italic.to_string()),
        ( "<BI>".to_string(), format!("{}{}", style::Bold, style::Italic)),
        ("<N>".to_string(), style::Reset.to_string()),
    ]
    .to_vec();

    for i in 0..*maxlength {
        if i < *artlen && i < *conflen {
            let mut concat = art[i].clone() + &conf[i].clone();

            for j in 0..linetoinfo.len() {
                concat = check_contains(&concat, linetoinfo[j].clone());
            }
            print!("{}", concat);

            println!();
        } else if i < *artlen {
            for k in 0..linetoinfo.len() {
                art[i] = check_contains(&art[i], linetoinfo[k].clone());
            }
            println!("{:20}", art[i]);
        } else if i < *conflen {
            for k in 0..linetoinfo.len() {
                conf[i] = check_contains(&conf[i], linetoinfo[k].clone());
            }
            println!("{:20}  {}", " ", conf[i]);
        }
    }
}
