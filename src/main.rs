mod totp;

use std::{collections::HashMap, env, fs::OpenOptions, path::PathBuf, time::Duration};

use qrism::reader::detect_qr;
use arboard::Clipboard;
use serde::{Serialize, Deserialize};
use clap::{Parser, Subcommand};
use totp::sane_totp;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command
}

#[derive(Subcommand)]
enum Command {
    Get {
        profile: String
    },
    Add {
        profile: String,
        secret: String
    },
    Import {
        profile: String,
        path: PathBuf,
    }
}

#[derive(Serialize, Deserialize)]
struct Config {
    profiles: HashMap<String, String>
}

impl Config {
    pub fn new() -> Self {
        Self { profiles: HashMap::new() }
    }

    fn get_path() -> PathBuf {
        let home = env::var("HOME").expect("home var unset!");
        PathBuf::from(home)
            .join(".lsotp.yaml")
    }

    pub fn load_or_create() -> Self {
        let p = Self::get_path();
        if !p.exists() {
            return Self::new();
        }

        let f = OpenOptions::new().read(true).open(Self::get_path()).expect("could not open config file!");
        serde_yaml::from_reader(f).expect("could not parse config file!")
    }

    pub fn add(&mut self, profile: String, secret: String) {
        self.profiles.insert(profile, secret);
    }

    pub fn import(&mut self, profile: String, qr_path: PathBuf) {
        if self.profiles.contains_key(&profile) {
            eprintln!("Profile with name '{profile}' already exists; not overwriting.");
            return;
        }

        let img = image::open(qr_path).expect("could not read provided image!");
        let mut res = detect_qr(&img);
        let symbol = res.symbols().first_mut().expect("could not read any data from QR");
        let (_, url) = symbol.decode().expect("found nothing in image");
        let (_, query) = url.split_once('?').expect("message is not in correct format");
        let mut secret = String::new();
        for kv in query.split('&') {
            if let Some((k, v)) = kv.split_once('=') {
                if k == "secret" {
                    secret = v.into();
                }
            }
        }
        println!("Added new profile '{profile}' successfully!");
        self.profiles.insert(profile, secret);
    }

    pub fn save(&self) {
        let p = Self::get_path();
        let f = OpenOptions::new().write(true).truncate(true).create(true).open(p).expect("failed to open config file for writing!");
        serde_yaml::to_writer(f, self).expect("failed to encode config file!");
    }

    pub fn get(&self, profile: String) -> String {
        let secret = self.profiles.get(&profile).cloned().expect("profile not found!");
        sane_totp(secret)
    }
}

fn main() {
    let mut cfg = Config::load_or_create();
    let args = Args::parse();
    match args.command {
        Command::Get { profile } => {
            let otp = cfg.get(profile);
            if atty::is(atty::Stream::Stdout) {
                if let Ok(mut clip) = Clipboard::new() {
                    clip.set_text(otp).expect("failed to set clipboard!");
                    std::thread::sleep(Duration::from_millis(500));
                    println!("OTP is in clipboard");
                }
                else {
                    println!("{otp}");
                }
            } else {
              print!("{otp}");
            }
        },
        Command::Add { profile, secret } => {
            cfg.add(profile, secret);
            cfg.save();
        },
        Command::Import { profile, path } => {
            cfg.import(profile, path);
            cfg.save();
        }
    }

}
