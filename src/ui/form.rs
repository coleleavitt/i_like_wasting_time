use iced::{
    widget::{button, column, container, pick_list, row, text, text_input, Space},
    Background, Border, Color, Length, Padding, Shadow, Theme, Vector,
};

use crate::data::JobStatus;
use crate::message::Message;
use crate::state::{FormState, JobTracker};
use crate::theme::*;
use crate::ui::common::*;

// Extract the pick list style to avoid duplication
fn pick_list_style(_theme: &Theme, _status: pick_list::Status) -> pick_list::Style {
    pick_list::Style {
        text_color: kraken_text(),
        placeholder_color: kraken_secondary_text(),
        handle_color: kraken_secondary_text(),
        background: Background::Color(Color::from_rgb(0.12, 0.14, 0.16)),
        border: Border {
            color: kraken_border(),
            width: 1.0,
            radius: 6.0.into(),
        },
    }
}

pub fn add_form(state: &JobTracker) -> container::Container<'_, Message, Theme> {
    if !state.form.is_expanded || state.editing_index.is_some() {
        return container(Space::with_height(0)).width(Length::Fill);
    }

    let status_options = [
        JobStatus::Applied,
        JobStatus::OA,
        JobStatus::Interview,
        JobStatus::Rejected,
        JobStatus::Offer,
        JobStatus::Accepted,
        JobStatus::Withdrawn,
    ];

    let form_content = column![
        text("Add New Application")
            .size(18)
            .style(|_| text::Style { color: Some(kraken_text()) }),

        // Row 1: Company and Position
        row![
            column![
                text("Company")
                    .size(12)
                    .style(|_| text::Style { color: Some(kraken_secondary_text()) }),
                text_input("Company name", &state.form.company)
                    .padding(8)
                    .style(input_style)
                    .on_input(Message::CompanyChanged)
            ]
            .spacing(5)
            .width(Length::FillPortion(1)),

            column![
                text("Position")
                    .size(12)
                    .style(|_| text::Style { color: Some(kraken_secondary_text()) }),
                text_input("Job title", &state.form.position)
                    .padding(8)
                    .style(input_style)
                    .on_input(Message::PositionChanged)
            ]
            .spacing(5)
            .width(Length::FillPortion(1)),
        ]
        .spacing(15),

        // Row 2: Date and Status
        row![
            column![
                text("Date Applied")
                    .size(12)
                    .style(|_| text::Style { color: Some(kraken_secondary_text()) }),
                text_input("YYYY-MM-DD", &state.form.date_applied)
                    .padding(8)
                    .style(input_style)
                    .on_input(Message::DateChanged)
            ]
            .spacing(5)
            .width(Length::FillPortion(1)),

            column![
                text("Status")
                    .size(12)
                    .style(|_| text::Style { color: Some(kraken_secondary_text()) }),
                pick_list(
                    status_options,
                    state.form.status,
                    Message::StatusSelected
                )
                .padding(8)
                .style(pick_list_style)  // Using the extracted style function
            ]
            .spacing(5)
            .width(Length::FillPortion(1)),
        ]
        .spacing(15),

        // Row 3: Notes
        column![
            text("Notes")
                .size(12)
                .style(|_| text::Style { color: Some(kraken_secondary_text()) }),
            text_input("Additional notes about the application", &state.form.notes)
                .padding(8)
                .style(input_style)
                .on_input(Message::NotesChanged)
        ]
        .spacing(5),

        // Row 4: URL
        column![
            text("URL (Optional)")
                .size(12)
                .style(|_| text::Style { color: Some(kraken_secondary_text()) }),
            text_input("https://...", &state.form.url)
                .padding(8)
                .style(input_style)
                .on_input(Message::UrlChanged)
        ]
        .spacing(5),

        // Action buttons
        row![
            Space::with_width(Length::Fill),
            button(text("Reset").size(14))
                .style(secondary_button_style)
                .padding(Padding::from([10, 20]))
                .on_press(Message::ResetForm),
            button(text("Add Application").size(14))
                .style(primary_button_style)
                .padding(Padding::from([10, 20]))
                .on_press(Message::AddJob),
        ]
        .spacing(10)
    ]
        .spacing(15)
        .padding(20);

    container(form_content)
        .width(Length::Fill)
        .style(form_style)
}

pub fn edit_form(index: usize, form: &FormState) -> container::Container<'_, Message, Theme> {
    let status_options = [
        JobStatus::Applied,
        JobStatus::OA,
        JobStatus::Interview,
        JobStatus::Rejected,
        JobStatus::Offer,
        JobStatus::Accepted,
        JobStatus::Withdrawn,
    ];

    let edit_form_content = column![
        text(format!("Edit Application: {}", form.company))
            .size(16)
            .style(|_| text::Style { color: Some(kraken_warning()) }),

        // Row 1: Company and Position
        row![
            column![
                text("Company")
                    .size(12)
                    .style(|_| text::Style { color: Some(kraken_secondary_text()) }),
                text_input("Company name", &form.company)
                    .padding(8)
                    .style(input_style)
                    .on_input(Message::CompanyChanged)
            ]
            .spacing(5)
            .width(Length::FillPortion(1)),

            column![
                text("Position")
                    .size(12)
                    .style(|_| text::Style { color: Some(kraken_secondary_text()) }),
                text_input("Job title", &form.position)
                    .padding(8)
                    .style(input_style)
                    .on_input(Message::PositionChanged)
            ]
            .spacing(5)
            .width(Length::FillPortion(1)),
        ]
        .spacing(15),

        // Row 2: Date and Status
        row![
            column![
                text("Date Applied")
                    .size(12)
                    .style(|_| text::Style { color: Some(kraken_secondary_text()) }),
                text_input("YYYY-MM-DD", &form.date_applied)
                    .padding(8)
                    .style(input_style)
                    .on_input(Message::DateChanged)
            ]
            .spacing(5)
            .width(Length::FillPortion(1)),

            column![
                text("Status")
                    .size(12)
                    .style(|_| text::Style { color: Some(kraken_secondary_text()) }),
                pick_list(
                    status_options,
                    form.status,
                    Message::StatusSelected
                )
                .padding(8)
                .style(pick_list_style)  // Using the extracted style function
            ]
            .spacing(5)
            .width(Length::FillPortion(1)),
        ]
        .spacing(15),

        // Row 3: Notes
        column![
            text("Notes")
                .size(12)
                .style(|_| text::Style { color: Some(kraken_secondary_text()) }),
            text_input("Additional notes about the application", &form.notes)
                .padding(8)
                .style(input_style)
                .on_input(Message::NotesChanged)
        ]
        .spacing(5),

        // Row 4: URL
        column![
            text("URL (Optional)")
                .size(12)
                .style(|_| text::Style { color: Some(kraken_secondary_text()) }),
            text_input("https://...", &form.url)
                .padding(8)
                .style(input_style)
                .on_input(Message::UrlChanged)
        ]
        .spacing(5),

        // Action buttons - with edit-specific actions
        row![
            button(text("Delete").size(14))
                .style(delete_button_style)
                .padding(Padding::from([10, 20]))
                .on_press(Message::DeleteJob(index)),
            Space::with_width(Length::Fill),
            button(text("Cancel").size(14))
                .style(secondary_button_style)
                .padding(Padding::from([10, 20]))
                .on_press(Message::CancelEdit),
            button(text("Save Changes").size(14))
                .style(save_button_style)
                .padding(Padding::from([10, 20]))
                .on_press(Message::SaveEdit),
        ]
        .spacing(10)
    ]
        .spacing(15)
        .padding(20);

    container(edit_form_content)
        .width(Length::Fill)
        .style(edit_form_style)
}

fn form_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.09, 0.10, 0.12))),
        text_color: Some(kraken_text()),
        border: Border {
            color: kraken_border(),
            width: 1.0,
            radius: 8.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
            offset: Vector::new(0.0, 3.0),
            blur_radius: 10.0,
        },
        ..container::Style::default()
    }
}

fn edit_form_style(_theme: &Theme) -> container::Style {
    let mut style = form_style(_theme);
    style.border.color = kraken_warning();
    style.border.width = 1.5;
    style
}
