extern crate getopts;
extern crate rumblebars;
extern crate rustc_serialize;
#[allow(dead_code)] #[macro_use] mod common;

use std::path::Path;
use std::thread;
use std::fs::File;
use std::io::Write;
use std::fs;
use getopts::Options;
use std::env;
use std::process::Command;
use std::path::Display;
use std::process::ExitStatus;
use common::SiteGenResult;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} init PROJECT_NAME\n       {} run\n       {} compile\n       {} update",
        program, program, program, program);
    print!("{}", opts.usage(&brief));
}

fn copy_default_config_file(filename: &str, home_dir: &Display, root_dir: &str) -> SiteGenResult<()> {
    let new_config = &format!("{}/{}", root_dir, filename);
    try!(fs::copy(format!("{}/.bolts/default_config/{}",home_dir, filename), new_config));
    return Ok(());
}

fn copy_files(project_name: &str) -> SiteGenResult<()> {
    let home_dir = env::home_dir().unwrap();
    let current_dir = env::current_dir().unwrap();
    let root_dir: String = format!("{}/{}/",current_dir.display(),project_name);
    try!(fs::create_dir_all(&format!("{}/config/src",&root_dir)));
    //TODO: stop this, do recursively
    let home_dir_display: Display = home_dir.display();
    try!(copy_default_config_file("config/config.rs", &home_dir_display, &root_dir));
    try!(copy_default_config_file("index.md", &home_dir_display, &root_dir));
    try!(copy_default_config_file(".gitignore", &home_dir_display, &root_dir));
    try!(copy_default_config_file("default_layout.hbs", &home_dir_display, &root_dir));
    try!(copy_default_config_file("header.hbs", &home_dir_display, &root_dir));
    try!(copy_default_config_file("footer.hbs", &home_dir_display, &root_dir));
    try!(copy_default_config_file("config/src/main.rs", &home_dir_display, &root_dir));

    let cargo_info: String = String::from(format!("{}{}{}","[package]
name = \"",&project_name,"\"
version = \"0.0.0\"

[dependencies.bolts]
git = \"https://github.com/ryanr1230/bolts.git\""));

    let mut cargo_file = try!(File::create(format!("{}{}",&root_dir, "/config/Cargo.toml")));
    cargo_file.write(cargo_info.as_ref()).unwrap();
    return Ok(());
}

fn cd_into(path: &Path) -> SiteGenResult<()> {
    let mut current_dir = env::current_dir().unwrap().clone();
    current_dir.push(path);
    try!(env::set_current_dir(current_dir.as_path()));
    Ok(())
}

fn update() -> SiteGenResult<()> {
    try!(cd_into(Path::new("config")));
    let updating_status: ExitStatus = Command::new("cargo").arg("update").arg("-p").arg("bolts").status().unwrap();
    if !updating_status.success() {
        panic!("Couldn't update bolts for configuration runner");
    }
    try!(cd_into(Path::new("..")));
    Ok(())
}

fn compile() -> SiteGenResult<()> {
    try!(cd_into(Path::new("config")));
    let compiling_status: ExitStatus = Command::new("cargo").arg("build").status().unwrap();
    if !compiling_status.success() {
        panic!("Couldn't compile configuration runner");
    }
    try!(cd_into(Path::new("..")));
    Ok(())
}

#[allow(unreachable_code)]
fn run_site_gen() -> SiteGenResult<()> {
    try!(compile());
    let current_dir_buf = env::current_dir().unwrap();
    let project_name = current_dir_buf.file_name().unwrap().to_str().unwrap();
    loop {
        let running_status: ExitStatus =
            Command::new(format!("{}/config/target/debug/{}", current_dir_buf.display(), project_name))
                .status().unwrap();
        if !running_status.success() {
            println!("Running exited with: {}", running_status);
        } else {
            println!("Generated site");
        }
        thread::sleep_ms(4000);
    }
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
        "compile" => compile().unwrap(),
        "update" => update().unwrap(),
        _ => print_usage(&program, opts),
    }
}
