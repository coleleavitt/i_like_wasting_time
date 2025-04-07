use iced::{
    widget::{button, container, row, text},
    alignment, Alignment, Border, Element, Length, Padding, Shadow, Theme, Color
};

use crate::data::{JobApplication, JobStatus};
use crate::message::Message;
use crate::theme::*;
use crate::ui::common::*;
use crate::state::{SortColumn, SortOrder};

pub fn job_row(index: usize, job: &JobApplication) -> Element<'_, Message, Theme> {
    let status = job.status;

    // Enhanced status badge with glow effect for special statuses
    let status_badge = container(
        text(job.status.to_string())
            .size(13)
            .style(|_| text::Style { color: Some(Color::WHITE) })
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center),
    )
        .padding(Padding::from([6, 12]))
        .style(move |_| status_badge_style(job.status))
        .width(Length::FillPortion(1));

    // Action buttons for editing
    let action_buttons = row![
        button(text("Edit").size(13))
            .style(edit_button_style)
            .padding(Padding::from([5, 10]))
            .on_press(Message::StartEditing(index)),
    ]
        .spacing(8)
        .align_y(Alignment::Center)
        .width(Length::FillPortion(1));

    // Enhanced row design with special styling for rejected/withdrawn jobs
    let row_content = if let Some(url) = &job.url {
        row![
            text(&job.company)
                .size(14)
                .width(Length::FillPortion(2))
                .style(company_text_style(status)),
            button(
                text(&job.position)
                    .size(14)
            )
            .style(link_button_style)
            .padding(Padding::from([5, 10]))
            .on_press(Message::OpenUrl(url.clone()))
            .width(Length::FillPortion(3)),
            text(&job.date_applied)
                .size(14)
                .width(Length::FillPortion(2))
                .style(|_| text::Style { color: Some(kraken_secondary_text()) }),
            status_badge,
            text(&job.notes)
                .size(14)
                .width(Length::FillPortion(4))
                .style(|_| text::Style { color: Some(kraken_text()) }),
            action_buttons,
        ]
    } else {
        row![
            text(&job.company)
                .size(14)
                .width(Length::FillPortion(2))
                .style(company_text_style(status)),
            text(&job.position)
                .size(14)
                .width(Length::FillPortion(3))
                .style(position_text_style(status)),
            text(&job.date_applied)
                .size(14)
                .width(Length::FillPortion(2))
                .style(|_| text::Style { color: Some(kraken_secondary_text()) }),
            status_badge,
            text(&job.notes)
                .size(14)
                .width(Length::FillPortion(4))
                .style(move |_| text::Style {
                    color: Some(if status == JobStatus::Rejected || status == JobStatus::Withdrawn {
                        kraken_secondary_text()
                    } else {
                        kraken_text()
                    })
                }),
            action_buttons,
        ]
    };

    // Format the row content
    let content = row_content
        .spacing(15)
        .align_y(Alignment::Center)
        .padding(Padding::new(18.0));

    // Return a container with the job application
    container(content)
        .width(Length::Fill)
        .style(move |theme| card_style(status, theme))
        .into()
}

pub fn table_header(sort_column: SortColumn, sort_order: SortOrder) -> container::Container<'static, Message, Theme> {
    // Company header with sorting triangle
    let company_text = row![
        text("COMPANY").size(13),
        text(match (sort_column, sort_order) {
            (SortColumn::Company, SortOrder::Ascending) => "▲",
            (SortColumn::Company, SortOrder::Descending) => "▼",
            _ => "⋮" // Neutral state
        }).size(9)
    ]
        .spacing(5);

    // Use button instead of container for interactivity
    let company_header = button(company_text)
        .width(Length::FillPortion(2))
        .style(|_theme, _status| button::Style {
            text_color: kraken_secondary_text(),
            background: None,
            border: Border::default(),
            shadow: Shadow::default(),
        })
        .on_press(Message::SortBy(SortColumn::Company));

    // Applied date header with sorting triangle
    let applied_text = row![
        text("APPLIED").size(13),
        text(match (sort_column, sort_order) {
            (SortColumn::DateApplied, SortOrder::Ascending) => "▲",
            (SortColumn::DateApplied, SortOrder::Descending) => "▼",
            _ => "⋮" // Neutral state
        }).size(9)
    ]
        .spacing(5);

    // Use button instead of container for interactivity
    let applied_header = button(applied_text)
        .width(Length::FillPortion(2))
        .style(|_theme, _status| button::Style {
            text_color: kraken_secondary_text(),
            background: None,
            border: Border::default(),
            shadow: Shadow::default(),
        })
        .on_press(Message::SortBy(SortColumn::DateApplied));

    container(
        row![
            company_header,
            text("POSITION").size(13).width(Length::FillPortion(3))
                .style(|_| text::Style { color: Some(kraken_secondary_text()) }),
            applied_header,
            text("STATUS").size(13).width(Length::FillPortion(1))
                .style(|_| text::Style { color: Some(kraken_secondary_text()) }),
            text("NOTES").size(13).width(Length::FillPortion(4))
                .style(|_| text::Style { color: Some(kraken_secondary_text()) }),
            text("ACTIONS").size(13).width(Length::FillPortion(1))
                .style(|_| text::Style { color: Some(kraken_secondary_text()) }),
        ]
            .spacing(15)
            .padding(Padding::new(15.0))
    )
        .width(Length::Fill)
        .style(table_header_style)
}
