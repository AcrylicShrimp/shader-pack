use crate::span::{SourceFile, Span};
use std::sync::Arc;

/// Represents a single diagnostics item.
/// It can hold multiple sub-items.
#[derive(Debug, Clone, Hash)]
pub struct Item {
    /// Code of the diagnostic, 0 for helper diagnostics.
    /// In other words, warning and error diagnostics should have a non-zero code.
    pub code: u32,
    pub level: ItemLevel,
    pub message: String,
    pub origin: Option<ItemOrigin>,
    pub sub_items: Vec<SubItem>,
}

/// Level of the diagnostics item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ItemLevel {
    Hint,
    Warning,
    Error,
}

/// Represents the origin of the diagnostics item.
/// Used to trace which part of the codebase caused the diagnostic.
#[derive(Debug, Clone, Hash)]
pub struct ItemOrigin {
    pub file: Arc<SourceFile>,
    pub span: Span,
}

/// Represents a single sub-item of the diagnostics item.
/// A sub-item is a child of its parent diagnostic.
#[derive(Debug, Clone, Hash)]
pub struct SubItem {
    pub level: ItemLevel,
    pub message: String,
    pub origin: Option<ItemOrigin>,
}
