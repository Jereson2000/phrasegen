use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Actions {
    Generate,
    Settings,
    Quit,
}

impl Actions {
    pub const VALUES: [Actions; 3] = [Actions::Generate, Actions::Settings, Actions::Quit];
}

// Dialoguer requires us to implement ToString method for the Selection struct
// and I want to use the enum because of the pattern matching so I did this :).
impl fmt::Display for Actions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Generate => write!(f, "Generate passphrase"),
            Self::Settings => write!(f, "Change settings"),
            Self::Quit => write!(f, "Quit"),
        }
    }
}
