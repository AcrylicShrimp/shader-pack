use super::{Item, ItemLevel, ItemOrigin};
use colored::{ColoredString, Colorize};

pub fn stringify_item(item: &Item, apply_styles: bool) -> String {
    let mut lines = Vec::with_capacity(2 + 2 * item.sub_items.len() + 1);
    lines.push(apply_level_header(item.level, &item.message, apply_styles));

    if let Some(origin) = &item.origin {
        lines.push(stringify_origin(item.level, &item.message, origin, apply_styles).into());
    }

    for item in &item.sub_items {
        lines.push(apply_level_header(item.level, &item.message, apply_styles));

        if let Some(origin) = &item.origin {
            lines.push(stringify_origin(item.level, &item.message, origin, apply_styles).into());
        }
    }

    lines.push("".into());
    Vec::from_iter(lines.into_iter().map(|line| line.to_string())).join("\n")
}

fn stringify_origin(
    level: ItemLevel,
    message: &str,
    origin: &ItemOrigin,
    apply_styles: bool,
) -> String {
    let mut lines = Vec::with_capacity(9);

    let line_col_low = origin.file.find_line_col(origin.span.low());
    let path = match origin.file.path() {
        Some(path) => path.display().to_string(),
        None => "?".to_owned(),
    };
    let mut location = format!(
        "at {}:{}:{}",
        path,
        line_col_low.line + 1,
        line_col_low.col + 1
    )
    .normal();

    if apply_styles {
        location = location.bold();
    }

    lines.push(location);

    let line_col_high = origin.file.find_line_col(origin.span.high());
    let max_line = (origin.file.line_lows().len() - 1) as u32;
    let line_low = if line_col_low.line == 0 {
        0
    } else {
        line_col_low.line - 1
    };
    let line_high = if line_col_high.line == max_line {
        max_line
    } else {
        line_col_high.line + 1
    };
    let max_line_number_width = ((line_high + 1) as f64).log(10f64).ceil() as usize;

    if line_low != line_col_low.line {
        let line = origin.file.slice_line(line_low);
        let line = line.trim_end_matches(&['\n', '\r']);
        lines.push(
            format!(
                "{:>width$} | {}",
                line_low + 1,
                line,
                width = max_line_number_width + 1
            )
            .into(),
        );
    }

    if line_col_low.line != line_col_high.line {
        let line = origin.file.slice_line(line_col_low.line);
        let line = line.trim_end_matches(&['\n', '\r']);
        lines.push(
            format!(
                "{:>width$} | {}",
                line_col_low.line + 1,
                line,
                width = max_line_number_width + 1
            )
            .into(),
        );

        let indent = " ".repeat(max_line_number_width + 4 + line_col_low.col as usize);
        let caret = "^".repeat(line.len() - line_col_low.col as usize);
        lines.push(
            format!(
                "{}{}",
                indent,
                apply_level_color(level, &caret, apply_styles)
            )
            .into(),
        );
    }

    if 2 <= line_col_high.line - line_col_low.line {
        lines.push(" ...".into());
    }

    if line_col_low.line == line_col_high.line {
        let line = origin.file.slice_line(line_col_high.line);
        let line = line.trim_end_matches(&['\n', '\r']);
        lines.push(
            format!(
                "{:>width$} | {}",
                line_col_high.line + 1,
                line,
                width = max_line_number_width + 1
            )
            .into(),
        );

        let indent = " ".repeat(max_line_number_width + 4 + line_col_low.col as usize);
        let caret = "^".repeat(line_col_high.col as usize - line_col_low.col as usize);
        let message = format!(" {}", message);
        lines.push(
            format!(
                "{}{}{}",
                indent,
                apply_level_color(level, &caret, apply_styles),
                apply_level_color(level, &message, apply_styles)
            )
            .into(),
        );
    } else {
        let line = origin.file.slice_line(line_col_high.line);
        let line = line.trim_end_matches(&['\n', '\r']);
        lines.push(
            format!(
                "{:>width$} | {}",
                line_col_high.line + 1,
                line,
                width = max_line_number_width + 1
            )
            .into(),
        );

        let indent = " ".repeat(max_line_number_width + 4);
        let caret = "^".repeat(usize::max(line_col_high.col as usize, 1));
        let message = format!(" {}", message);
        lines.push(
            format!(
                "{}{}{}",
                indent,
                apply_level_color(level, &caret, apply_styles),
                apply_level_color(level, &message, apply_styles)
            )
            .into(),
        );
    }

    if line_high != line_col_high.line {
        let line = origin.file.slice_line(line_high);
        let line = line.trim_end_matches(&['\n', '\r']);
        lines.push(
            format!(
                "{:>width$} | {}",
                line_high + 1,
                line,
                width = max_line_number_width + 1
            )
            .into(),
        );
    }

    lines.push("".into());
    Vec::from_iter(lines.into_iter().map(|line| line.to_string())).join("\n")
}

fn apply_level_header(level: ItemLevel, str: &str, apply_styles: bool) -> ColoredString {
    let tag = match level {
        ItemLevel::Hint => " hint:",
        ItemLevel::Warning => " warn:",
        ItemLevel::Error => "error:",
    };
    let header = format!("{} {}", tag, str);

    if !apply_styles {
        return header.into();
    }

    apply_level_color(level, &header, true).bold()
}

fn apply_level_color(level: ItemLevel, str: &str, apply_styles: bool) -> ColoredString {
    if !apply_styles {
        return str.into();
    }

    match level {
        ItemLevel::Hint => str.bright_green(),
        ItemLevel::Warning => str.yellow(),
        ItemLevel::Error => str.red(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::{SourceMap, Span};

    #[test]
    fn test_stringify_item() {
        let mut map = SourceMap::new();
        let file = map.add_file("foo\nbar\nbaz\nbazz", "foo.txt", Some("foo.txt".into()));
        let item = Item {
            code: 0,
            level: ItemLevel::Error,
            message: "test".into(),
            origin: Some(ItemOrigin {
                file: file.clone(),
                span: Span::new(0, 1),
            }),
            sub_items: vec![],
        };

        let stringified = stringify_item(&item, false);
        println!("{}", stringified);
        assert_eq!(
            stringified,
            "error: test
at foo.txt:1:1
 1 | foo
     ^ test
 2 | bar

"
        );
    }
}
