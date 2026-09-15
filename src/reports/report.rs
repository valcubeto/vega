use crossterm::style::{Stylize, Color};
use std::{fmt, path::Path};

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Report<'a> {
    pub kind: ReportKind,
    name: String,
    message: String,
    stack: Vec<Location<'a>>,
    line_views: Vec<LineView<'a>>,
}

#[allow(dead_code)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ReportKind {
    Warning,
    SoftError,
    HardError
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy)]
pub struct Location<'a> {
    pub file: &'a Path,
    pub line: u16,
    pub column: u16,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct LineView<'a> {
    pub location: Location<'a>,
    pub line: &'a str,
    pub marker: Marker,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy)]
pub struct Marker {
    pub start: usize,
    pub end: usize,
}

impl Marker {
    pub fn char(idx: usize) -> Self {
        Marker { start: idx, end: idx + 1 }
    }
    pub fn span(start: usize, end: usize) -> Self {
        Marker { start, end }
    }
}

impl<'a> Report<'a> {
    pub fn new(kind: ReportKind, name: String, message: String) -> Self {
        Report {
            kind,
            name,
            message,
            line_views: Vec::new(),
            stack: Vec::new()
        }
    }

    pub fn add_stack(&mut self, location: Location<'a>) -> &mut Self {
        self.stack.push(location);
        self
    }

    pub fn add_line_view(&mut self, view: LineView<'a>) -> &mut Self {
        self.line_views.push(view);
        self
    }
}

impl<'a> fmt::Display for Report<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        type S = fn (x: &str) -> crossterm::style::StyledContent<&str>;
        let style: S = match self.kind {
            ReportKind::Warning   => |x| x.yellow().bold(),
            ReportKind::SoftError => |x| x.red().with(Color::Rgb { r: 255, g: 160, b: 100 }).bold(),
            ReportKind::HardError => |x| x.red().bold(),
        };
        for view in self.line_views.iter() {
            let lineno_str = view.location.line.to_string();
            writeln!(f, "{} {} {}", " ".repeat(lineno_str.len()), style("┌───"), view.location.file.display())?;
            writeln!(f, "{} {} {}", lineno_str.as_str().magenta(), style("│"), view.line)?;
            let Marker { start, end } = view.marker;
            let len = view.line.len();
            let spaces = len - (len - start);
            let marker = end - start;
            {
                let line = format!(
                    "{} │ {}{}",
                    " ".repeat(lineno_str.len()),
                    " ".repeat(spaces),
                    "^".repeat(marker).bold(),
                );
                writeln!(f, "{}", style(line.as_str()))?;
            };
            {
                let line = format!(
                    "{} └─{}{}",
                    " ".repeat(lineno_str.len()),
                    "─".repeat(spaces),
                    "┘".repeat(marker),
                );
                writeln!(f, "{}", style(line.as_str()))?;
            };
        }
        writeln!(f)?;
        let name = style(self.name.as_str());
        writeln!(f, "{} {}: {}", style("┌─"), name, self.message)?;
        for location in self.stack[..self.stack.len() - 1].iter() {
            writeln!(f,
                "{} {}:{}:{}",
                style("├───>"),
                location.file.display(),
                location.line,
                location.column,
            )?;
        }
        let location = &self.stack[self.stack.len() - 1];
        write!(f,
            "{} {}:{}:{}",
            style("└─────>"),
            location.file.display(),
            location.line,
            location.column,
        )?;
        Ok(())
    }
}
