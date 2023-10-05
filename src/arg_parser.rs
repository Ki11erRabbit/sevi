use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use clap::Parser;


pub static IGNORE_USER_SETTINGS: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Parser)]
pub struct Args {
    /// The filepath to open
    pub file: Option<String>,

    #[arg(short, long)]
    pub generate_default_settings: bool,
    #[arg(short, long)]
    pub ignore_user_settings: bool,
}



impl Args {
    pub fn perform_commands(&self) {
        let mut performed_command = false;
        if self.generate_default_settings {
            println!("Generating Editor Settings");
            crate::models::settings::editor_settings::EditorSettings::create_default_config_file().expect("Could not create default config file");
            println!("Generating Keybindings");
            crate::models::settings::mode_keybindings::ModeKeybindings::create_default_config_file().expect("Could not create default config file");
            println!("Generating Colors");
            crate::models::settings::colors::EditorColors::create_default_config_file().expect("Could not create default config file");
            performed_command = true;
        }
        if performed_command {
            std::process::exit(0);
        }
        if self.ignore_user_settings {
            IGNORE_USER_SETTINGS.store(true,std::sync::atomic::Ordering::Relaxed);
        }
    }

    pub fn get_path(&self) -> Option<PathBuf> {
        match &self.file {
            Some(path) => {
                Some(PathBuf::from(path))
            },
            None => {
                None
            }
        }
    }
}