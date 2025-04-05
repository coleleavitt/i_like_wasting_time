pub mod common;
pub mod form;
pub mod header;
pub mod job_row;

use iced::{
    widget::{column, container, horizontal_rule, scrollable, Space},
    Element, Length, Padding, Theme,
};

use crate::message::Message;
use crate::state::JobTracker;
use crate::theme::*;
use crate::ui::common::*;
use crate::ui::form::{add_form, edit_form};
use crate::ui::header::app_header;
use crate::ui::job_row::{job_row, table_header};

pub fn view(state: &JobTracker) -> Element<'_, Message, Theme> {
    // App header with upgraded Kraken-style
    let header = app_header(state);

    // Create job rows with enhanced styling (with inline edit form)
    let job_rows = state.jobs.iter().enumerate().fold(
        column![].spacing(12),
        |col, (index, job)| {
            // If this index is being edited, show the edit form directly in place of that row
            if state.editing_index == Some(index) {
                col.push(edit_form(index, &state.edit_form))
            } else {
                col.push(job_row(index, job))
            }
        },
    );

    // Put it all together with enhanced layout
    let content = column![
        header,
        horizontal_rule(1).style(|_| {
            iced::widget::rule::Style {
                color: kraken_border(),
                width: 1,
                radius: 0.0.into(),
                fill_mode: iced::widget::rule::FillMode::Full,
            }
        }),
        // Only show add form if we're not currently editing
        if state.editing_index.is_none() { add_form(state) } else { container(Space::with_height(0)).width(Length::Fill) },
        container(
            column![
                table_header(),
                job_rows,
                // Add some space at the bottom
                Space::with_height(Length::Fixed(20.0)),
            ]
            .spacing(15)
        )
        .padding(Padding::new(20.0))
        .width(Length::Fill)
    ];

    // Main container with improved scrolling
    container(scrollable(content))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(main_background)
        .into()
}
