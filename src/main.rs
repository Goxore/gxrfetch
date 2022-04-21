use std::env;
use std::fs::{self,DirBuilder};
use std::io::{self, Result};
use regex::Regex;
use std::process::Command;
extern crate termion;
use systemstat::{saturating_sub_bytes, Platform, System};
use termion::{
    color::{self, *},
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
        "[env]" => matchvalue(nixinfo::environment()),
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
            Ok(battery) =>  (battery.remaining_capacity * 100.0).floor().to_string(),
            Err(x) => format!("err, {}", x),
        },
        "[mem]" => 
        match sys.memory() {
            Ok(mem) => format!( "{} / {}", saturating_sub_bytes(mem.total, mem.free), mem.total),
            Err(x) => format!("err, {}", x),
        },
        "[col]" => {
            format!("{}  {}  {}  {}  {}  {}  {}  {}  {}",Bg(Black),Bg(Red),Bg(Green),Bg(Yellow),Bg(Blue),Bg(Magenta),Bg(Cyan),Bg(White),Bg(Black))
        },
        "[col2]" => {
            format!("{}  {}  {}  {}  {}  {}  {}  {}  {}",Bg(LightBlack),Bg(LightRed),Bg(LightGreen),Bg(LightYellow),Bg(LightBlue),Bg(LightMagenta),Bg(LightCyan),Bg(LightWhite),Bg(Black))
        },

        "(r)" => Fg(Red).to_string(),
        "(g)" => Fg(Green).to_string(),
        "(y)" => Fg(Yellow).to_string(),
        "(b)" => Fg(Blue).to_string(),
        "(m)" => Fg(Magenta).to_string(),
        "(c)" => Fg(Cyan).to_string(),

        "(bg)" => Fg(Black).to_string(),
        "(fg)" => Fg(White).to_string(),
        "(rl)" => Fg(LightRed).to_string(),
        "(gl)" => Fg(LightGreen).to_string(),
        "(yl)" => Fg(LightYellow).to_string(),
        "(bl)" => Fg(LightBlue).to_string(),
        "(ml)" => Fg(LightMagenta).to_string(),
        "(cl)" => Fg(LightCyan).to_string(),
        "(bgl)" => Fg(LightBlack).to_string(),
        "(fgl)" => Fg(LightWhite).to_string(),

        "((r))" => Bg(Red).to_string(),
        "((g))" => Bg(Green).to_string(),
        "((y))" => Bg(Yellow).to_string(),
        "((b))" => Bg(Blue).to_string(),
        "((m))" => Bg(Magenta).to_string(),
        "((c))" => Bg(Cyan).to_string(),

        "((bg))" => Bg(Black).to_string(),
        "((fg))" => Bg(White).to_string(),
        "((rl))" => Bg(LightRed).to_string(),
        "((gl))" => Bg(LightGreen).to_string(),
        "((yl))" => Bg(LightYellow).to_string(),
        "((bl))" => Bg(LightBlue).to_string(),
        "((ml))" => Bg(LightMagenta).to_string(),
        "((cl))" => Bg(LightCyan).to_string(),
        "((bgl))" => Bg(LightBlack).to_string(),
        "((fgl))" => Bg(LightWhite).to_string(),

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
        if &contains == "[["
        {
            let re = Regex::new(r"\[\[.*\]\]").unwrap();
            let m = re.find(line).unwrap().as_str();
            let m2 = &m[2..m.len()-2];
            let cmnd = Command::new("bash")
                    .arg(format!("-c"))
                    .arg(m2)
                    .output()
                    .expect("failed to execute process");
            newline.replace(m, std::str::from_utf8(&cmnd.stdout).unwrap().trim())
        }else{
            newline.replace(&contains, &get_specific(&contains))
        }
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
"  (m)<BI>[user](fgl)@(m)[host]
  (fg)----------------
  (b)<B> (fgl): <N>[name]
  (b)<B>﬙ (fgl): <N>[cpu]
  (b)<B> (fgl): <N>[cores] cores
  (b)<B> (fgl): <N>[bat]%
  (b)<B> (fgl): <N>[mem]
  (b)<B> (fgl): <N>[uptime]
  (b)<B> (fgl): <N>[distro]
  (b)<B> (fgl): <N>[kernel]
  (b)<B> (fgl): <N>[shell]
  (b)<B> (fgl): <N>[term]

  (b)[col]
  (b)[col2]
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
        "[env]",  
        "[col]",  
        "[col2]",  
        "[[",  
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
                concat = check_contains(&concat, linetoinfo[j].to_string());
            }
            print!("{}{}", concat,style::Reset.to_string());

            println!();
        } else if i < artlen {
            for k in 0..linetoinfo.len() {
                art[i] = check_contains(&art[i], linetoinfo[k].to_string());
            }
            println!("{:20}{}", art[i],style::Reset.to_string());
        } else if i < conflen {
            for k in 0..linetoinfo.len() {
                conf[i] = check_contains(&conf[i], linetoinfo[k].to_string());
            }
            println!("{:20}  {}{}", " ", conf[i],style::Reset.to_string());
        }
    }
}
