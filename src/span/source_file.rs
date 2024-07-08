use super::{LineCol, Span};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Hash)]
pub struct SourceFile {
    span: Span,
    content: String,
    line_lows: Vec<u32>,
    name: String,
    path: Option<PathBuf>,
}

impl SourceFile {
    pub fn new(
        span_low: u32,
        content: impl Into<String>,
        name: impl Into<String>,
        path: impl Into<Option<PathBuf>>,
    ) -> Self {
        let content: String = content.into();
        let name = name.into();
        let path = path.into();

        if (u32::MAX as usize) < content.len() {
            panic!("content is too long");
        }

        let span = Span::new(span_low, span_low + content.len() as u32);
        let mut line_lows = Vec::with_capacity(512);
        line_lows.push(span_low);
        line_lows.extend(
            content
                .match_indices('\n')
                .map(|(pos, _)| span_low + pos as u32 + 1),
        );

        Self {
            span,
            content,
            line_lows,
            name,
            path,
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn line_lows(&self) -> &[u32] {
        &self.line_lows
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub fn line_span(&self, line: u32) -> Span {
        assert!((line as usize) < self.line_lows.len());
        Span::new(
            self.line_lows[line as usize],
            if (line + 1) as usize == self.line_lows.len() {
                self.span.high()
            } else {
                self.line_lows[line as usize + 1]
            },
        )
    }

    pub fn find_line(&self, pos: u32) -> u32 {
        assert!(self.span.contains(pos));
        match self.line_lows.binary_search(&pos) {
            Ok(line) => line as u32,
            Err(line) => line as u32 - 1,
        }
    }

    pub fn find_line_col(&self, pos: u32) -> LineCol {
        let line = self.find_line(pos);
        let line_span = self.line_span(line);
        let col = self.slice(line_span)[..(pos - line_span.low()) as usize]
            .chars()
            .count();
        LineCol::new(line, col as u32)
    }

    pub fn slice(&self, span: Span) -> &str {
        debug_assert!(self.span.contains_span(span));
        &self.content
            [(span.low() - self.span.low()) as usize..(span.high() - self.span.low()) as usize]
    }

    pub fn slice_line(&self, line: u32) -> &str {
        let span = self.line_span(line);
        self.slice(span)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn span_low() -> u32 {
        rand::thread_rng().gen_range(0..=u32::MAX / 2)
    }

    #[test]
    fn test_source_file_span() {
        let span_low = span_low();
        let file = SourceFile::new(span_low, "hello, world!", "test", None);
        assert_eq!(file.span(), Span::new(span_low, span_low + 13));
    }

    #[test]
    fn test_source_file_content() {
        let span_low = span_low();
        let file = SourceFile::new(span_low, "hello, world!", "test", None);
        assert_eq!(file.content(), "hello, world!");
    }

    #[test]
    fn test_source_file_line_lows() {
        let span_low = span_low();
        let file = SourceFile::new(span_low, "hello, world!\nfoo\nbar\nbaz\nbazz", "test", None);
        assert_eq!(
            file.line_lows(),
            &[
                span_low + 0,
                span_low + 14,
                span_low + 18,
                span_low + 22,
                span_low + 26
            ]
        );
    }

    #[test]
    fn test_source_file_name() {
        let span_low = span_low();
        let file = SourceFile::new(span_low, "hello, world!\nfoo\nbar\nbaz\nbazz", "test", None);
        assert_eq!(file.name(), "test");
    }

    #[test]
    fn test_source_file_path() {
        let span_low = span_low();
        let file = SourceFile::new(span_low, "hello, world!\nfoo\nbar\nbaz\nbazz", "test", None);
        assert_eq!(file.path(), None);

        let file = SourceFile::new(
            span_low,
            "hello, world!\nfoo\nbar\nbaz\nbazz",
            "test",
            Some(PathBuf::from("test")),
        );
        assert_eq!(file.path(), Some(Path::new("test")));
    }

    #[test]
    fn test_source_file_line_span() {
        let span_low = span_low();
        let file = SourceFile::new(span_low, "hello, world!\nfoo\nbar\nbaz\nbazz", "test", None);
        assert_eq!(file.line_span(0), Span::new(span_low + 0, span_low + 14));
        assert_eq!(file.line_span(1), Span::new(span_low + 14, span_low + 18));
        assert_eq!(file.line_span(2), Span::new(span_low + 18, span_low + 22));
        assert_eq!(file.line_span(3), Span::new(span_low + 22, span_low + 26));
        assert_eq!(file.line_span(4), Span::new(span_low + 26, span_low + 30));
    }

    #[test]
    #[should_panic]
    fn test_source_file_line_span_out_of_bounds() {
        let span_low = span_low();
        let file = SourceFile::new(span_low, "hello, world!\nfoo\nbar\nbaz\nbazz", "test", None);
        file.line_span(5);
    }

    #[test]
    fn test_source_file_find_line() {
        let span_low = span_low();
        let file = SourceFile::new(span_low, "hello, world!\nfoo\nbar\nbaz\nbazz", "test", None);
        assert_eq!(file.find_line(span_low + 0), 0);
        assert_eq!(file.find_line(span_low + 1), 0);
        assert_eq!(file.find_line(span_low + 2), 0);
        assert_eq!(file.find_line(span_low + 3), 0);
        assert_eq!(file.find_line(span_low + 4), 0);
        assert_eq!(file.find_line(span_low + 5), 0);
        assert_eq!(file.find_line(span_low + 6), 0);
        assert_eq!(file.find_line(span_low + 7), 0);
        assert_eq!(file.find_line(span_low + 8), 0);
        assert_eq!(file.find_line(span_low + 9), 0);
        assert_eq!(file.find_line(span_low + 10), 0);
        assert_eq!(file.find_line(span_low + 11), 0);
        assert_eq!(file.find_line(span_low + 12), 0);
        assert_eq!(file.find_line(span_low + 13), 0);
        assert_eq!(file.find_line(span_low + 14), 1);
        assert_eq!(file.find_line(span_low + 15), 1);
        assert_eq!(file.find_line(span_low + 16), 1);
        assert_eq!(file.find_line(span_low + 17), 1);
        assert_eq!(file.find_line(span_low + 18), 2);
        assert_eq!(file.find_line(span_low + 19), 2);
        assert_eq!(file.find_line(span_low + 20), 2);
        assert_eq!(file.find_line(span_low + 21), 2);
        assert_eq!(file.find_line(span_low + 22), 3);
        assert_eq!(file.find_line(span_low + 23), 3);
        assert_eq!(file.find_line(span_low + 24), 3);
        assert_eq!(file.find_line(span_low + 25), 3);
        assert_eq!(file.find_line(span_low + 26), 4);
        assert_eq!(file.find_line(span_low + 27), 4);
        assert_eq!(file.find_line(span_low + 28), 4);
        assert_eq!(file.find_line(span_low + 29), 4);
    }

    #[test]
    #[should_panic]
    fn test_source_file_find_line_out_of_bounds() {
        let span_low = span_low();
        let file = SourceFile::new(span_low, "hello, world!\nfoo\nbar\nbaz\nbazz", "test", None);
        file.find_line(span_low + 30);
    }

    #[test]
    fn test_source_file_find_line_col() {
        let span_low = span_low();
        let file = SourceFile::new(span_low, "hello, world!\nfoo\nbar\nbaz\nbazz", "test", None);
        assert_eq!(file.find_line_col(span_low + 0), LineCol::new(0, 0));
        assert_eq!(file.find_line_col(span_low + 1), LineCol::new(0, 1));
        assert_eq!(file.find_line_col(span_low + 13), LineCol::new(0, 13));

        assert_eq!(file.find_line_col(span_low + 14), LineCol::new(1, 0));
        assert_eq!(file.find_line_col(span_low + 15), LineCol::new(1, 1));
        assert_eq!(file.find_line_col(span_low + 17), LineCol::new(1, 3));

        assert_eq!(file.find_line_col(span_low + 18), LineCol::new(2, 0));
        assert_eq!(file.find_line_col(span_low + 19), LineCol::new(2, 1));
        assert_eq!(file.find_line_col(span_low + 21), LineCol::new(2, 3));

        assert_eq!(file.find_line_col(span_low + 22), LineCol::new(3, 0));
        assert_eq!(file.find_line_col(span_low + 23), LineCol::new(3, 1));
        assert_eq!(file.find_line_col(span_low + 25), LineCol::new(3, 3));

        assert_eq!(file.find_line_col(span_low + 26), LineCol::new(4, 0));
        assert_eq!(file.find_line_col(span_low + 27), LineCol::new(4, 1));
        assert_eq!(file.find_line_col(span_low + 29), LineCol::new(4, 3));
    }

    #[test]
    #[should_panic]
    fn test_source_file_fine_line_col_out_of_bounds() {
        let span_low = span_low();
        let file = SourceFile::new(span_low, "hello, world!\nfoo\nbar\nbaz\nbazz", "test", None);
        file.find_line_col(span_low + 30);
    }

    #[test]
    fn test_source_file_slice() {
        let span_low = span_low();
        let file = SourceFile::new(span_low, "hello, world!\nfoo\nbar\nbaz\nbazz", "test", None);
        assert_eq!(file.slice(file.span()), file.content());
        assert_eq!(
            file.slice(Span::new(span_low + 0, span_low + 13)),
            "hello, world!"
        );
        assert_eq!(file.slice(Span::new(span_low + 14, span_low + 17)), "foo");
        assert_eq!(file.slice(Span::new(span_low + 18, span_low + 21)), "bar");
        assert_eq!(file.slice(Span::new(span_low + 22, span_low + 25)), "baz");
        assert_eq!(file.slice(Span::new(span_low + 26, span_low + 30)), "bazz");
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn test_source_file_slice_out_of_bounds() {
        let span_low = span_low();
        let file = SourceFile::new(span_low, "hello, world!\nfoo\nbar\nbaz\nbazz", "test", None);
        file.slice(Span::new(span_low + 0, span_low + 31));
    }

    #[test]
    fn test_source_file_slice_line() {
        let span_low = span_low();
        let file = SourceFile::new(span_low, "hello, world!\nfoo\nbar\nbaz\nbazz", "test", None);
        assert_eq!(file.slice_line(0), "hello, world!\n");
        assert_eq!(file.slice_line(1), "foo\n");
        assert_eq!(file.slice_line(2), "bar\n");
        assert_eq!(file.slice_line(3), "baz\n");
        assert_eq!(file.slice_line(4), "bazz");
    }

    #[test]
    #[should_panic]
    fn test_source_file_slice_line_out_of_bounds() {
        let span_low = span_low();
        let file = SourceFile::new(span_low, "hello, world!\nfoo\nbar\nbaz\nbazz", "test", None);
        file.slice_line(5);
    }

    #[test]
    fn test_source_file_line_span_and_slice_line_len_equals() {
        let span_low = span_low();
        let file = SourceFile::new(span_low, "hello, world!\nfoo\nbar\nbaz\nbazz", "test", None);
        assert_eq!(file.line_span(0).len() as usize, file.slice_line(0).len());
        assert_eq!(file.line_span(1).len() as usize, file.slice_line(1).len());
        assert_eq!(file.line_span(2).len() as usize, file.slice_line(2).len());
        assert_eq!(file.line_span(3).len() as usize, file.slice_line(3).len());
        assert_eq!(file.line_span(4).len() as usize, file.slice_line(4).len());
    }
}
