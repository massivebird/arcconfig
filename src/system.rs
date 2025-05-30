use colored::Colorize;

/// An abstraction over a game system. Built from the configuration file in the archive root.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct System {
    pub label: String,
    pub display_name: String,
    pub rgb: [u8; 3],
    pub directory: String,
    pub games_are_directories: bool,
}

impl System {
    /// Creates a single `System` instance.
    ///
    /// This is a manual alternative to `read_config`.
    #[must_use]
    pub fn new(
        label: &str,
        display_name: &str,
        rgb: [u8; 3],
        dir_name: &str,
        games_are_directories: bool,
    ) -> Self {
        Self {
            label: String::from(label),
            display_name: String::from(display_name),
            directory: String::from(dir_name),
            rgb,
            games_are_directories,
        }
    }
}

impl std::fmt::Display for System {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rgb = self.rgb;

        write!(f, "{}", self.display_name.truecolor(rgb[0], rgb[1], rgb[2]))
    }
}

impl std::hash::Hash for System {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.directory.hash(state);
        self.games_are_directories.hash(state);
    }
}
