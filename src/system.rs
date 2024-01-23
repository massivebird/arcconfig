use colored::ColoredString;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct System {
    pub label: String,
    pub pretty_string: ColoredString,
    pub directory: String,
    pub games_are_directories: bool,
}

impl System {
    pub fn new(label: &str, pretty_string: ColoredString, dir_name: &str, games_are_directories: bool) -> System {
        System {
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
