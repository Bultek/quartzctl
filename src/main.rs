use clap::*;
use colored::*;
use libquartz::*;
use std::{fs,env,path, io::Read};
fn main() {
    enable_windows();
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
                .subcommand_required(true)
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
        .subcommand(
            Command::new("servers").about("List all servers")
        )
        .about("Libquartz based apps control utility")
        .get_matches();
        println!("{}", "Welcome to quartz control utility".green());
        let _debug = ArgMatches::contains_id(&_matches, "debug");
        if let Some(subc) = _matches.subcommand_matches("key"){
            if let Some(subc2) = subc.subcommand_matches("gen") {
                let keyname = subc2.get_one::<String>("keyname").unwrap();
                gen_key(&keyname);
            }
            if let Some(subc2) = subc.subcommand_matches("set") {
                let keyindex = subc2.get_one::<String>("keyindex").unwrap();
                set_key(&keyindex);
            }
            if let Some(_subc) = subc.subcommand_matches("list") {
                list_keys(&_debug);
            }
        }
        if let Some(_subc) = _matches.subcommand_matches("servers") {
            list();
        }
}

fn list() {
    let servers = get_servers();
    let mut i = 0;
    for server in servers.names {
        println!("{}{}{} - {} ({})","[".bright_yellow(), i.to_string().bright_yellow(), "]".bright_yellow(), server.trim().bright_green(),&servers.urls[i].trim());
        i += 1;
    }
}

fn get_servers() -> ServerData {
    create_config();
    #[allow(deprecated)]
    let home = env::home_dir().unwrap();
    let srvpath = path::Path::new(&home)
        .join(".config")
        .join("libquartz")
        .join("servers");
    if fs::metadata(&srvpath).is_err() {
        fs::create_dir_all(&srvpath).unwrap();
    }
    let mut servernames: Vec<String> = Vec::new();
    let mut serverurls: Vec<String> = Vec::new();
    for entry in fs::read_dir(&srvpath).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let filename = path.file_name().unwrap().to_str().unwrap();
            servernames.push(filename.to_string());
            let mut file = fs::File::open(path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            serverurls.push(contents.to_string());
        }
    }
    ServerData {
        names: servernames,
        urls: serverurls,
    }
}
struct ServerData {
    names: Vec<String>,
    urls: Vec<String>,
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
        println!("{}", "Key index out of range".red());
        std::process::exit(1);
    }
    allkeys[_index].to_string()
}

fn set_key(_keyindex: &str) {
    println!("{}", "Setting key".blue());
    println!("{}", "Saving key to file".blue());
    // write key to file
    let _kindex = _keyindex.parse::<usize>();
    let mut _index: usize = 0; 
    match _kindex {
        Ok(_kindex) => {
            _index = _kindex -1;
        }
        Err(_) => {
            panic!("Key index is not a number");
        }
    }
    
    let keydata = get_key_by_index(_index);
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
    let mut index = 1;
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
        println!("{}{}{} {} - {}", "[".bright_blue(), index, "]".bright_blue(), keyname.bright_yellow(), keydata.on_yellow());
        index += 1;
    }
}


#[cfg(windows)]
fn enable_windows() {    
    let _wincolorfix = colored::control::set_virtual_terminal(true);
}