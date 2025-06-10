mod totp;

use std::{collections::HashMap, env, fs::OpenOptions, path::PathBuf};

use copypasta::{x11_clipboard::{Clipboard, X11ClipboardContext}, ClipboardContext, ClipboardProvider};
use notify_rust::{Notification, Timeout};
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
            //let mut ctx = X11ClipboardContext::<Clipboard>::new().expect("failed to get clipboard context");
            //ctx.set_contents(otp).expect("failed to set clipboard!");
            eprintln!("{otp}");
            Notification::new()
                .summary("LSOTP")
                .body(&format!("{otp}"))
                .timeout(Timeout::Never)
                .show().expect("failed to show notification");
        }
        Command::Add { profile, secret } => {
            cfg.add(profile, secret);
            cfg.save();
        }
    }

}
