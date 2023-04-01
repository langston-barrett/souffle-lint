#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Interactive {
    Yes,
    #[default]
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
