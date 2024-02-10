use std::ops::Deref;
use std::process::Command;
use std::string::String;
use std::{fs, path::Path};
use toml::map::Map;

use serde::{Deserialize, Serialize};

struct Config {
    pmlist: Map<String, PackageManager>,
}

impl Deref for Config {
    type Target = Map<String, PackageManager>;
    fn deref(&self) -> &Self::Target {
        &self.pmlist
    }
}

// impl IntoIterator for Config {
//     type Item = PackageManager;
//     type IntoIter = toml::Table;
//
//     fn into_iter(self) -> Self::IntoIter {}
// }

#[derive(Deserialize, Serialize)]
struct PackageManager {
    prefix: Option<String>,
    name: String,
    cliname: String,
    install_command: String,
    update: Option<String>,
    upgrade_command: String,
}

impl PackageManager {
    fn base_name(&self) -> String {
        let prefix = self.prefix.unwrap_or("".to_owned());
        format!("{prefix} {0}", self.cliname)
    }

    pub fn update(&self) -> Option<String> {
        if let None = self.update {
            return None;
        }

        let update = self.update.unwrap();
        Some(format!("{base} {update}", base = self.base_name()))
    }
    pub fn upgrade() {}
}

impl IntoIterator for PackageManager {
    fn into_iter(self) -> Self::IntoIter {}
}

fn main() {
    // let package_managers = vec!["apt", "flatpak", "cargo"];

    // use toml::Table;
    // let value = "foo = 'bar'".parse::<Table>().unwrap();

    let config_path = get_config_location();
    let config = parse_toml_file(config_path);

    let printed = toml::to_string_pretty(&config).unwrap();
    print!("{}", printed);

    let config: Config = toml::from_str(&printed).unwrap();

    for pm in config {
        Command::new("sh").arg("-c").arg(pm.prefix)
    }
}

fn get_config_location() -> &'static Path {
    return Path::new("./thing_config.toml");
}

fn parse_toml_file(path: &Path) -> toml::Table {
    let config_file = fs::read_to_string(path).expect("Could not find config file");
    let config_file = config_file.as_str();

    let config = toml::from_str::<toml::Table>(config_file).unwrap();
    return config;
}

#[cfg(test)]
mod tests {
    use crate::parse_toml_file;
    use std::path::Path;

    fn fake_config_path() -> &'static Path {
        return &Path::new("./tests/thing_config.toml");
    }

    #[test]
    fn parses_toml_file() {
        let _fake_config = parse_toml_file(fake_config_path());
    }
}
