use crate::RootDatabase;
use hir::HasAttrs;
use std::sync::Arc;
use stdx::SepBy;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Documentation(Arc<str>);

impl Documentation {
    pub fn as_str(&self) -> &str {
        &*self.0
    }
}

pub trait HasDocs {
    fn docs(&self, db: &RootDatabase) -> Option<Documentation>;
}

impl Into<String> for Documentation {
    fn into(self) -> String {
        self.as_str().to_owned()
    }
}

impl<T: HasAttrs + Clone> HasDocs for T {
    fn docs(&self, db: &RootDatabase) -> Option<Documentation> {
        let attrs = self.clone().attrs(db);
        let strings = attrs.by_key("doc").string_values();
        let mut doc_texts = strings.peekable();
        if doc_texts.peek().is_none() {
            None
        } else {
            Some(Documentation(doc_texts.sep_by("\n").to_string().into()))
        }
    }
}
