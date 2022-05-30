use clap::*;
use colored::*;
use libquartz::*;
use std::{fs,env,path};
fn main() {
    let _matches = Command::new("Quartz control utility")
        .subcommand_required(true)
        .version("0.1")
        .author("Bultek. <help@bultek.com.ua>")
        .arg(
            Arg::new("debug")
                .long("debug")
                .help("Enable debug output")
                .takes_value(false),
        )
        .subcommand(
            Command::new("key")
                .about("Key interaction")
                .subcommand(
                    Command::new("gen").about("Generates a qkey").arg(
                        Arg::new("keyname")
                            .long("name")
                            .short('n')
                            .help("Name of the key")
                            .required(true)
                            .value_name("name"),
                    ),
                )
                .subcommand(
                    Command::new("set").about("Set a specific key").arg(
                        Arg::new("keyindex")
                            .long("index")
                            .short('i')
                            .help("Index of the key")
                            .required(true)
                            .value_name("index"),
                    ),
                )
                .subcommand(Command::new("list").about("List all keys")),
        )
        .about("Libquartz based apps control utility")
        .get_matches();
        println!("{}", "Welcome to quartz control utility".green());
        let _debug = _matches.is_present("debug");
        if let Some(subc) = _matches.subcommand_matches("key"){
            if let Some(subc2) = subc.subcommand_matches("gen") {
                let keyname = subc2.value_of("keyname").unwrap().to_string();
                gen_key(&keyname);
            }
            if let Some(subc2) = subc.subcommand_matches("set") {
                let keyindex = subc2.value_of("keyindex").unwrap().to_string();
                set_key(&keyindex);
            }
            if let Some(_subc) = subc.subcommand_matches("list") {
                list_keys(&_debug);
            }
        }
}
fn gen_key(_keyname: &String) {
    println!("{}", "Generating key".blue());
    let _key = keytools::gen_key();
    println!("{}", "Key generated".green());
    println!("{}", "Saving key to file".blue());
    // write key to file
    create_config();
    #[allow(deprecated)]
    let home = env::home_dir().unwrap();    
    let cfgpath = path::Path::new(&home).join(".config").join("libquartz").join("keys");
    let out = fs::write(cfgpath.join(_keyname), _key);
    match out {
        Ok(_) => { println!("{}", "Key saved".green()) }
        Err(_) => { println!("{}", "Key saving failed".red()) ; std::process::exit(1);}
    }
}

fn create_config() {
    #[allow(deprecated)]
    let home = env::home_dir().unwrap();
    // Join paths
    let cfgpath = path::Path::new(&home).join(".config");
    // Check if the path exists
    if !cfgpath.exists() {
        // Create the path
        fs::create_dir(&cfgpath).expect("Could not create config directory");
    }
    // Join paths
    let libquartzpath = cfgpath.join("libquartz");
    // Check if the path exists
    if !libquartzpath.exists() {
        // Create the path
        fs::create_dir(&libquartzpath).expect("Could not create libquartz directory");
    }
    // Join paths
    let keyspath = libquartzpath.join("keys");
    // Check if the path exists
    if !keyspath.exists() {
        // Create the path
        fs::create_dir(&keyspath).expect("Could not create keys directory");
    }
}

fn get_key_by_index(_index: usize) -> String {
    create_config();
    // Get the home dir
    #[allow(deprecated)]
    let home = env::home_dir().unwrap();    
    let cfgpath = path::Path::new(&home).join(".config").join("libquartz").join("keys");
    // List all files in the keys directory
    let keys = fs::read_dir(&cfgpath).expect("Could not read keys directory");
    // Iterate over the files
    let mut allkeys: Vec<String> = Vec::new();
    for key in keys {
        // Get the file name
        let keyy = key.unwrap().path();
        let keydata = fs::read_to_string(&keyy).expect("Error to read key");
        // Print the file name
        allkeys.push(keydata);
    }
    if allkeys.len() <= _index {
        return "".to_string();
    }
    return allkeys[_index].to_string();
}

fn set_key(_keyindex: &String) {
    println!("{}", "Setting key".blue());
    println!("{}", "Saving key to file".blue());
    // write key to file
    let _kindex = _keyindex.parse::<usize>().unwrap() - 1;
    let keydata = get_key_by_index(_kindex);
    create_config();
    #[allow(deprecated)]
    let home = env::home_dir().unwrap();    
    let cfgpath = path::Path::new(&home).join(".config").join("libquartz");
    let out = fs::write(cfgpath.join("defaultkey"), keydata);
    match out {
        Ok(_) => { println!("{}", "Key saved".green()) }
        Err(_) => { println!("{}", "Key saving failed".red()) ; std::process::exit(1);}
    }
}
fn list_keys(_debug: &bool) {
    create_config();
    // Get the home dir
    #[allow(deprecated)]
    let home = env::home_dir().unwrap();    
    let cfgpath = path::Path::new(&home).join(".config").join("libquartz").join("keys");
    // List all files in the keys directory
    let keys = fs::read_dir(&cfgpath).expect("Could not read keys directory");
    // Iterate over the files
    for key in keys {
        // Get the file name
        let keyy = key.unwrap().path();
        let keyname = keyy.file_name().unwrap().to_str().unwrap();
        let keydata = fs::read_to_string(&keyy).expect("Error to read key");
        // Print the file name
        println!("{} - {}", keyname.bright_yellow(), keydata.on_yellow());
    }
}