use super::SourceFile;
use std::{path::PathBuf, sync::Arc};

#[derive(Debug, Clone, Hash)]
pub struct SourceMap {
    span_high: u32,
    files: Vec<Arc<SourceFile>>,
}

impl SourceMap {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            span_high: 0,
        }
    }

    pub fn add_file(
        &mut self,
        content: impl Into<String>,
        name: impl Into<String>,
        path: impl Into<Option<PathBuf>>,
    ) -> Arc<SourceFile> {
        let file = Arc::new(SourceFile::new(self.span_high, content, name, path));
        self.span_high = file.span().high();
        self.files.push(file.clone());
        file
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_map_add_file() {
        let mut source_map = SourceMap::new();
        let file = source_map.add_file("content", "name", None);
        assert_eq!(file.span().low(), 0);
        assert_eq!(file.span().high(), 7);
        assert_eq!(file.content(), "content");
        assert_eq!(file.name(), "name");
        assert_eq!(file.path(), None);
    }
}
