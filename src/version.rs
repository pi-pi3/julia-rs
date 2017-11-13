
use std::fmt;
use std::str::FromStr;

const MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
const MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
const PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");
const RELEASE: bool = false;
const BRANCH: Option<&str> = None;
const COMMIT: Option<&str> = None;

#[derive(Clone)]
pub struct Version<'a> {
    pub name: &'a str,
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub release: bool,
    pub branch: Option<&'a str>,
    pub commit: Option<&'a str>,
}

pub fn get() -> Version<'static> {
    Version {
        name: "julia-rs",
        major: u32::from_str(MAJOR).expect("COULD NOT PARSE MAJOR VERSION NUMBER"),
        minor: u32::from_str(MINOR).expect("COULD NOT PARSE MINOR VERSION NUMBER"),
        patch: u32::from_str(PATCH).expect("COULD NOT PARSE PATCH VERSION NUMBER"),
        release: RELEASE,
        branch: BRANCH,
        commit: COMMIT,
    }
}

impl<'a> Version<'a> {
    pub fn shorten(self) -> Version<'a> {
        Version {
            name: self.name,
            major: self.major,
            minor: self.minor,
            patch: self.patch,
            release: self.release,
            branch: None,
            commit: None,
        }
    }
}

impl<'a> fmt::Debug for Version<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"v"{}.{}.{}""#, self.major, self.minor, self.patch)
    }
}

impl<'a> fmt::Display for Version<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(branch) = self.branch {
            if let Some(commit) = self.commit {
                write!(
                    f,
                    r#"{} {}.{}.{} ({}-{})"#,
                    self.name,
                    self.major,
                    self.minor,
                    self.patch,
                    branch,
                    &commit[..7]
                )
            } else {
                write!(
                    f,
                    r#"{} {}.{}.{} ({})"#,
                    self.name,
                    self.major,
                    self.minor,
                    self.patch,
                    branch
                )
            }
        } else if let Some(commit) = self.commit {
            write!(
                f,
                r#"{} {}.{}.{} ({})"#,
                self.name,
                self.major,
                self.minor,
                self.patch,
                &commit[..7]
            )
        } else {
            write!(
                f,
                r#"{} {}.{}.{}"#,
                self.name,
                self.major,
                self.minor,
                self.patch
            )
        }
    }
}
