use clap::{Parser, ValueEnum};
use regex::{Regex, RegexSet};
use std::collections::HashMap;
use std::fs;

use std::fs::File;
use std::io::BufRead;
use std::path::{Path, PathBuf};

fn main() {
    let cli = &Cli::parse();
    let mut map = read_sys_env();
    if let Some(env_file) = &cli.env_file {
        let env_map = read_env_file(&env_file);
        map.extend(env_map);
    }

    let default_var_types: &Vec<VarType> = &vec![VarType::LINUX];

    let arr_regex_types = match &cli.regex {
        Some(arr) => arr,
        None => default_var_types,
    };

    let is_include_mode = cli.exclude.is_none();
    let regex_set = get_regex_set(&cli);
    println!("{:?}", regex_set.patterns());

    let regex = get_regex(&arr_regex_types);

    for entry in walkdir::WalkDir::new(&cli.path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.path().is_file() {
            let file_name = entry.file_name().to_string_lossy();
            println!("file name : {}", file_name.to_owned());
            if regex_set.is_match(&file_name) {
                println!("match file name : {}", file_name);
                if is_include_mode {
                    handle_file(&entry, &regex, &map);
                }
            } else {
                println!("not match file name : {}", file_name);
                if !is_include_mode {
                    handle_file(&entry, &regex, &map);
                }
            }
        }
    }
}

fn get_regex_set(cli: &Cli) -> RegexSet {
    let empty = Vec::new();
    let all = &".*".to_owned();
    let include_arr = match &cli.include {
        Some(v) => v,
        None => &empty,
    };
    let exclude_arr = match &cli.exclude {
        Some(v) => v,
        None => &empty,
    };

    let mut file_regex_arr = Vec::new();
    file_regex_arr.extend(include_arr);
    file_regex_arr.extend(exclude_arr);

    if cli.include.is_none() && cli.exclude.is_none() {
        let mut tmp_vec = vec![all];
        file_regex_arr.append(&mut tmp_vec);
    }

    let regex_set = RegexSet::new(&file_regex_arr).unwrap();
    regex_set
}

fn handle_file(entry: &walkdir::DirEntry, regex: &String, map: &HashMap<String, String>) {
    let path = entry.path();
    let content = fs::read_to_string(path).unwrap();
    let text = replace_values(regex, content, map);
    fs::write(path, text).unwrap();
}

fn get_regex(arr_regex_types: &Vec<VarType>) -> String {
    let mut lst: Vec<&str> = Vec::new();
    for ele in arr_regex_types {
        let reg = match ele {
            VarType::LINUX => r"(\$(\w+))|(\$\{\s?(\w+)\s?\})",
            VarType::PYTHON => r"\{\{\s?(\w+)\s?\}\}",
            VarType::MYBATIS => r"(#(\w+))|(\#\{\s?(\w+)\s?\})",
            VarType::WINDOWS => r"(%(\w+)%)",
        };
        lst.push(reg);
    }
    let regex = lst.join("|");
    regex
}

fn replace_values(regex: &String, content: String, map: &HashMap<String, String>) -> String {
    let regex = Regex::new(&regex).unwrap();
    let ans = regex.replace_all(&content, |caps: &regex::Captures| {
        let key = find_var_name(caps);
        let orgin_key = caps.get(0).unwrap().as_str().to_owned();
        let ans = map.get(&key).unwrap_or(&orgin_key);
        ans.to_owned()
    });
    ans.to_string()
}

fn find_var_name(caps: &regex::Captures) -> String {
    let g0 = caps.get(0).unwrap();
    let orgin_key = g0.as_str();
    for g in caps.iter() {
        if let Some(m) = g {
            if !m.eq(&g0) {
                return m.as_str().to_owned();
            }
        }
    }
    return orgin_key.to_string();
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum VarType {
    /// $var and ${var}
    LINUX,
    /// {{var}}
    PYTHON,
    /// #var and #{var}
    MYBATIS,
    /// %var%
    WINDOWS,
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    path: PathBuf,

    /// default use linux type
    #[arg(short, long, value_enum)]
    regex: Option<Vec<VarType>>,

    #[arg(long, conflicts_with = "exclude")]
    include: Option<Vec<String>>,

    #[arg(long, conflicts_with = "include")]
    exclude: Option<Vec<String>>,

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
        if s.is_empty() {
            continue;
        }

        if let Some((k, v)) = s.split_once('=') {
            let key = k.trim().to_string();
            let value = v.trim().to_string();
            map.insert(key, value);
        }
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
fn replace_vars_test() {
    let mut map = HashMap::new();
    map.insert("name".to_owned(), "tom".to_owned());
    map.insert("age".to_owned(), "18".to_owned());
    map.insert("address".to_owned(), "homeless".to_owned());
    map.insert("phone".to_owned(), "123456".to_owned());
    map.insert("account".to_owned(), "0$".to_owned());

    println!("{:?}", map);

    let regex_str = get_regex(&vec![
        VarType::LINUX,
        VarType::PYTHON,
        VarType::WINDOWS,
        VarType::MYBATIS,
    ]);
    println!("{}", regex_str);
    let content = "name ${name}, age: $age , address: %address% , phone: #phone ${phone} , account: {{account}}, nothing: ${nothing}".to_owned();
    let text = replace_values(&regex_str, content, &map);
    let ans = "name tom, age: 18 , address: homeless , phone: 123456 123456 , account: 0$, nothing: ${nothing}";
    assert_eq!(ans, text);
}
