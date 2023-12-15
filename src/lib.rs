use colored::{Colorize, ColoredString};
use yaml_rust::YamlLoader;
use std::fs;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct System {
    pub pretty_string: ColoredString,
    pub directory: String,
    pub games_are_directories: bool,
}

impl System {
    pub fn new(pretty_string: ColoredString, dir_name: &str, games_are_directories: bool) -> System {
        System {
            directory: String::from(dir_name),
            pretty_string,
            games_are_directories,
        }
    }
}

impl Hash for System {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.directory.hash(state);
        self.games_are_directories.hash(state);
    }
}

pub fn read_config(archive_root: &str) -> Vec<System> {
    let yaml_path = String::from(archive_root) + "/config.yaml";
    let read_to_string = fs::read_to_string(yaml_path).expect(
        "Fatal error: `config.yaml` not found in archive root."
    );

    let data = &YamlLoader::load_from_str(&read_to_string).expect(
        "Fatal error: `config.yaml` could not be parsed."
    )[0]["systems"];

    if data.is_badvalue() {
        println!("Fatal error: `config.yaml` does not contain a `systems` key.");
        std::process::exit(1);
    }

    let mut systems: Vec<System> = Vec::new();

    for system in data.as_hash().unwrap().values() {
        let display_name = system["display_name"].as_str().unwrap();
        let color = system["color"].as_vec().unwrap();
        let path = system["path"].as_str().unwrap();
        let games_are_directories = system["games_are_directories"].as_bool().unwrap();

        let nth_color = |n: usize| -> u8 {
            color.get(n).unwrap().as_i64().unwrap() as u8
        };

        let display_name = display_name.truecolor(
            nth_color(0),
            nth_color(1),
            nth_color(2)
        );

        systems.push(System::new(
            display_name,
            path,
            games_are_directories,
        ));
    }

    systems
}

#[cfg(test)]
mod tests {
    use yaml_rust::{YamlLoader, Yaml};

    const DEMO: &str = "
systems:
    wii:
        display_name: WII
        color: [0,255,0]
        path: wbfs
        games_are_directories: true
    gamecube:
        display_name: GCN
        color: [62,255,0]
        path: games
        games_are_directories: true
    ds:
        display_name: DS
        color: [0,255,55]
        path: ds
        games_are_directories: false
";

    #[test]
    fn parse_display_name() {
        let data = &YamlLoader::load_from_str(DEMO).unwrap()[0]["systems"];
        assert_eq!(data["wii"]["display_name"], Yaml::String("WII".to_string()));
    }

    #[test]
    fn parse_color() {
        let data = &YamlLoader::load_from_str(DEMO).unwrap()[0]["systems"];
        let color_vec = data["wii"]["color"].as_vec().unwrap();
        assert_eq!(color_vec.get(1).unwrap().as_i64().unwrap(), 255);
    }

    #[test]
    fn parse_path() {
        let data = &YamlLoader::load_from_str(DEMO).unwrap()[0]["systems"];
        assert_eq!(data["gamecube"]["path"], Yaml::String("games".to_string()));
        assert_eq!(data["ds"]["games_are_directories"], Yaml::Boolean(false));
    }

    #[test]
    fn parse_games_are_directories() {
        let data = &YamlLoader::load_from_str(DEMO).unwrap()[0]["systems"];
        assert_eq!(data["ds"]["games_are_directories"], Yaml::Boolean(false));
    }

}
