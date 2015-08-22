extern crate getopts;
extern crate handlebars;
extern crate rustc_serialize;

use std::fs::File;
use std::io::Write;
use std::fs;
use getopts::Options;
use std::env;
use std::process::Command;
use handlebars::Handlebars;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use std::collections::BTreeMap;
use std::io::Error;

pub struct CargoInfo {
    project_name: String,
}

impl CargoInfo {
    pub fn new(project_name: String) -> CargoInfo {
        CargoInfo { project_name: project_name }
    }
}

impl ToJson for CargoInfo {
  fn to_json(&self) -> Json {
    let mut m: BTreeMap<String, Json> = BTreeMap::new();
    m.insert("project_name".to_string(), self.project_name.to_json());
    m.to_json()
  }
}


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} init PROJECT_NAME\n       {} run", program, program);
    print!("{}", opts.usage(&brief));
}

fn copy_files(project_name: &str) -> Result<(),Error> {
    let home_dir = env::home_dir().unwrap();
    let current_dir = env::current_dir().unwrap();
    let root_dir: String = format!("{}/{}/",current_dir.display(),project_name);
    try!(fs::create_dir_all(&format!("{}/src",&root_dir)));
    //TODO: stop this, do recursively
    let new_config = &format!("{}/config.rs", &root_dir);
    try!(fs::copy(format!("{}/.bolts/default_config/config.rs",home_dir.display()), new_config));
    let new_index = &format!("{}/index.md", &root_dir);
    try!(fs::copy(format!("{}/.bolts/default_config/index.md",home_dir.display()), new_index));
    let new_layout = &format!("{}/default_layout.hbs", &root_dir);
    try!(fs::copy(format!("{}/.bolts/default_config/default_layout.hbs",home_dir.display()), new_layout));
    let new_main = &format!("{}/src/main.rs", &root_dir);
    try!(fs::copy(format!("{}/.bolts/default_config/src/main.rs",home_dir.display()), new_main));
    let mut processor: Handlebars = Handlebars::new();
    processor.register_template_string("Cargo.toml.default", String::from("[package]
name = \"{{{project_name}}}\"
version = \"0.0.0\"

[dependencies.bolts]
git = \"https://github.com/ryanr1230/bolts.git\"")).ok().unwrap();
    let cargo_info: CargoInfo = CargoInfo::new(String::from(project_name));
    let to_write = processor.render("Cargo.toml.default", &cargo_info);
    let mut cargo_file = try!(File::create(format!("{}{}",&root_dir, "/Cargo.toml")));
    cargo_file.write(to_write.unwrap().as_ref()).unwrap();
    return Ok(());
}

fn run_site_gen() -> Result<(), Error> {
    let compiling_status = Command::new("cargo").arg("build").status().unwrap();
    println!("Compiling exited with: {}", compiling_status);
    let running_status = Command::new("cargo").arg("run").status().unwrap();
    println!("Running exited with: {}", running_status);
    return Ok(());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let command: &str = &args[1].clone();
    match command {
        "init" => copy_files(&args[2].clone()).unwrap(),
        "run" => run_site_gen().unwrap(),
        _ => print_usage(&program, opts),
    }
}
