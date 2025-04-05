use iced::{
    widget::{button, container, text, text_input},
    Border, Background, Color, Shadow, Theme, Vector,
};
use crate::theme::*;
use crate::data::JobStatus;

pub fn main_background(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(kraken_background_gradient()),
        text_color: Some(kraken_text()),
        ..container::Style::default()
    }
}

pub fn table_header_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(kraken_header_bg())),
        text_color: Some(kraken_secondary_text()),
        border: Border {
            color: kraken_border(),
            width: 0.0,
            radius: 6.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 2.0,
        },
        ..container::Style::default()
    }
}

pub fn card_style(status: JobStatus, _theme: &Theme) -> container::Style {
    // Base style for all cards
    let mut style = container::Style {
        background: Some(Background::Color(kraken_card_bg())),
        text_color: Some(kraken_text()),
        border: Border {
            color: kraken_card_border(),
            width: 1.0,
            radius: 8.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 8.0,
        },
    };

    // Special styling for rejected/withdrawn applications
    match status {
        JobStatus::Rejected => {
            style.border.color = kraken_negative_dark();
            style.border.width = 1.0;
            // Add a subtle red tint to the background
            style.background = Some(Background::Color(Color::from_rgb(0.13, 0.106, 0.112)));
        },
        JobStatus::Withdrawn => {
            style.background = Some(Background::Color(Color::from_rgb(0.088, 0.096, 0.112)));
            style.border.color = Color::from_rgb(0.3, 0.3, 0.3);
        },
        JobStatus::Accepted => {
            // Add subtle green tint to accepted jobs
            style.background = Some(Background::Color(Color::from_rgb(0.088, 0.116, 0.112)));
            style.border.color = Color::from_rgba(0.129, 0.737, 0.514, 0.5);
            style.border.width = 1.5;
        },
        JobStatus::Offer => {
            // Add subtle purple tint to offers
            style.background = Some(Background::Color(Color::from_rgb(0.108, 0.096, 0.132)));
            style.border.color = Color::from_rgba(0.608, 0.349, 0.714, 0.5);
            style.border.width = 1.5;
        },
        _ => {}
    }

    style
}

pub fn status_badge_style(status: JobStatus) -> container::Style {
    let color = status_color(status);

    // Common style for all badges
    let mut style = container::Style {
        background: Some(Background::Color(color)),
        text_color: Some(Color::WHITE),
        border: Border {
            color,
            width: 0.0,
            radius: 4.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 4.0,
        },
        ..container::Style::default()
    };

    // Special styling for specific statuses
    match status {
        JobStatus::Accepted => {
            style.shadow = Shadow {
                color: Color::from_rgba(0.129, 0.737, 0.514, 0.6),
                offset: Vector::new(0.0, 0.0),
                blur_radius: 8.0,
            };
        },
        JobStatus::Rejected => {
            style.shadow = Shadow {
                color: Color::from_rgba(0.949, 0.267, 0.267, 0.4),
                offset: Vector::new(0.0, 0.0),
                blur_radius: 6.0,
            };
        },
        JobStatus::Offer => {
            style.shadow = Shadow {
                color: Color::from_rgba(0.608, 0.349, 0.714, 0.5),
                offset: Vector::new(0.0, 0.0),
                blur_radius: 8.0,
            };
        },
        _ => {}
    }

    style
}

// Button Styles
pub fn link_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(kraken_highlight_subtle())),
            text_color: kraken_highlight_hover(),
            border: Border {
                color: kraken_highlight(),
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.129, 0.737, 0.514, 0.2),
                offset: Vector::new(0.0, 0.0),
                blur_radius: 6.0,
            },
        },
        _ => button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: kraken_highlight(),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 4.0.into(),
            },
            shadow: Shadow::default(),
        },
    }
}

pub fn edit_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgba(0.945, 0.769, 0.059, 0.15))),
            text_color: Color::from_rgb(0.945, 0.769, 0.059),
            border: Border {
                color: Color::from_rgb(0.945, 0.769, 0.059),
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.945, 0.769, 0.059, 0.2),
                offset: Vector::new(0.0, 0.0),
                blur_radius: 6.0,
            },
        },
        _ => button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: Color::from_rgb(0.945, 0.769, 0.059),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 4.0.into(),
            },
            shadow: Shadow::default(),
        },
    }
}

pub fn delete_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgba(0.949, 0.267, 0.267, 0.15))),
            text_color: kraken_negative(),
            border: Border {
                color: kraken_negative(),
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.949, 0.267, 0.267, 0.2),
                offset: Vector::new(0.0, 0.0),
                blur_radius: 6.0,
            },
        },
        _ => button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: kraken_negative(),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 4.0.into(),
            },
            shadow: Shadow::default(),
        },
    }
}

pub fn primary_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(kraken_highlight_hover())),
            text_color: Color::WHITE,
            border: Border {
                color: kraken_highlight_hover(),
                width: 0.0,
                radius: 6.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.129, 0.737, 0.514, 0.4),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
        },
        _ => button::Style {
            background: Some(Background::Color(kraken_highlight())),
            text_color: Color::WHITE,
            border: Border {
                color: kraken_highlight(),
                width: 0.0,
                radius: 6.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.129, 0.737, 0.514, 0.2),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 4.0,
            },
        },
    }
}

pub fn save_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    let mut style = primary_button_style(_theme, status);
    match status {
        button::Status::Hovered => {
            style.background = Some(Background::Color(Color::from_rgb(0.22, 0.69, 0.9)));
            style.border.color = Color::from_rgb(0.22, 0.69, 0.9);
            style.shadow.color = Color::from_rgba(0.22, 0.69, 0.9, 0.4);
        },
        _ => {
            style.background = Some(Background::Color(Color::from_rgb(0.18, 0.59, 0.8)));
            style.border.color = Color::from_rgb(0.18, 0.59, 0.8);
            style.shadow.color = Color::from_rgba(0.18, 0.59, 0.8, 0.2);
        }
    }
    style
}

pub fn secondary_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.18, 0.2, 0.22))),
            text_color: kraken_text(),
            border: Border {
                color: kraken_border(),
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 3.0,
            },
        },
        _ => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.12, 0.14, 0.16))),
            text_color: kraken_text(),
            border: Border {
                color: kraken_border(),
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 2.0,
            },
        },
    }
}

pub fn input_style(_theme: &Theme, _status: text_input::Status) -> text_input::Style {
    text_input::Style {
        background: Background::Color(Color::from_rgb(0.11, 0.12, 0.14)),
        border: Border {
            color: kraken_border(),
            width: 1.0,
            radius: 6.0.into(),
        },
        icon: kraken_secondary_text(),
        placeholder: kraken_secondary_text(),
        value: kraken_text(),
        selection: kraken_highlight_subtle(),
    }
}

pub fn company_text_style(status: JobStatus) -> impl Fn(&Theme) -> text::Style {
    move |_theme| {
        let base_color = kraken_text();

        // Different styles for different statuses
        let color = match status {
            JobStatus::Rejected | JobStatus::Withdrawn => kraken_secondary_text(),
            _ => base_color,
        };

        text::Style {
            color: Some(color),
        }
    }
}

pub fn position_text_style(status: JobStatus) -> impl Fn(&Theme) -> text::Style {
    move |_theme| {
        let base_color = kraken_text();

        // Different styles for different statuses
        let color = match status {
            JobStatus::Rejected | JobStatus::Withdrawn => kraken_secondary_text(),
            _ => base_color,
        };

        text::Style {
            color: Some(color),
        }
    }
}
