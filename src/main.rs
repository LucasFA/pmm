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

    let config_path = get_config_location();
    let config = parse_toml_file(config_path);

    let printed = toml::to_string_pretty(&config).unwrap();
    print!("{}", printed)
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
