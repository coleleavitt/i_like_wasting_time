use iced::{widget::{button, container, row, text, Space}, Alignment, Background, Border, Color, Length, Padding, Shadow, Theme, Vector};

use crate::message::Message;
use crate::state::JobTracker;
use crate::theme::*;

pub fn app_header(state: &JobTracker) -> container::Container<'_, Message, Theme> {
    let content = row![
        text("JOB TRACKER by Cole Leavitt")
            .size(24)
            .style(|_| text::Style { color: Some(kraken_text()) }),
        Space::with_width(Length::Fill),
        button(
            text(if state.form.is_expanded { "Hide Form" } else { "Add New Job" })
                .size(14)
        )
        .style(toggle_form_button_style)
        .padding(Padding::from([8, 15]))
        .on_press(Message::ToggleForm),
        text(format!("{} APPLICATIONS", state.jobs.len()))
            .size(14)
            .style(|_| text::Style { color: Some(kraken_secondary_text()) }),
    ]
        .spacing(15)
        .align_y(Alignment::Center)
        .padding(Padding::new(20.0));

    container(content)
        .width(Length::Fill)
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
