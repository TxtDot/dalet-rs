use crate::daletl::{DlArg, DlBody, IsNull};

impl IsNull for DlBody {
    fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
}

impl IsNull for DlArg {
    fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
}
