use crate::daletl::{DlArgument, DlBody, IsNull};

impl IsNull for DlBody {
    fn is_null(&self) -> bool {
        match self {
            Self::Null => true,
            _ => false,
        }
    }
}

impl IsNull for DlArgument {
    fn is_null(&self) -> bool {
        match self {
            Self::Null => true,
            _ => false,
        }
    }
}
