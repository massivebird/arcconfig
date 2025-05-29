const PREFIX: &str = "archive config error: ";

#[derive(Debug)]
pub struct BadSystemLabelSomewhere;

impl std::fmt::Display for BadSystemLabelSomewhere {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{PREFIX} bad system label somewhere")
    }
}

impl std::error::Error for BadSystemLabelSomewhere {}

#[derive(Debug)]
pub struct MissingProperty {
    pub sys_label: String,
    pub property: String,
}

impl MissingProperty {
    #[must_use]
    pub const fn new(sys_label: String, property: String) -> Self {
        Self {
            sys_label,
            property,
        }
    }
}

impl std::fmt::Display for MissingProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{PREFIX} missing `{}` property for {}",
            self.property, self.sys_label
        )
    }
}

impl std::error::Error for MissingProperty {}

#[derive(Debug)]
pub struct SystemDirNotFound {
    pub sys_label: String,
    pub dir: String,
}

impl SystemDirNotFound {
    #[must_use]
    pub const fn new(sys_label: String, dir: String) -> Self {
        Self { sys_label, dir }
    }
}

impl std::fmt::Display for SystemDirNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{PREFIX} system dir `{}` not found for system labelled {}",
            self.dir, self.sys_label
        )
    }
}

impl std::error::Error for SystemDirNotFound {}
