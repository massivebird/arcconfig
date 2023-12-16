# arcconfig

> short for "archive configuration"

[![Crate](https://img.shields.io/crates/v/arcconfig)](https://crates.io/crates/arcconfig)
[![GitHub License](https://img.shields.io/github/license/massivebird/arcconfig?color=blue)](https://github.com/massivebird/arcconfig/blob/main/LICENSE)

This is a dependency for my [arcsearch](https://github.com/massivebird/arcsearch) and [arcstat](https://github.com/massivebird/arcstat) projects!

🦀 written in Rust

## Why a shared dependency?

Both projects interact with my digital video game archive in similar ways. The two can sensibly share one configuration!

This reduces code redundancy and provides a great place to start on new, similar projects.

## Can I use it?

Yes! Honor the licensing, or else this guy will get you → 🦖

### Cool! So how do I use it?

First, you need a `config.yaml` file located in the root of your archive.

The `read_config` function reads this file's contents and generates a collection of `System` instances!

> For a quickstart on YAML syntax, click [here](https://docs.ansible.com/ansible/latest/reference_appendices/YAMLSyntax.html).

Here is an example configuration:

```yaml
# config.yaml
systems:
  ds: # system "label" — call it whatever you want!
    display_name: "DS"
    color: [135,215,255]
    path: "ds" # path relative to archive root
    games_are_directories: false # are games stored as directories?
  snes:
    display_name: "SNES"
    color: [95,0,255]
    path: "snes"
    games_are_directories: false
  wii:
    display_name: "WII"
    color: [0,215,255]
    path: "wbfs"
    games_are_directories: true
```

Feel free to use these `System` instances however you'd like!
