use std::env;
use std::fs::{self,DirBuilder};
use std::io::{self, Result};
extern crate termion;
use systemstat::{saturating_sub_bytes, Platform, System};
use termion::{
    color::{self, Fg},
    style,
};


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


fn matchvalue(result: io::Result<String>) -> String
{
    match result {
        Ok(result) => result,
        Err(x) => format!("err, {}",x),
    }
}

fn get_specific(name: &str) -> String
{

    let sys = System::new();
    
    match name
    {
        "[cpu]" => matchvalue(nixinfo::cpu()),
        "[uptime]" => matchvalue(nixinfo::uptime()),
        "[os]" => env::consts::OS.to_string(),
        "[user]" => env::var("USER").unwrap(),
        "[host]" => matchvalue(nixinfo::hostname()),
        "[distro]" => matchvalue(nixinfo::distro()),
        "[shell]" => fsi::get_shell().unwrap_or("not found".to_string()),
        "[kernel]" => matchvalue(nixinfo::kernel()),
        "[term]" => matchvalue(nixinfo::terminal()),
        "[name]" => matchvalue(nixinfo::device()),
        "[gpu]" => matchvalue(nixinfo::gpu()),
        "[cores]" =>
        {
            let cpu_info = fs::read_to_string("/proc/cpuinfo").expect("Error while reading cpu info");

            let mut cpu_cores = String::new();
            let lines = cpu_info.lines().map(String::from).collect::<Vec<String>>();
            for line in lines {
                if line.is_empty() {
                    break;
                }
                let line = line.replace('\t', "");
                if let Some(("siblings",s)) = line.split_once(':'){cpu_cores = String::from(s.trim())}
            }
            cpu_cores
        },
        "[bat]" =>  
        match sys.battery_life() {
            Ok(battery) => (battery.remaining_capacity * 100.0).to_string(),
            Err(x) => format!("err, {}", x).to_string(),
        },
        "[mem]" => 
        match sys.memory() {
            Ok(mem) => format!( "{} / {}", saturating_sub_bytes(mem.total, mem.free), mem.total),
            Err(x) => format!("err, {}", x),
        },

        "(r)" => Fg(color::Red).to_string(),
        "(g)" => Fg(color::Green).to_string(),
        "(y)" => Fg(color::Yellow).to_string(),
        "(b)" => Fg(color::Blue).to_string(),
        "(m)" => Fg(color::Magenta).to_string(),
        "(c)" => Fg(color::Cyan).to_string(),
        "(bg)" => Fg(color::Black).to_string(),
        "(fg)" => Fg(color::White).to_string(),
        "(rl)" => Fg(color::LightRed).to_string(),
        "(gl)" => Fg(color::LightGreen).to_string(),
        "(yl)" => Fg(color::LightYellow).to_string(),
        "(bl)" => Fg(color::LightBlue).to_string(),
        "(ml)" => Fg(color::LightMagenta).to_string(),
        "(cl)" => Fg(color::LightCyan).to_string(),
        "(bg)" => Fg(color::LightBlack).to_string(),
        "(fg)" => Fg(color::LightWhite).to_string(),
        "<B>"  => style::Bold.to_string(),
        "<I>"  => style::Italic.to_string(),
        "<BI>" => format!("{}{}", style::Bold, style::Italic),
        "<N>"  => style::Reset.to_string(),
        _=> "".to_string()

    }
}


fn check_contains(line: &String, contains: String) -> String {
    let newline = line.to_string();

    if newline.contains(&contains) {
        newline.replace(&contains, &get_specific(&contains))
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

    let path = match home::home_dir() {
        Some(dir) => format!("{}/.config/gxrfetch/", dir.display()),
        None => "./".to_string(),
    };

    if !check_dir_existance(&path) {
        generate_config(&path).unwrap();
    }

    let mut art = get_art(&path);
    let mut conf = get_config(&path);

    let artlen = art.len();
    let conflen = conf.len();
    let mut maxlength = artlen;

    if maxlength < conflen {
        maxlength = conflen;
    }

    let linetoinfo: Vec<&str> = [
        "[cpu]",   
        "[cores]", 
        "[bat]",   
        "[mem]",   
        "[uptime]",
        "[os]",    
        "[user]",  
        "[host]",  
        "[distro]",
        "[shell]", 
        "[kernel]",
        "[term]",  
        "[name]",  
        "[gpu]",  
        "(r)",
        "(g)",
        "(y)",
        "(b)",
        "(m)",
        "(c)",
        "(bg)",
        "(fg)",
        "(rl)",
        "(gl)",
        "(yl)",
        "(bl)",
        "(ml)",
        "(cl)",
        "(bgl)",
        "(fgl)",
        "<B>",
        "<I>",
        "<BI>",
        "<N>",
    ]
    .to_vec();

    for i in 0..maxlength {
        if i < artlen && i < conflen {
            let mut concat = art[i].clone() + &conf[i].clone();

            for j in 0..linetoinfo.len() {
                // concat = check_contains(&concat, linetoinfo[j].clone());
                concat = check_contains(&concat, linetoinfo[j].to_string());
            }
            print!("{}", concat);

            println!();
        } else if i < artlen {
            for k in 0..linetoinfo.len() {
                art[i] = check_contains(&art[i], linetoinfo[k].to_string());
            }
            println!("{:20}", art[i]);
        } else if i < conflen {
            for k in 0..linetoinfo.len() {
                conf[i] = check_contains(&conf[i], linetoinfo[k].to_string());
            }
            println!("{:20}  {}", " ", conf[i]);
        }
    }
}
