#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Interactive {
    Yes,
    No,
}

impl From<bool> for Interactive {
    fn from(b: bool) -> Self {
        if b {
            Interactive::Yes
        } else {
            Interactive::No
        }
    }
}

impl From<&Interactive> for bool {
    fn from(i: &Interactive) -> Self {
        match i {
            Interactive::Yes => true,
            Interactive::No => false,
        }
    }
}

impl Default for Interactive {
    fn default() -> Self {
        Interactive::No
    }
}
