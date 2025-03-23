use ltrait::color_eyre::Result;
use ltrait::{Launcher, Level, filter::ClosureFilter, sorter::ClosureSorter};

use ltrait_ui_tui::{Tui, TuiConfig, TuiEntry, Viewport, style::Style};

use std::cmp;

enum Item {
    Num(u32),
}

impl Into<String> for &Item {
    fn into(self) -> String {
        match self {
            Item::Num(x) => format!("{x}"),
            _ => "unknown item".into(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // keeping _guard is required to write log
    let _guard = ltrait::setup(Level::INFO)?;

    let launcher = Launcher::default()
        // the simplest source
        .add_source(
            ltrait::source::from_iter(1..=5000),
            /* transformer */ Item::Num,
        )
        .add_raw_filter(ClosureFilter::new(|c, _ /* input */| {
            match c {
                Item::Num(x) => (x % 2) == 0,
                _ => true, // If variants are added to Item in the future, they are ignored here
            }
        }))
        .add_raw_sorter(ClosureSorter::new(|lhs, rhs, _| {
            match (lhs, rhs) {
                // In default, the Launcher evaluates so that the larger ones are brought to the front,
                // but here the smaller ones are brought to the front.
                //
                // I'll make it configurable soon.
                (Item::Num(lhs), Item::Num(rhs)) => rhs.cmp(lhs),
                _ => cmp::Ordering::Equal,
            }
        }))
        .batch_size(500)
        .set_ui(
            Tui::new(TuiConfig::new(
                Viewport::Fullscreen,
                '>', // Selected
                ' ',
            )),
            |c| TuiEntry {
                text: (c.into(), Style::new()),
            },
        );

    launcher.run().await
}
