use std::fmt;
use crossterm::style::Stylize;
use super::{ Error, ErrorKind };

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str().red().bold())
    }
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.kind, self.message)?;
        let (start, end) = self.pos.as_span();
        write!(f, "    @ {}:[{}..{}]", self.file.display(), start, end)
//         type S = fn (x: &str) -> crossterm::style::StyledContent<&str>;
//         let style: S = match self.kind {
//             ReportKind::Warning   => |x| x.yellow().bold(),
//             ReportKind::SoftError => |x| x.red().with(Color::Rgb { r: 255, g: 160, b: 100 }).bold(),
//             ReportKind::HardError => |x| x.red().bold(),
//         };
//         for view in self.line_views.iter() {
//             let lineno_str = view.location.line.to_string();
//             writeln!(f, "{} {} {}", " ".repeat(lineno_str.len()), style("┌───"), view.location.file.display())?;
//             writeln!(f, "{} {} {}", lineno_str.as_str().magenta(), style("│"), view.line)?;
//             let Marker { start, end } = view.marker;
//             let len = view.line.len();
//             let spaces = len - (len - start);
//             let marker = end - start;
//             {
//                 let line = format!(
//                     "{} │ {}{}",
//                     " ".repeat(lineno_str.len()),
//                     " ".repeat(spaces),
//                     "^".repeat(marker).bold(),
//                 );
//                 writeln!(f, "{}", style(line.as_str()))?;
//             };
//             {
//                 let line = format!(
//                     "{} └─{}{}",
//                     " ".repeat(lineno_str.len()),
//                     "─".repeat(spaces),
//                     "┘".repeat(marker),
//                 );
//                 writeln!(f, "{}", style(line.as_str()))?;
//             };
//         }
//         writeln!(f)?;
//         let name = style(self.name.as_str());
//         writeln!(f, "{} {}: {}", style("┌─"), name, self.message)?;
//         for location in self.stack[..self.stack.len() - 1].iter() {
//             writeln!(f,
//                 "{} {}:{}:{}",
//                 style("├───>"),
//                 location.file.display(),
//                 location.line,
//                 location.column,
//             )?;
//         }
//         let location = &self.stack[self.stack.len() - 1];
//         write!(f,
//             "{} {}:{}:{}",
//             style("└─────>"),
//             location.file.display(),
//             location.line,
//             location.column,
//         )?;
//         Ok(())
    }
}
