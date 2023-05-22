use std::{
    fs::{self, File},
    io::{self, BufRead},
    path::{Path, PathBuf},
};

use log::{error, info};

pub struct ConfigInfo {
    pub data: Vec<PathBuf>,
    pub plugins: Vec<String>,
}

/// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Returns the default openmw.cfg path if it exists, and None if not
///
/// # Panics
///
/// Panics if Home dir is not found in the OS
pub fn get_openmwcfg() -> Option<PathBuf> {
    let os_str = std::env::consts::OS;
    match os_str {
        "linux" => {
            // default cfg for linux is at $HOME/.config/openmw
            let preference_dir = dirs::config_dir().unwrap();
            let cfg = preference_dir.join("openmw.cfg");
            if cfg.exists() {
                Some(cfg)
            } else {
                None
            }
        }
        "macos" => {
            // default cfg for mac is at /Users/Username/Library/Preferences/openmw
            let preference_dir = dirs::preference_dir().unwrap();
            let cfg = preference_dir.join("openmw").join("openmw.cfg");
            if cfg.exists() {
                Some(cfg)
            } else {
                None
            }
        }
        "windows" => {
            // default cfg for windows is at C:\Users\Username\Documents\my games\openmw
            let preference_dir = dirs::document_dir().unwrap();
            let cfg = preference_dir
                .join("my games")
                .join("openmw")
                .join("openmw.cfg");
            if cfg.exists() {
                Some(cfg)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Get all plugins (esp, omwaddon, omwscripts) in a folder
pub fn get_plugins_in_folder<P>(path: &P, use_omw_plugins: bool) -> Vec<PathBuf>
where
    P: AsRef<Path>,
{
    // get all plugins
    let mut results: Vec<PathBuf> = vec![];
    if let Ok(plugins) = fs::read_dir(path) {
        plugins.for_each(|p| {
            if let Ok(file) = p {
                let file_path = file.path();
                if file_path.is_file() {
                    if let Some(ext_os) = file_path.extension() {
                        let ext = ext_os.to_ascii_lowercase();
                        if ext == "esm"
                            || ext == "esp"
                            || (use_omw_plugins && ext == "omwaddon")
                            || (use_omw_plugins && ext == "omwscripts")
                        {
                            results.push(file_path);
                        }
                    }
                }
            }
        });
    }
    results
}

/// Parses the omwcfg and returns the data directories and content files
pub fn parse_cfg(cfg_path: PathBuf) -> Option<ConfigInfo> {
    let mut data_dirs: Vec<PathBuf> = vec![];
    let mut plugin_names: Vec<String> = vec![];

    info!("Parsing cfg {} ...", cfg_path.display());
    if let Ok(lines) = read_lines(&cfg_path) {
        for line in lines.flatten() {
            // parse each line
            if let Some(data_dir) = line.strip_prefix("data=") {
                // we found a data folder
                // add it to the folder list
                // we later copy all filtered plugins from that folder to the output_path
                let trimmed = data_dir.replace('"', "");
                let path = Path::new(trimmed.as_str()).to_path_buf();

                data_dirs.push(path);
            }

            if let Some(name) = line.strip_prefix("content=") {
                // we found a plugin name
                // add it to the plugin list
                // we filter later with that
                plugin_names.push(name.to_owned())
            }
        }

        Some(ConfigInfo {
            data: data_dirs,
            plugins: plugin_names,
        })
    } else {
        error!("Could not parse cfg file {}", cfg_path.display());
        None
    }
}
