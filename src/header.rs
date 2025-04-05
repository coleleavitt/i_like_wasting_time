use iced::widget::{button, container, row, text, Space};
use iced::{Alignment, Element, Length, Padding};

use crate::message::Message;
use crate::theme;

pub fn view_header(is_form_expanded: bool, job_count: usize) -> Element<'static, Message> {
    container(
        row![
            text("JOB TRACKER by Cole Leavitt")
                .size(24)
                .style(|_| text::Style { color: Some(theme::kraken_text()) }),
            Space::with_width(Length::Fill),
            button(
                text(if is_form_expanded { "Hide Form" } else { "Add New Job" })
                    .size(14)
            )
            .style(theme::toggle_form_button_style)
            .padding(Padding::from([8, 15]))
            .on_press(Message::ToggleForm),
            text(format!("{} APPLICATIONS", job_count))
                .size(14)
                .style(|_| text::Style { color: Some(theme::kraken_secondary_text()) }),
        ]
            .spacing(15)
            .align_y(Alignment::Center)
            .padding(Padding::new(20.0)),
    )
        .width(Length::Fill)
        .style(theme::header_style)
        .into()
}

pub fn view_table_header() -> Element<'static, Message> {
    container(
        row![
            text("COMPANY").size(13).width(Length::FillPortion(2))
                .style(|_| text::Style { color: Some(theme::kraken_secondary_text()) }),
            text("POSITION").size(13).width(Length::FillPortion(3))
                .style(|_| text::Style { color: Some(theme::kraken_secondary_text()) }),
            text("APPLIED").size(13).width(Length::FillPortion(2))
                .style(|_| text::Style { color: Some(theme::kraken_secondary_text()) }),
            text("STATUS").size(13).width(Length::FillPortion(1)) // Adjusted portion slightly
                .style(|_| text::Style { color: Some(theme::kraken_secondary_text()) }),
            text("NOTES").size(13).width(Length::FillPortion(4))
                .style(|_| text::Style { color: Some(theme::kraken_secondary_text()) }),
            text("ACTIONS").size(13).width(Length::FillPortion(1)) // Adjusted portion slightly
                .style(|_| text::Style { color: Some(theme::kraken_secondary_text()) }),
        ]
            .spacing(15)
            .padding(Padding::new(15.0))
    )
        .width(Length::Fill)
        .style(theme::table_header_style)
        .into()
}