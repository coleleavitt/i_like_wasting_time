use iced::{
    Background, Border, Color, Length, Padding, Shadow, Theme, Vector, alignment,
    widget::{Space, button, column, container, pick_list, row, text, text_input},
};

use crate::data::JobStatus;
use crate::message::Message;
use crate::state::JobTracker;
use crate::theme::*;
use crate::ui::common::{
    delete_button_style, filter_section_style, input_style, secondary_button_style,
};

pub fn app_header(state: &JobTracker) -> container::Container<'_, Message, Theme> {
    // Top row: Title, main action buttons, and job count
    let top_row = row![
        // Left side: Title
        text("JOB TRACKER by Cole Leavitt")
            .size(24)
            .style(|_| text::Style {
                color: Some(kraken_text())
            }),
        Space::with_width(Length::Fill),
        // Right side: Primary actions
        if state.editing_index.is_some() {
            // When editing, show a button to finish editing first - styled to stand out
            button(text("Exit Edit Mode").size(14))
                .style(editing_action_button_style)
                .padding(Padding::from([8, 15]))
                .on_press(Message::CancelEdit)
        } else {
            // Normal toggle form button when not editing
            button(
                text(if state.form.is_expanded {
                    "Hide Form"
                } else {
                    "Add New Job"
                })
                .size(14),
            )
            .style(toggle_form_button_style)
            .padding(Padding::from(8))
            .on_press(Message::ToggleForm)
        },
        // Add some space between the button and application count
        Space::with_width(Length::Fixed(15.0)),
        // Application count with padding
        container(
            text(format!("{} APPLICATIONS", state.jobs.len()))
                .size(14)
                .style(|_| text::Style {
                    color: Some(kraken_secondary_text())
                })
        )
        .padding(Padding::from(4)),
    ]
        .spacing(15)
        .align_y(alignment::Vertical::Center)
        .padding(Padding::from([10, 20]));

    // Second row: Save status, save/reload buttons
    let status_row = row![
        // Left: Save status - Now checks has_unsaved_changes in both branches
        if state.has_unsaved_changes {
            text("Unsaved changes").size(12).style(|_| text::Style {
                color: Some(kraken_warning()),
            })
        } else if let Some(last_saved) = &state.last_saved {
            text(format!("Last saved: {}", last_saved))
                .size(12)
                .style(|_| text::Style {
                    color: Some(kraken_secondary_text()),
                })
        } else {
            // No changes and no previous save - first time launching with no data
            text("No changes").size(12).style(|_| text::Style {
                color: Some(kraken_secondary_text()),
            })
        },
        Space::with_width(Length::Fill),
        // Right: Save/Reload buttons with enhanced padding
        button(text("Save").size(14))
            .style(secondary_button_style)
            .padding(Padding::from([8, 15]))
            .on_press(Message::SaveData),
        Space::with_width(Length::Fixed(10.0)),
        button(text("Reload").size(14))
            .style(secondary_button_style)
            .padding(Padding::from([8, 15]))
            .on_press(Message::LoadData),
    ]
        .spacing(10)
        .align_y(alignment::Vertical::Center)
        .padding(Padding::from([5, 20]));

    // Filter row: Job filter controls - Disabled during editing
    let filter_row = row![
        text("Filter:").size(14).style(|_| text::Style {
            color: Some(if state.editing_index.is_some() {
                with_alpha(kraken_secondary_text(), 0.5) // Dimmed while editing
            } else {
                kraken_secondary_text()
            })
        }),
        pick_list(
            [
                JobStatus::All,
                JobStatus::Applied,
                JobStatus::OA,
                JobStatus::Interview,
                JobStatus::Rejected,
                JobStatus::Offer,
                JobStatus::Accepted,
                JobStatus::Withdrawn
            ],
            state.filter_status,
            Message::FilterStatusChanged
        )
        .padding(5)
        .style(|_theme, _status| {
            let alpha = if state.editing_index.is_some() { 0.6 } else { 1.0 };
            pick_list::Style {
                text_color: with_alpha(kraken_text(), alpha),
                placeholder_color: with_alpha(kraken_secondary_text(), alpha),
                handle_color: with_alpha(kraken_secondary_text(), alpha),
                background: Background::Color(with_alpha(Color::from_rgb(0.12, 0.14, 0.16), alpha)),
                border: Border {
                    color: with_alpha(kraken_border(), alpha),
                    width: 1.0,
                    radius: 4.0.into(),
                },
            }
        }),
        Space::with_width(Length::Fixed(15.0)),
        text_input("Search jobs...", &state.search_query)
            .padding(5)
            .width(Length::Fixed(200.0))
            .style(input_style)
            .on_input(Message::SearchQueryChanged),
        Space::with_width(Length::Fill),
        button(text("Clear").size(14))
            .style(secondary_button_style)
            .padding(Padding::from([6, 12]))
            .on_press(Message::ClearFilters),
    ]
        .spacing(10)
        .align_y(alignment::Vertical::Center)
        .padding(Padding::from([10, 20]));

    // Error display (only shows when there's an error)
    let error_display = if let Some(error) = &state.error_message {
        container(
            row![
                text(error).size(12).style(|_| text::Style {
                    color: Some(kraken_negative())
                }),
                Space::with_width(Length::Fill),
                button(text("✕").size(12))
                    .style(delete_button_style)
                    .padding(Padding::from(4))
                    .on_press(Message::ErrorDismissed),
            ]
                .spacing(10)
                .padding(Padding::from([8, 20])),
        )
            .width(Length::Fill)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgba(0.95, 0.27, 0.27, 0.1))),
                border: Border {
                    color: kraken_negative(),
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..container::Style::default()
            })
    } else {
        container(Space::with_height(Length::Fixed(0.0))).width(Length::Fill)
    };

    // Editing mode overlay notification
    let editing_notification = if state.editing_index.is_some() {
        container(
            text("Currently in edit mode - other actions are limited")
                .size(12)
                .style(|_| text::Style {
                    color: Some(fade_color(kraken_warning(), 0.9)) // Using fade_color here to address warning
                })
        )
            .width(Length::Fill)
            .padding(Padding::from([5, 20]))
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.3))),
                border: Border {
                    color: kraken_warning_subtle(),
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..container::Style::default()
            })
    } else {
        container(Space::with_height(Length::Fixed(0.0))).width(Length::Fill)
    };

    // Combine all sections into the header with improved spacing
    let header_content = column![
        top_row,
        status_row,
        editing_notification,
        container(filter_row)
            .width(Length::Fill)
            .style(filter_section_style),
        error_display,
    ]
        .spacing(8);

    container(header_content)
        .width(Length::Fill)
        .padding(Padding::from(5).bottom(5)) // Add bottom padding only
        .style(header_style)
}

fn toggle_form_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.14, 0.15, 0.17))),
            text_color: kraken_highlight_hover(),
            border: Border {
                color: kraken_highlight(),
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.129, 0.737, 0.514, 0.1),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 3.0,
            },
        },
        _ => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.11, 0.12, 0.14))),
            text_color: kraken_highlight(),
            border: Border {
                color: kraken_highlight_subtle(),
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow::default(),
        },
    }
}

// New enhanced style for the editing action button
fn editing_action_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    // Using the editing_button_style as a base and enhancing it
    let base_style = editing_button_style(_theme, status);

    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.22, 0.18, 0.12))),
            text_color: Color::WHITE,
            border: Border {
                color: kraken_warning(),
                width: 1.5,
                radius: 6.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.945, 0.769, 0.059, 0.2),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 4.0,
            },
        },
        _ => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.18, 0.15, 0.10))),
            text_color: Color::WHITE,
            border: Border {
                color: base_style.border.color,
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: base_style.shadow,
        },
    }
}

// This function is now used by editing_action_button_style
fn editing_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.18, 0.15, 0.12))),
            text_color: kraken_warning(),
            border: Border {
                color: kraken_warning(),
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.945, 0.769, 0.059, 0.1),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 3.0,
            },
        },
        _ => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.15, 0.13, 0.10))),
            text_color: kraken_warning(),
            border: Border {
                color: kraken_warning_subtle(),
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow::default(),
        },
    }
}

fn header_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(kraken_header_bg())),
        text_color: Some(kraken_text()),
        border: Border {
            color: kraken_border(),
            width: 1.0,
            radius: 0.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        ..container::Style::default()
    }
}
