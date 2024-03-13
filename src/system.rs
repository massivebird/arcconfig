use colored::ColoredString;

/// An abstraction over a game system.
///
/// # Properties
///
/// + `label`: An arbitrary system label for identifying this system.
/// + `pretty_string`: A `colored::ColoredString` for output purposes.
/// + `dir_name`: the directory in which this system's games are stored (relative to the
/// archive root).
/// + `games_are_directories`: are this system's games represented as directories?
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct System {
    pub label: String,
    pub pretty_string: ColoredString,
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
        pretty_string: ColoredString,
        dir_name: &str,
        games_are_directories: bool,
    ) -> Self {
        Self {
            label: String::from(label),
            directory: String::from(dir_name),
            pretty_string,
            games_are_directories,
        }
    }
}

impl std::hash::Hash for System {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.directory.hash(state);
        self.games_are_directories.hash(state);
    }
}
