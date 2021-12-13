use std::time::{Duration, Instant};
use prettytable::{Table, Row, Cell, Attr, color};
use prettytable::format::{Alignment, consts};

#[derive(Debug)]
pub struct ReportEntry {
    pub day: u32,
    pub answer_1: Option<String>,
    pub answer_2: Option<String>,
    pub read_duration: Duration,
    pub solve_duration: Duration,
    pub full_duration: Duration,
}

pub struct Report {
    entries: Vec<ReportEntry>,
    start_instant: Instant,
    end_instant: Option<Instant>,
}

impl Report {
    pub fn start() -> Report {
        Report {
            entries: vec![],
            start_instant: Instant::now(),
            end_instant: None,
        }
    }

    pub fn end(&mut self) {
        self.end_instant = Some(Instant::now());
    }

    pub fn add_entry(&mut self, entry: ReportEntry) {
        self.entries.push(entry);
    }

    pub fn display(&mut self) {
        self.entries.sort_by(|a, b| a.day.cmp(&b.day));
        let mut table = Table::new();
        table.set_format(*consts::FORMAT_BORDERS_ONLY);
        // table.set_format(*consts::FORMAT_BOX_CHARS);

        table.set_titles(Row::new(vec![
            Cell::new("Day").with_style(Attr::Bold),
            Cell::new("Part 1").with_style(Attr::Bold),
            Cell::new("Part 2").with_style(Attr::Bold),
            Cell::new("Read (ms)").with_style(Attr::Bold),
            Cell::new("Solve (ms)").with_style(Attr::Bold),
            Cell::new("Total (ms)").with_style(Attr::Bold),
        ]));

        for e in self.entries.iter() {
            table.add_row(Row::new(vec![
                Cell::new(&format!("{:02}", e.day)),
                self.format_answer(&e.answer_1),
                self.format_answer(&e.answer_2),
                self.format_duration(e.read_duration, None),
                self.format_duration(e.solve_duration, None),
                self.format_duration(e.full_duration, None).with_style(Attr::Bold),
            ]));
        }
        match self.end_instant {
            Some(t) => self.add_footer(&mut table, t - self.start_instant),
            None => ()
        };
        table.printstd();
    }

    fn add_footer(&self, table: &mut Table, duration: Duration) {
        table.add_row(Row::new(vec![
            Cell::new_align("Full duration (ms):", Alignment::RIGHT).with_hspan(5).with_style(Attr::Bold),
            self.format_duration(duration, Some(|d|
                if d <= Duration::from_millis(100) { color::BRIGHT_GREEN }
                else { color::BRIGHT_RED }
            )).with_style(Attr::Bold)
        ]));
    }

    fn format_answer(&self, ans: &Option<String>) -> Cell {
        let empty_value = "-".to_string();
        let value = ans.as_ref().unwrap_or(&empty_value);
        Cell::new(&value)
    }

    fn format_duration(&self, d: Duration, color_fn: Option<fn(Duration) -> color::Color>) -> Cell {
        let seconds = d.as_secs_f64();
        let cell = Cell::new(&format!("{:.3}", seconds * 1000 as f64));
        let color = match color_fn {
            Some(pick_color) => pick_color(d),
            _ => if d <= Duration::from_millis(2) {
                color::BRIGHT_GREEN
            }
            else if d <= Duration::from_millis(4) {
                color::BRIGHT_YELLOW
            }
            else {
                color::BRIGHT_RED
            }
        };
        cell.with_style(Attr::ForegroundColor(color))
    }
}

