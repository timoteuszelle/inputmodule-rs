//! Configuration Sources
use toml;
use std::fs;
use std::fmt;

/// Configuration Sources
///
/// To be used when initializing a config.
///
/// # Example
/// ```
/// static CONFIG: ezconf::Config = ezconf::INIT;
///
/// fn main() {
///     CONFIG.init([
///         // Try a config in the current directory
///         ezconf::Source::File("config.toml"),
///         // Then try some standard paths
///         ezconf::Source::File("~/.config.toml"),
///         ezconf::Source::File("~/.config/config.toml"),
///         ezconf::Source::File("/etc/config.toml"),
///         // Finally resort to a default config embedded in the program
///         ezconf::Source::Memory(include_str!("default.toml")),
///     ].iter()).unwrap();
/// }
/// ```
#[derive(Clone)]
pub enum Source<'a> {
    /// A config file
    File(&'a str),
    /// An in-memory config file
    Memory(&'a str),
}

impl<'a> fmt::Debug for Source<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Source::File(file) => write!(f, "File({:?})", file),
            Source::Memory(data) => {
                write!(
                f,
                "Memory({:?} ..., {})",
                data.chars().take(10).collect::<String>(),
                data.len(),
            )
            }
        }
    }
}

impl<'a> Source<'a> {
    pub(crate) fn try_read(&self) -> Option<toml::Value> {
        match self {
            Source::File(file) => {
                fs::File::open(file)
                    .and_then(|mut f| {
                        use std::io::Read;

                        let mut config_string = String::new();
                        f.read_to_string(&mut config_string).expect(
                            "Reading config file failed after succesful open",
                        );

                        Ok(config_string)
                    })
                    .ok()
                    .map(|s| {
                        s.parse::<toml::Value>().expect(&format!(
                            "File {:?} does not contain valid toml!",
                            file
                        ))
                    })
            }
            Source::Memory(data) => {
                Some(data.parse::<toml::Value>().expect(&format!(
                    "Memory slice does not contain valid toml!"
                )))
            }
        }
    }
}
