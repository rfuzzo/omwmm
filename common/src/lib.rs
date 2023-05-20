use std::{
    fs::File,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

use log::{error, info};

/// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Parses the omwcfg and returns the data directories and content files
pub fn parse_cfg(cfg_path: PathBuf) -> Option<(Vec<PathBuf>, Vec<String>)> {
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

        Some((data_dirs, plugin_names))
    } else {
        error!("Could not parse cfg file {}", cfg_path.display());
        None
    }
}
