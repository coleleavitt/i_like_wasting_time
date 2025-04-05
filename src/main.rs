mod data;
mod message;
mod state;
mod theme;
mod ui;
mod update;
mod storage;

use iced::{application, Size, Theme, Task};
use state::JobTracker;
use ui::view;
use update::update;

fn main() -> iced::Result {
    // Using the new application API with Wayland compatibility
    application("Job Application Tracker", update, view)
        .theme(|_| Theme::Dark)
        .window_size(Size::new(1100.0, 700.0))
        .antialiasing(true)
        .run_with(|| {
            (
                JobTracker::new(), // Fixed - no arguments needed
                Task::none(),
            )
        })
}
