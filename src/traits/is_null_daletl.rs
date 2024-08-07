use crate::daletl::{DlArgument, DlBody, IsNull};

impl IsNull for DlBody {
    fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
}

impl IsNull for DlArgument {
    fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
}
