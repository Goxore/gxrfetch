use std::fs;
use std::env;
extern crate termion;
use termion::{color::Fg, color};
use systemstat::{System, Platform, saturating_sub_bytes};
use fsi;
use home;
use nixinfo;

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
    let art = fs::read_to_string(format!("{}{}",path,"herb")).
        expect("Error while reading asciiart");
    let artvec = art
        .lines()
        .map(|ln| String::from(ln))
        .collect::<Vec<String>>();
    artvec
}

fn get_config(path: &String) -> Vec<String> {
    let config = fs::read_to_string(format!("{}{}",path,"config")).
        expect("Error while reading asciiart");
    let confvec = config
        .lines()
        .map(|ln| String::from(ln))
        .collect::<Vec<String>>();
    confvec
}

fn get_info() -> Info {
    let mut info = Info::default();
    let cpu_info = fs::read_to_string("/proc/cpuinfo").
        expect("Error while reading cpu info");
    
    let mut cpu_name = String::from("not found");
    let mut cpu_cores = String::new();
    let lines = cpu_info
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();
    for line in lines {
        if line == "" {
            break;
        }
        let line = line.replace("\t", "");
        match line.split_once(":") {
            Some(("model name", s)) => cpu_name = String::from(s.trim()),
            Some(("siblings", s)) => cpu_cores = String::from(s.trim()),
            _ => (),
        }
    }
    info.cpu_name = cpu_name;
    info.cpu_cores = cpu_cores;

    let sys = System::new();

    match sys.battery_life() {
        Ok(battery) => info.battery = (battery.remaining_capacity*100.0).to_string(),
        Err(x) => print!("\nBattery: error: {}", x)
    }


    match sys.memory() {
        Ok(mem) => info.memory=format!("{} / {}", saturating_sub_bytes(mem.total, mem.free), mem.total),
        Err(x) => println!("\nMemory: error: {}", x)
    }

    info.os = format!("{}", env::consts::OS);


    match fsi::get_shell() {
        Ok(shell) => info.shell = shell,
        Err(x) => println!("err"),
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

fn check_contains(line: &String, contains: Cnts) -> String
{
  let newline = line.to_string();

  if newline.contains(&contains.0)
  {
    newline.replace(&contains.0, &contains.1)
  }else {
    newline.to_string()
  }

}

fn main() {
    let info = get_info();

    let mut path = String::new();

    match home::home_dir() {
        Some(dir) => path = format!("{}/.config/gxrfetch/", dir.display()),
        None => path = format!("./"),
    }

    let mut art = get_art(&path);
    let mut conf = get_config(&path);


    let artlen = &art.len();
    let conflen = &conf.len();
    let mut maxlength = &artlen;

    if maxlength < &conflen {maxlength = &conflen;}

    let linetoinfo: Vec<Cnts> = [
      ("[cpu]"  .to_string(), info.cpu_name),
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

      ("(r)"    .to_string(), Fg(color::Red).to_string()),
      ("(g)"    .to_string(), Fg(color::Green).to_string()),
      ("(y)"    .to_string(), Fg(color::Yellow).to_string()),
      ("(b)"    .to_string(), Fg(color::Blue).to_string()),
      ("(m)"    .to_string(), Fg(color::Magenta).to_string()),
      ("(c)"    .to_string(), Fg(color::Cyan).to_string()),
      ("(bg)"   .to_string(), Fg(color::Black).to_string()),
      ("(fg)"   .to_string(), Fg(color::White).to_string()),
      ("(rl)"   .to_string(), Fg(color::LightRed).to_string()),
      ("(gl)"   .to_string(), Fg(color::LightGreen).to_string()),
      ("(yl)"   .to_string(), Fg(color::LightYellow).to_string()),
      ("(bl)"   .to_string(), Fg(color::LightBlue).to_string()),
      ("(ml)"   .to_string(), Fg(color::LightMagenta).to_string()),
      ("(cl)"   .to_string(), Fg(color::LightCyan).to_string()),
      ("(bgl)"  .to_string(), Fg(color::LightBlack).to_string()),
      ("(fgl)"  .to_string(), Fg(color::LightWhite).to_string()),
    ].to_vec();

    for i in 0..**maxlength{
        if i <= artlen-1 && i <= conflen-1 {
          let mut concat = art[i].clone() + &conf[i].clone();
          
          for j in 0..linetoinfo.len() {
            concat = check_contains(&concat, linetoinfo[j].clone());
          }
          print!("{}", concat);

          println!();

        }else if i <= artlen-1
        {
          for k in 0..linetoinfo.len() {
              art[i] = check_contains(&art[i], linetoinfo[k].clone());
          }
          println!("{:20}", art[i]);

        }else if i <= conflen-1
        {
          for k in 0..linetoinfo.len() {
              conf[i] = check_contains(&conf[i], linetoinfo[k].clone());
          }
          println!("{:20}  {}"," ", conf[i]);
        }
    }
}
