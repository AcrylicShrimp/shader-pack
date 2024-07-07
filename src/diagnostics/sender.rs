use super::{Item, ItemLevel, ItemOrigin, SubItem};
use crate::span::{SourceFile, Span};
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug, Clone)]
pub struct ItemSender {
    file: Arc<SourceFile>,
    sender: UnboundedSender<Item>,
}

impl ItemSender {
    pub fn new(file: Arc<SourceFile>, sender: UnboundedSender<Item>) -> Self {
        Self { file, sender }
    }

    pub fn file(&self) -> Arc<SourceFile> {
        self.file.clone()
    }

    pub fn hint(&self, span: Span, message: String) {
        self.sender
            .send(Item {
                code: 0,
                level: ItemLevel::Hint,
                message,
                origin: Some(ItemOrigin {
                    file: self.file.clone(),
                    span,
                }),
                sub_items: vec![],
            })
            .unwrap();
    }

    pub fn hint_sub(&self, span: Span, message: String, sub_items: Vec<SubItem>) {
        self.sender
            .send(Item {
                code: 0,
                level: ItemLevel::Hint,
                message,
                origin: Some(ItemOrigin {
                    file: self.file.clone(),
                    span,
                }),
                sub_items,
            })
            .unwrap();
    }

    pub fn hint_simple(&self, message: String) {
        self.sender
            .send(Item {
                code: 0,
                level: ItemLevel::Hint,
                message,
                origin: None,
                sub_items: vec![],
            })
            .unwrap()
    }

    pub fn warning(&self, code: u32, span: Span, message: String) {
        self.sender
            .send(Item {
                code,
                level: ItemLevel::Warning,
                message,
                origin: Some(ItemOrigin {
                    file: self.file.clone(),
                    span,
                }),
                sub_items: vec![],
            })
            .unwrap()
    }

    pub fn warning_sub(&self, code: u32, span: Span, message: String, sub_items: Vec<SubItem>) {
        self.sender
            .send(Item {
                code,
                level: ItemLevel::Warning,
                message,
                origin: Some(ItemOrigin {
                    file: self.file.clone(),
                    span,
                }),
                sub_items,
            })
            .unwrap()
    }

    pub fn warning_simple(&self, code: u32, message: String) {
        self.sender
            .send(Item {
                code,
                level: ItemLevel::Warning,
                message,
                origin: None,
                sub_items: vec![],
            })
            .unwrap()
    }

    pub fn error(&self, code: u32, span: Span, message: String) {
        self.sender
            .send(Item {
                code,
                level: ItemLevel::Error,
                message,
                origin: Some(ItemOrigin {
                    file: self.file.clone(),
                    span,
                }),
                sub_items: vec![],
            })
            .unwrap()
    }

    pub fn error_sub(&self, code: u32, span: Span, message: String, sub_items: Vec<SubItem>) {
        self.sender
            .send(Item {
                code,
                level: ItemLevel::Error,
                message,
                origin: Some(ItemOrigin {
                    file: self.file.clone(),
                    span,
                }),
                sub_items,
            })
            .unwrap()
    }

    pub fn error_simple(&self, code: u32, message: String) {
        self.sender
            .send(Item {
                code,
                level: ItemLevel::Error,
                message,
                origin: None,
                sub_items: vec![],
            })
            .unwrap()
    }

    pub fn sub_hint(&self, span: Span, message: String) -> SubItem {
        SubItem {
            level: ItemLevel::Hint,
            message,
            origin: Some(ItemOrigin {
                file: self.file.clone(),
                span,
            }),
        }
    }

    pub fn sub_hint_simple(&self, message: String) -> SubItem {
        SubItem {
            level: ItemLevel::Hint,
            message,
            origin: None,
        }
    }

    pub fn sub_warning(&self, span: Span, message: String) -> SubItem {
        SubItem {
            level: ItemLevel::Warning,
            message,
            origin: Some(ItemOrigin {
                file: self.file.clone(),
                span,
            }),
        }
    }

    pub fn sub_warning_simple(&self, message: String) -> SubItem {
        SubItem {
            level: ItemLevel::Warning,
            message,
            origin: None,
        }
    }

    pub fn sub_error(&self, span: Span, message: String) -> SubItem {
        SubItem {
            level: ItemLevel::Error,
            message,
            origin: Some(ItemOrigin {
                file: self.file.clone(),
                span,
            }),
        }
    }

    pub fn sub_error_simple(&self, message: String) -> SubItem {
        SubItem {
            level: ItemLevel::Error,
            message,
            origin: None,
        }
    }
}
