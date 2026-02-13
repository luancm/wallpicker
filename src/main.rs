use std::fs::remove_file;
use std::io::{ErrorKind, Write};
use std::process::{Command, Stdio};
use std::{collections::HashMap, fs, path::Path};

const PROGRAM_NAME: &str = "Wallpicker";
const SUPPORTED_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "bmp", "gif"];

fn find_wallpapers(dir_path: &Path) -> HashMap<String, String> {
    // read only image files
    fs::read_dir(dir_path)
        .expect("Failed to read wallpapers directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let extension = path.extension()?.to_str()?.to_lowercase();

            if !SUPPORTED_EXTENSIONS.contains(&extension.as_str()) {
                return None;
            }

            let filename = path.file_name()?.to_str()?.to_string();
            let full_path = path.to_str()?.to_string();
            Some((filename, full_path))
        })
        .collect()
}

fn notify(message: &str) {
    Command::new("notify-send")
        .arg(PROGRAM_NAME)
        .arg(message)
        .output()
        .expect("Failed to send notification");
}

/// Opens Walker in dmenu mode to select a wallpaper.
/// Returns the full path of the selected wallpaper,
/// or None if no selection was made or an error occurred.
///
/// # Arguments
/// * `wallpapers` - A HashMap mapping wallpaper filenames to their full paths.
///
/// # Returns
/// * `Option<String>` - The full path of the selected wallpaper
fn select_wallpaper(wallpapers: &HashMap<String, String>) -> Option<String> {
    println!("Opening Walker in dmenu mode to select a wallpaper...");
    let mut walker_process = Command::new("walker")
        .args(["--dmenu", "--placeholder", "Select a wallpaper"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn walker");

    {
        let stdin = walker_process.stdin.as_mut().expect("Failed to open stdin");
        wallpapers
            .keys()
            .try_for_each(|filename| writeln!(stdin, "{}", filename))
            .ok()?
    } // stdin drops here, closing the pipe

    let output = walker_process
        .wait_with_output()
        .expect("Failed to read walker output");

    if !output.status.success() {
        eprintln!("Walker exited with error {}", output.status);
        return None;
    }

    let selected_filename = String::from_utf8_lossy(&output.stdout).trim().to_string();
    wallpapers.get(&selected_filename).cloned()
}

fn set_wallpaper(wallpaper_path: &str) {
    let swww_process = Command::new("swww")
        .args(["img", wallpaper_path])
        .args(["--transition-type", "wipe"])
        .args(["--transition-fps", "60"])
        .output()
        .expect("Failed to set wallpaper with swww");

    if !swww_process.status.success() {
        eprintln!("swww exited with error {}", swww_process.status);
        return;
    }

    // Link the selected wallpaper file to home ./cache/current_wallpaper
    let wallpaper_cache_path = dirs::home_dir()
        .expect("Failed to get home directory")
        .join(".cache/current_wallpaper");

    if let Err(e) = remove_file(&wallpaper_cache_path) {
        if e.kind() != ErrorKind::NotFound {
            eprintln!("Failed to remove existing wallpaper cache file: {}", e);
            return;
        }
    }

    if let Err(e) = std::os::unix::fs::symlink(wallpaper_path, &wallpaper_cache_path) {
        eprintln!("Failed to create symlink for wallpaper cache: {}", e);
    }
}

fn main() {
    let wallpapers_path = dirs::home_dir()
        .expect("Failed to get home directory")
        .join("pictures/wallpapers");

    let wallpapers = find_wallpapers(&wallpapers_path);

    // use notify-send if no wallpapers found
    if wallpapers.is_empty() {
        let msg = format!("No wallpapers found in {}", wallpapers_path.display());
        println!("WARN: {}", msg);
        notify(&msg);
        return;
    }

    let selected_file = match select_wallpaper(&wallpapers) {
        Some(file) => file,
        None => {
            println!("No wallpaper selected.");
            return;
        }
    };

    println!("Selected wallpaper: {}", selected_file);

    set_wallpaper(&selected_file);

    notify(&format!("Wallpaper set to {}", selected_file));
}
