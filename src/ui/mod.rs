pub mod common;
pub mod form;
pub mod header;
pub mod job_row;

use iced::{
    alignment,
    widget::{button, column, container, horizontal_rule, row, scrollable, text, Space},
    Element, Length, Padding, Theme,
};

use crate::message::Message;
use crate::state::JobTracker; // Removed unused imports
use crate::theme::*;
use crate::ui::common::*;
use crate::ui::form::{add_form, edit_form};
use crate::ui::header::app_header;
use crate::ui::job_row::{job_row, table_header};

pub fn view(state: &JobTracker) -> Element<'_, Message, Theme> {
    // App header with upgraded Kraken-style
    let header = app_header(state);

    // Get filtered and sorted jobs
    let jobs_to_display = state.sorted_jobs();

    // Create job rows with enhanced styling (with inline edit form)
    let job_rows = jobs_to_display
        .iter()
        .fold(column![].spacing(12), |col, &(index, job)| {
            // If this index is being edited, show the edit form directly in place of that row
            if state.editing_index == Some(index) {
                col.push(edit_form(index, &state.edit_form))
            } else {
                col.push(job_row(index, job))
            }
        });

    // Status summary counts
    let applied_count = state
        .jobs
        .iter()
        .filter(|j| j.status.to_string() == "Applied")
        .count();
    let rejected_count = state
        .jobs
        .iter()
        .filter(|j| j.status.to_string() == "Rejected")
        .count();
    let offer_count = state
        .jobs
        .iter()
        .filter(|j| j.status.to_string() == "Offer")
        .count();

    // Statistics summary
    let stats_row = row![
        text(format!("Applied: {}", applied_count))
            .size(12)
            .style(|_| text::Style {
                color: Some(kraken_secondary_text())
            }),
        text(format!("Rejected: {}", rejected_count))
            .size(12)
            .style(|_| text::Style {
                color: Some(kraken_secondary_text())
            }),
        text(format!("Offers: {}", offer_count))
            .size(12)
            .style(|_| text::Style {
                color: Some(kraken_secondary_text())
            }),
        Space::with_width(Length::Fill),
        text(format!(
            "Showing {} of {} applications",
            jobs_to_display.len(),
            state.jobs.len()
        ))
        .size(12)
        .style(|_| text::Style {
            color: Some(kraken_secondary_text())
        }),
    ]
        .spacing(15)
        .padding(Padding::new(10.0));

    // Empty state message when no jobs match filters
    let jobs_content = if jobs_to_display.is_empty() && !state.jobs.is_empty() {
        column![
            Space::with_height(Length::Fixed(30.0)),
            text("No applications match your filters")
                .size(16)
                .style(|_| text::Style {
                    color: Some(kraken_secondary_text())
                }),
            Space::with_height(Length::Fixed(10.0)),
            button(text("Clear Filters").size(14))
                .style(secondary_button_style)
                .padding(Padding::from([8, 15]))
                .on_press(Message::ClearFilters),
        ]
            .spacing(10)
            .align_x(alignment::Horizontal::Center)
            .width(Length::Fill)
    } else {
        column![
            table_header(state.sort_column, state.sort_order),
            job_rows,
            // Add some space at the bottom
            Space::with_height(Length::Fixed(20.0)),
        ]
            .spacing(15)
    };

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
        if state.editing_index.is_none() {
            add_form(state)
        } else {
            container(Space::with_height(Length::Fixed(0.0))).width(Length::Fill)
        },
        container(column![stats_row, jobs_content,].spacing(15))
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
