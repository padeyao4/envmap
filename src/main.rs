use clap::Parser;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

use std::fs::File;
use std::io::BufRead;
use std::path::{Path, PathBuf};

fn main() {
    let cli = Cli::parse();
    let input = cli.input;
    let output = cli.output;
    let mut map = read_sys_env();
    if let Some(env_file) = cli.env_file {
        let env_map = read_env_file(&env_file);
        map.extend(env_map);
    }

    let regex = match cli.regex {
        Some(reg) => reg,
        None => r"(\$(\w+))|(\$\{\w+\})".to_owned(),
    };

    let regex = Regex::new(&regex).unwrap();

    let content = fs::read_to_string(input).unwrap();

    let ans = regex.replace_all(&content, |caps: &regex::Captures| {
        let group = caps.get(0);
        let var_name = group.unwrap().as_str();
        let var_name = var_name.strip_prefix("$").unwrap_or(var_name);        
        let var_name = var_name.strip_prefix("{").unwrap_or(var_name);        
        let var_name = var_name.strip_suffix("}").unwrap_or(var_name);        

        println!("{}", var_name);
        let var_value = map.get(var_name);
        match var_value {
            Some(value) => value.to_owned(),
            None => "".to_owned(),
        }
    });

    fs::write(output, ans.into_owned()).unwrap();
    // regex.replace_all(text, rep)
}

// // 定义一个函数，接受一个字符串引用，并返回一个Result<String>
// fn replace_env_vars(text: &str) -> Result<String, env::VarError> {
//     // 定义一个正则表达式，匹配以$开头的环境变量名
//     let re = Regex::new(r"\$(\w+)").unwrap();

//     // 使用闭包作为替换函数，获取环境变量的值并返回
//     let result = re.replace_all(text, |caps: &regex::Captures| {
//         if let Some(var_name) = caps.get(1) {
//             if let Ok(var_value) = env::var(var_name.as_str()) {
//                 return var_value;
//             }
//         }
//         caps[0].to_string()
//     });

//     Ok(result.to_string())
// }

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    input: PathBuf,

    output: PathBuf,

    /// default '${}' will be replace
    #[arg(short, long)]
    regex: Option<String>,

    /// env_file contain key=value
    #[arg(short, long)]
    env_file: Option<PathBuf>,
}

fn read_env_file(path: &Path) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let file = File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let s = line.unwrap();
        let (k, v) = s.split_once('=').unwrap();
        let key = k.trim().to_string();
        let value = v.trim().to_string();
        map.insert(key, value);
    }
    return map;
}

fn read_sys_env() -> HashMap<String, String> {
    let mut map = HashMap::new();
    let vars = std::env::vars();
    for var in vars {
        map.insert(var.0, var.1);
    }
    return map;
}

#[test]
fn test_env() {
    read_sys_env();
}
