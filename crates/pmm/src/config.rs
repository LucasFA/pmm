// use pm::PackageManager;
// use std::ops::Deref;
// use std::string::String;
use config::Config;
use std::{fs, path::Path};
use directories::ProjectDirs;
use toml;

pub type ConfigTable = toml::Table;

// pub(crate) struct Config {
//     pmlist: Map<String, PackageManager>,
// }
//
// impl Deref for Config {
//     type Target = Map<String, PackageManager>;
//     fn deref(&self) -> &Self::Target {
//         &self.pmlist
//     }
// }
//
// impl IntoIterator for Config {
//     type Item = PackageManager;
//     type IntoIter = toml::Table;
//
//     fn into_iter(self) -> Self::IntoIter {}
// }

pub fn get_config_location() -> &'static Path {
    // todo!();
    return Path::new("./thing_config.toml");
}

pub fn get_config() -> ConfigTable {
    parse_config_file(get_config_location())
}

fn parse_config_file(path: &Path) -> toml::Table {
    let config_file = fs::read_to_string(path).expect("Could not find config file");
    let config_file = config_file.as_str();

    let config = toml::from_str::<toml::Table>(config_file).unwrap();
    return config;
}

fn new_config() {
    let env = std::env::var("XDG_CONFIG_HOME");
    let config_directory =
    let settings = Config::builder()
        .add_source(File::from(Path::new(env).join("pmm").join("pmm.toml")))
        .add_source(File::from);
}

#[cfg(test)]
mod tests {
    use crate::config::*;
    use std::path::Path;

    fn fake_config_path() -> &'static Path {
        return &Path::new("./tests/thing_config.toml");
    }

    #[test]
    fn parses_toml_file() {
        let _fake_config = parse_config_file(fake_config_path());
    }
}
