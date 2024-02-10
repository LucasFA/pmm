use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
struct Command {
    command: String,
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.command)
    }
}

#[derive(Deserialize, Serialize)]
pub struct PackageManager {
    prefix: Option<Command>,
    name: String,
    cliname: String,
    install: Option<Command>,
    update: Option<Command>,
    upgrade: Command,
}

impl PackageManager {
    fn base_name(&self) -> String {
        // self.prefix.or()
        let prefix = self.prefix.clone().unwrap_or_default();
        format!("{prefix} {0}", self.cliname)
    }

    pub fn update(&self) -> Option<String> {
        if let Some(update) = &self.update {
            return Some(format!("{base} {update}", base = self.base_name()));
        }

        None
    }
    pub fn upgrade() {}
}

impl TryFrom<toml::Table> for PackageManager {
    fn try_from(value: toml::Table) -> Result<Self, Self::Error> {}
}

// impl IntoIterator for PackageManager {
//     fn into_iter(self) -> Self::IntoIter {}
// }

#[cfg(test)]
mod tests {
    use super::*;
}
