use pm;
use std::process::Command;
mod config;

fn main() {
    // let package_managers = vec!["apt", "flatpak", "cargo"];

    // let value = "foo = 'bar'".parse::<Table>().unwrap();

    // let config_path = config::get_config_location();
    let config = config::get_config();

    let printed = toml::to_string_pretty(&config).unwrap();
    print!("{}", printed);

    let config = toml::from_str(&printed).unwrap();

    for pm in config {
        Command::new("sh").arg("-c").arg(pm.prefix);
    }
}
