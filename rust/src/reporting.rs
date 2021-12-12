use std::time::{Duration, Instant};

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

    pub fn display(&self) {

        println!(
            "{0: <4} {1: <20} {2: <20} {3: >15} {4: >15} {5: >15}",
            "Day", "Part 1", "Part 2", "Read (ms)", "Solve (ms)", "Total (ms)"
        );

        for e in self.entries.iter() {
            println!(
                "{0: <4} {1: <20} {2: <20} {3: >15.3} {4: >15.3} {5: >15.3}",
                e.day,
                e.answer_1.as_ref().unwrap_or(&"-".to_string()),
                e.answer_2.as_ref().unwrap_or(&"-".to_string()),
                e.read_duration.as_micros() as f32 / 1000 as f32,
                e.solve_duration.as_micros() as f32 / 1000 as f32,
                e.full_duration.as_micros() as f32 / 1000 as f32,
            );
        }
        match self.end_instant {
            Some(t) => println!("Full duration: {:.3} ms", (t - self.start_instant).as_micros() as f32 / 1000 as f32),
            None => ()
        };
    }
}

