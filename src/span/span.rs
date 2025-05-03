#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    low: u32,
    high: u32,
}

impl Span {
    pub const ZERO: Self = Self::new(0, 0);

    pub const fn new(low: u32, high: u32) -> Self {
        debug_assert!(low <= high);
        Self { low, high }
    }

    pub const fn empty(pos: u32) -> Self {
        Self::new(pos, pos)
    }

    pub const fn low(self) -> u32 {
        self.low
    }

    pub const fn high(self) -> u32 {
        self.high
    }

    pub const fn len(self) -> u32 {
        self.high - self.low
    }

    pub const fn is_empty(self) -> bool {
        self.low == self.high
    }

    pub const fn contains(self, pos: u32) -> bool {
        self.low <= pos && pos < self.high
    }

    pub const fn contains_span(self, span: Self) -> bool {
        self.low <= span.low && span.high <= self.high
    }

    pub fn expand_to(self, high: u32) -> Self {
        if high == u32::MAX {
            return self;
        }

        Self {
            low: self.low,
            high: high.max(self.high),
        }
    }

    pub fn merge(lhs: Self, rhs: Self) -> Self {
        Self {
            low: lhs.low.min(rhs.low),
            high: rhs.high.max(rhs.high),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_new() {
        Span::new(0, 0);
        Span::new(0, 1);
        Span::new(1, 2);
        Span::new(2, 3);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn test_span_new_low_is_higher_than_high() {
        Span::new(1, 0);
    }

    #[test]
    fn test_span_len() {
        let span = Span::new(0, 5);
        assert_eq!(span.len(), 5);
    }

    #[test]
    fn test_span_is_empty() {
        let span = Span::new(0, 0);
        assert_eq!(span.is_empty(), true);

        let span = Span::new(0, 1);
        assert_eq!(span.is_empty(), false);
    }

    #[test]
    fn test_span_contains() {
        let span = Span::new(0, 5);
        assert_eq!(span.contains(0), true);
        assert_eq!(span.contains(4), true);
        assert_eq!(span.contains(5), false);
    }

    #[test]
    fn test_span_contains_span() {
        let span1 = Span::new(10, 15);
        let span2 = Span::new(11, 14);
        assert_eq!(span1.contains_span(span2), true);

        let span3 = Span::new(5, 12);
        assert_eq!(!span1.contains_span(span3), true);

        let span4 = Span::new(13, 16);
        assert_eq!(!span1.contains_span(span4), true);
    }

    #[test]
    fn test_span_expand_to() {
        let span = Span::new(0, 5);
        let expanded_span = span.expand_to(10);
        assert_eq!(expanded_span, Span::new(0, 10));

        let expanded_span = span.expand_to(3);
        assert_eq!(expanded_span, Span::new(0, 5));
    }

    #[test]
    fn test_span_merge() {
        let span1 = Span::new(0, 5);
        let span2 = Span::new(3, 10);
        let merged_span = Span::merge(span1, span2);
        assert_eq!(merged_span, Span::new(0, 10));
    }
}
