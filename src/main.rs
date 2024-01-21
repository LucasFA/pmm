use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct PackageManager {
    prefix: String,
    name: String,
    install_command: String,
    update_command: String,
    upgrade_command: String,
}

fn main() {
    // let package_managers = vec!["apt", "flatpak", "cargo"];

    // use toml::Table;
    // let value = "foo = 'bar'".parse::<Table>().unwrap();

    let config_path = Path::new("./thing_config.toml");

    let config = get_config(config_path);

    let printed = toml::to_string_pretty(&config).unwrap();
    print!("{}", printed)
}

fn get_config(path: &Path) -> toml::Table {
    let config_file = fs::read_to_string(path).expect("Could not find config file");
    let config_file = config_file.as_str();

    let config = toml::from_str::<toml::Table>(config_file).unwrap();
    return config;
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_toml_string() {}
}
