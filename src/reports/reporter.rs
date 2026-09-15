use std::{fmt, path::Path};
use super::*;

pub struct Reporter<'a> {
    file: &'a Path,
    pub data: &'a str,
    reports: Vec<Report<'a>>,
}

impl<'a> Reporter<'a> {
    pub fn new(file: &'a Path, data: &'a str) -> Self {
        Reporter {
            file,
            data,
            reports: Vec::new(),
        }
    }

    pub fn print_all(&self) {
        for report in self.reports.iter() {
            eprintln!("{report}");
        }
    }

    pub fn errored(&self) -> bool {
        self.reports.iter().any(|rep| rep.kind == ReportKind::SoftError || rep.kind == ReportKind::HardError)
    }

    pub fn view(&self) -> &[Report<'a>] {
        self.reports.as_slice()
    }

    #[allow(unused)]
    pub fn warn<D: fmt::Display>(&mut self, msg: D, idx: usize) {
        let mut report = Report::new(ReportKind::Warning, "Warning".to_string(), msg.to_string());
        report.add_stack(self.get_pos(idx));
        self.reports.push(report);
    }

    fn error<D: fmt::Display>(&mut self, kind: ReportKind, name: &str, msg: D, marker: Marker) {
        let mut report = Report::new(kind, name.to_string(), msg.to_string());
        let location = self.get_pos(marker.start);
        report.add_line_view(self.get_line_view(marker, location));
        report.add_stack(location);
        self.reports.push(report);
    }

    fn soft_error<D: fmt::Display>(&mut self, name: &str, msg: D, marker: Marker) {
        self.error(ReportKind::SoftError, name, msg, marker)
    }

    // TODO: replace T with never once it's stable.
    fn hard_error<D: fmt::Display>(&mut self, name: &str, msg: D, marker: Marker) {
        self.error(ReportKind::HardError, name, msg, marker);
    }

    pub fn todo<D: fmt::Display>(&mut self, msg: D, marker: Marker) {
        self.hard_error("Not yet implemented", msg, marker)
    }

    pub fn syntax_soft<D: fmt::Display>(&mut self, msg: D, marker: Marker) {
        self.soft_error("Syntax error", msg, marker)
    }

    pub fn syntax<D: fmt::Display>(&mut self, msg: D, marker: Marker) {
        self.hard_error("Syntax error", msg, marker)
    }

    pub fn parse<D: fmt::Display>(&mut self, msg: D, marker: Marker) {
        self.hard_error("Parse error", msg, marker)
    }

    // If idx is out of bounds, it's my fault.
    fn get_line_view(&self, marker: Marker, location: Location<'a>) -> LineView<'a> {
        // Actual start of tokens, not counting spaces.
        let mut start = marker.start;
        let mut left = self.data.as_bytes()[..start].iter().rev().enumerate();
        while let Some((i, ch)) = left.next() && *ch != b'\n' {
            if !ch.is_ascii_whitespace() {
                start = marker.start - i - 1;
            }
        }

        let mut end = marker.end;
        let mut right = self.data.as_bytes()[end..].iter().enumerate();
        while let Some((i, ch)) = right.next() && *ch != b'\n' {
            if !ch.is_ascii_whitespace() {
                end = marker.end + i + 1;
            }
        }

        LineView {
            location,
            line: &self.data[start..end],
            marker: Marker {
                start: marker.start - start,
                end: (marker.end - marker.start) + marker.start - start,
            },
        }
    }

    fn get_pos(&self, idx: usize) -> Location<'a> {
        // TODO: cache line start indexes
        let mut line = 1;
        let mut column = 1;

        for ch in self.data[..idx].chars() {
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }

        Location {
            file: self.file,
            line,
            column
        }
    }
}
