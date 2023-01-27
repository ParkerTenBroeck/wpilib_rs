use std::{
    fmt::{Debug, Display},
    time::{Duration, Instant},
};

#[derive(Debug, Clone)]
pub struct WatchdogEpoch {
    name: String,
    start: Instant,
    end: Instant,
}

#[derive(Debug, Clone)]
pub struct Watchdog {
    target_dur: Duration,
    start: Instant,
    epochs: Vec<WatchdogEpoch>,
    curr_epoch: Option<WatchdogEpoch>,
}

#[derive(Debug, Clone)]
pub struct WatchdogOverrun {
    overrun: Duration,
    total: Duration,
    target: Duration,
    epochs: Vec<WatchdogEpoch>,
}

impl Display for WatchdogOverrun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Target  Duration: {:.6}ms",
            self.target.as_secs_f64() * 1000.0
        )?;
        writeln!(
            f,
            "Total   Duration: {:.6}ms",
            self.total.as_secs_f64() * 1000.0
        )?;
        writeln!(
            f,
            "Overrun Duration: {:.6}ms",
            self.overrun.as_secs_f64() * 1000.0
        )?;
        for epoch in &self.epochs {
            writeln!(
                f,
                "\t{} -> {:.6}ms",
                epoch.name,
                (epoch.end - epoch.start).as_secs_f64() * 1000.0
            )?;
        }
        writeln!(f)
    }
}

impl Watchdog {
    pub fn start(target_dur: Duration) -> Self {
        Self {
            target_dur,
            start: Instant::now(),
            epochs: Vec::new(),
            curr_epoch: None,
        }
    }

    pub fn add_epoch(&mut self, name: impl Into<String>) {
        let now = Instant::now();
        if let Some(mut curr_epoch) = self.curr_epoch.take() {
            curr_epoch.end = Instant::now();
            self.epochs.push(curr_epoch);
        }
        self.curr_epoch = Some(WatchdogEpoch {
            name: name.into(),
            start: now,
            end: now,
        })
    }

    pub fn end_epoch(&mut self) {
        if let Some(mut curr_epoch) = self.curr_epoch.take() {
            curr_epoch.end = Instant::now();
            self.epochs.push(curr_epoch);
            self.curr_epoch = None;
        }
    }

    pub fn end(mut self) -> Result<Duration, WatchdogOverrun> {
        let now = Instant::now();
        if let Some(mut curr_epoch) = self.curr_epoch.take() {
            curr_epoch.end = now;
            self.epochs.push(curr_epoch);
            self.curr_epoch = None;
        }
        let dur = now.duration_since(self.start);
        if dur >= self.target_dur {
            Err(WatchdogOverrun {
                overrun: dur - self.target_dur,
                total: dur,
                target: self.target_dur,
                epochs: self.epochs,
            })
        } else {
            Ok(dur)
        }
    }
}
