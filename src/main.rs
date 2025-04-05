use std::fmt;
use iced::widget::{button, column, container, row, scrollable, text, horizontal_rule, Space, text_input, pick_list};
use iced::{
    alignment, application, Alignment, Border, Color, Element,
    Length, Padding, Size, Theme, Task, Vector, Background, Shadow,
};

// --- Data Structures ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum JobStatus {
    Applied,
    OA,
    Interview,
    Rejected,
    Offer,
    Accepted,
    Withdrawn,
}

impl fmt::Display for JobStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                JobStatus::Applied => "Applied",
                JobStatus::OA => "OA",
                JobStatus::Interview => "Interview",
                JobStatus::Rejected => "Rejected",
                JobStatus::Offer => "Offer",
                JobStatus::Accepted => "Accepted",
                JobStatus::Withdrawn => "Withdrawn",
            }
        )
    }
}

#[derive(Debug, Clone)]
struct JobApplication {
    company: String,
    position: String,
    date_applied: String,
    status: JobStatus,
    notes: String,
    url: Option<String>,
}

// --- Form State ---
#[derive(Debug, Default, Clone)]
struct FormState {
    company: String,
    position: String,
    date_applied: String,
    notes: String,
    url: String,
    status: Option<JobStatus>,
    is_expanded: bool,
}

// --- Application State ---

#[derive(Default)]
struct JobTracker {
    jobs: Vec<JobApplication>,
    form: FormState,
    editing_index: Option<usize>,
    edit_form: FormState,
}

// --- Messages ---

#[derive(Debug, Clone)]
enum Message {
    OpenUrl(String),
    ToggleForm,
    CompanyChanged(String),
    PositionChanged(String),
    DateChanged(String),
    NotesChanged(String),
    UrlChanged(String),
    StatusSelected(JobStatus),
    AddJob,
    ResetForm,
    StartEditing(usize),
    SaveEdit,
    CancelEdit,
    DeleteJob(usize),
}

// --- Enhanced Kraken-inspired Color Palette ---

fn kraken_background_gradient() -> Background {
    Background::Color(kraken_background())
    // Note: Using color instead of gradient due to API limitations in this version
}

fn kraken_background() -> Color {
    Color::from_rgb(0.078, 0.086, 0.102) // Deep blue-black
}

fn kraken_card_bg() -> Color {
    Color::from_rgb(0.098, 0.106, 0.122) // Slightly lighter blue-black
}

fn kraken_header_bg() -> Color {
    Color::from_rgb(0.055, 0.063, 0.075) // Darker blue-black for header
}

fn kraken_text() -> Color {
    Color::from_rgb(0.88, 0.90, 0.92) // Crisp white
}

fn kraken_secondary_text() -> Color {
    Color::from_rgb(0.63, 0.65, 0.67) // Gray for less important text
}

fn kraken_highlight() -> Color {
    Color::from_rgb(0.129, 0.737, 0.514) // Kraken's green accent
}

fn kraken_highlight_hover() -> Color {
    Color::from_rgb(0.169, 0.847, 0.584) // Brighter green for hover
}

fn kraken_highlight_subtle() -> Color {
    Color::from_rgba(0.129, 0.737, 0.514, 0.12) // Very subtle green for backgrounds
}

fn kraken_negative() -> Color {
    Color::from_rgb(0.949, 0.267, 0.267) // Red for negative outcomes
}

fn kraken_negative_dark() -> Color {
    Color::from_rgb(0.649, 0.137, 0.137) // Darker red for rejected jobs
}

fn kraken_warning() -> Color {
    Color::from_rgb(0.945, 0.769, 0.059) // Yellow for warnings/edit actions
}

fn kraken_border() -> Color {
    Color::from_rgb(0.149, 0.169, 0.204) // Subtle borders
}

fn kraken_card_border() -> Color {
    Color::from_rgb(0.169, 0.189, 0.224) // Slightly brighter card borders
}

// --- Status Colors ---

fn status_color(status: JobStatus) -> Color {
    match status {
        JobStatus::Applied => Color::from_rgb(0.22, 0.51, 0.78),      // Blue
        JobStatus::OA => Color::from_rgb(0.90, 0.62, 0.0),            // Orange
        JobStatus::Interview => Color::from_rgb(0.129, 0.737, 0.514), // Kraken Green
        JobStatus::Rejected => Color::from_rgb(0.949, 0.267, 0.267),  // Kraken Red
        JobStatus::Offer => Color::from_rgb(0.608, 0.349, 0.714),     // Purple
        JobStatus::Accepted => Color::from_rgb(0.129, 0.737, 0.514),  // Kraken Green
        JobStatus::Withdrawn => Color::from_rgb(0.5, 0.5, 0.5),       // Gray
    }
}

// --- Enhanced Custom Styling ---

fn main_background(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(kraken_background_gradient()),
        text_color: Some(kraken_text()),
        ..container::Style::default()
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

fn table_header_style(_theme: &Theme) -> container::Style {
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

fn card_style(status: JobStatus, _theme: &Theme) -> container::Style {
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

fn status_badge_style(status: JobStatus) -> container::Style {
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

fn link_button_style(_theme: &Theme, status: button::Status) -> button::Style {
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

fn edit_button_style(_theme: &Theme, status: button::Status) -> button::Style {
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

fn delete_button_style(_theme: &Theme, status: button::Status) -> button::Style {
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

fn primary_button_style(_theme: &Theme, status: button::Status) -> button::Style {
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

fn save_button_style(_theme: &Theme, status: button::Status) -> button::Style {
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

fn secondary_button_style(_theme: &Theme, status: button::Status) -> button::Style {
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

// Fixed function to match the actual text_input::Style structure
fn input_style(_theme: &Theme, _status: text_input::Status) -> text_input::Style {
    text_input::Style {
        background: Background::Color(Color::from_rgb(0.11, 0.12, 0.14)),
        border: Border {
            color: kraken_border(),
            width: 1.0,
            radius: 6.0.into(),
        },
        // Updated to use the correct field names
        icon: kraken_secondary_text(),
        placeholder: kraken_secondary_text(),
        value: kraken_text(),
        selection: kraken_highlight_subtle(),
    }
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

// --- Custom Text Renderer ---

fn company_text_style(status: JobStatus) -> impl Fn(&Theme) -> text::Style {
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

fn position_text_style(status: JobStatus) -> impl Fn(&Theme) -> text::Style {
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

// --- Application Logic ---

fn update(state: &mut JobTracker, message: Message) -> Task<Message> {
    match message {
        Message::OpenUrl(url) => {
            if webbrowser::open(&url).is_err() {
                eprintln!("Failed to open URL: {}", url);
            }
            Task::none()
        },
        Message::ToggleForm => {
            state.form.is_expanded = !state.form.is_expanded;
            Task::none()
        },
        Message::CompanyChanged(value) => {
            if state.editing_index.is_some() {
                state.edit_form.company = value;
            } else {
                state.form.company = value;
            }
            Task::none()
        },
        Message::PositionChanged(value) => {
            if state.editing_index.is_some() {
                state.edit_form.position = value;
            } else {
                state.form.position = value;
            }
            Task::none()
        },
        Message::DateChanged(value) => {
            if state.editing_index.is_some() {
                state.edit_form.date_applied = value;
            } else {
                state.form.date_applied = value;
            }
            Task::none()
        },
        Message::NotesChanged(value) => {
            if state.editing_index.is_some() {
                state.edit_form.notes = value;
            } else {
                state.form.notes = value;
            }
            Task::none()
        },
        Message::UrlChanged(value) => {
            if state.editing_index.is_some() {
                state.edit_form.url = value;
            } else {
                state.form.url = value;
            }
            Task::none()
        },
        Message::StatusSelected(status) => {
            if state.editing_index.is_some() {
                state.edit_form.status = Some(status);
            } else {
                state.form.status = Some(status);
            }
            Task::none()
        },
        Message::AddJob => {
            // Only add if required fields are filled
            if !state.form.company.is_empty() &&
                !state.form.position.is_empty() &&
                !state.form.date_applied.is_empty() &&
                state.form.status.is_some() {

                state.jobs.push(JobApplication {
                    company: state.form.company.clone(),
                    position: state.form.position.clone(),
                    date_applied: state.form.date_applied.clone(),
                    status: state.form.status.unwrap(),
                    notes: state.form.notes.clone(),
                    url: if state.form.url.is_empty() { None } else { Some(state.form.url.clone()) },
                });

                // Reset the form after adding
                state.form = FormState {
                    is_expanded: true, // Keep form open for multiple adds
                    ..Default::default()
                };
            }
            Task::none()
        },
        Message::ResetForm => {
            state.form = FormState {
                is_expanded: true, // Keep form open
                ..Default::default()
            };
            Task::none()
        },
        Message::StartEditing(index) => {
            if index < state.jobs.len() {
                // Set up the edit form with the current job data
                let job = &state.jobs[index];
                state.edit_form = FormState {
                    company: job.company.clone(),
                    position: job.position.clone(),
                    date_applied: job.date_applied.clone(),
                    notes: job.notes.clone(),
                    url: job.url.clone().unwrap_or_default(),
                    status: Some(job.status),
                    is_expanded: true,
                };
                state.editing_index = Some(index);
            }
            Task::none()
        },
        Message::SaveEdit => {
            if let Some(index) = state.editing_index {
                if index < state.jobs.len() &&
                    !state.edit_form.company.is_empty() &&
                    !state.edit_form.position.is_empty() &&
                    !state.edit_form.date_applied.is_empty() &&
                    state.edit_form.status.is_some() {

                    // Update the job with the edited values
                    state.jobs[index] = JobApplication {
                        company: state.edit_form.company.clone(),
                        position: state.edit_form.position.clone(),
                        date_applied: state.edit_form.date_applied.clone(),
                        status: state.edit_form.status.unwrap(),
                        notes: state.edit_form.notes.clone(),
                        url: if state.edit_form.url.is_empty() { None } else { Some(state.edit_form.url.clone()) },
                    };
                }

                // Clear editing state
                state.editing_index = None;
                state.edit_form = FormState::default();
            }
            Task::none()
        },
        Message::CancelEdit => {
            // Clear editing state without saving changes
            state.editing_index = None;
            state.edit_form = FormState::default();
            Task::none()
        },
        Message::DeleteJob(index) => {
            if index < state.jobs.len() {
                state.jobs.remove(index);

                // If we were editing this index, clear the editing state
                if state.editing_index == Some(index) {
                    state.editing_index = None;
                    state.edit_form = FormState::default();
                }
                // If we were editing an index after the deleted one, adjust the index
                else if let Some(editing_index) = state.editing_index {
                    if editing_index > index {
                        state.editing_index = Some(editing_index - 1);
                    }
                }
            }
            Task::none()
        },
    }
}

// Enhanced View function
fn view(state: &JobTracker) -> Element<Message> {
    // App header with upgraded Kraken-style
    let app_header = container(
        row![
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
            .padding(Padding::new(20.0))
    )
        .width(Length::Fill)
        .style(header_style);

    // Form for adding new jobs
    let add_form = if state.form.is_expanded && state.editing_index.is_none() {
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
                    .style(|_theme, _status| {
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
                    })
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
    } else {
        container(Space::with_height(0))
            .width(Length::Fill)
    };

    // Enhanced table header
    let table_header = container(
        row![
            text("COMPANY").size(13).width(Length::FillPortion(2))
                .style(|_| text::Style { color: Some(kraken_secondary_text()) }),
            text("POSITION").size(13).width(Length::FillPortion(3))
                .style(|_| text::Style { color: Some(kraken_secondary_text()) }),
            text("APPLIED").size(13).width(Length::FillPortion(2))
                .style(|_| text::Style { color: Some(kraken_secondary_text()) }),
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
        .style(table_header_style);

    // Function to create the edit form for a job
    let create_edit_form = |index: usize, form: &FormState| {
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
                    .style(|_theme, _status| {
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
                    })
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

            // Action buttons
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
    };

    // Create job rows with enhanced styling
    let job_rows = state.jobs.iter().enumerate().fold(
        column![].spacing(12),
        |col, (index, job)| {
            // If this index is being edited, show the edit form instead of the job card
            if state.editing_index == Some(index) {
                col.push(create_edit_form(index, &state.edit_form))
            } else {
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

                // Action buttons for editing/deleting
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

                // Add row content to column
                let content = row_content
                    .spacing(15)
                    .align_y(Alignment::Center)
                    .padding(Padding::new(18.0));

                // Push a card with the job application
                col.push(
                    container(content)
                        .width(Length::Fill)
                        .style(move |theme| card_style(status, theme))
                )
            }
        },
    );

    // Put it all together with enhanced layout
    let content = column![
        app_header,
        horizontal_rule(1).style(|_| {
            iced::widget::rule::Style {
                color: kraken_border(),
                width: 1,
                radius: 0.0.into(),
                fill_mode: iced::widget::rule::FillMode::Full,
            }
        }),
        add_form,
        container(
            column![
                table_header,
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

// --- Helper Functions ---

fn load_job_data() -> Vec<JobApplication> {
    vec![
        JobApplication {
            company: "Accenture Federal Services".to_string(),
            position: "RedHat Linux Administrator".to_string(),
            date_applied: "April 3, 2025".to_string(),
            status: JobStatus::Applied,
            notes: "Secret clearance required; $85K-$138K salary range; Houston, TX location".to_string(),
            url: Some("https://job-boards.greenhouse.io/accenturefederalservices/jobs/4441555006".to_string()),
        },
        JobApplication {
            company: "NASA/Amentum".to_string(),
            position: "FSL Linux Systems Administrator".to_string(),
            date_applied: "April 3, 2025".to_string(),
            status: JobStatus::Applied,
            notes: "RHEL experience required; supports ISS and other space programs".to_string(),
            url: Some("https://jacobs.taleo.net/careersection/ex/jobdetail.ftl?job=ADV000ABM".to_string()),
        },
        JobApplication {
            company: "Google".to_string(),
            position: "Site Reliability Engineer".to_string(),
            date_applied: "April 4, 2025".to_string(),
            status: JobStatus::OA,
            notes: "Online assessment received; deadline is April 10".to_string(),
            url: Some("https://careers.google.com/jobs/results/".to_string()),
        },
        JobApplication {
            company: "Microsoft".to_string(),
            position: "Cloud Solutions Architect".to_string(),
            date_applied: "March 28, 2025".to_string(),
            status: JobStatus::Interview,
            notes: "First interview scheduled for April 10, 2025".to_string(),
            url: Some("https://careers.microsoft.com/".to_string()),
        },
        JobApplication {
            company: "Amazon".to_string(),
            position: "Systems Engineer".to_string(),
            date_applied: "March 15, 2025".to_string(),
            status: JobStatus::Rejected,
            notes: "Received rejection email on April 1, 2025".to_string(),
            url: None,
        },
        JobApplication {
            company: "LinkedIn".to_string(),
            position: "DevOps Engineer".to_string(),
            date_applied: "March 10, 2025".to_string(),
            status: JobStatus::Offer,
            notes: "Offer received: $145K/year + benefits. Need to respond by April 15".to_string(),
            url: Some("https://linkedin.com/jobs/".to_string()),
        },
        JobApplication {
            company: "Salesforce".to_string(),
            position: "Infrastructure Engineer".to_string(),
            date_applied: "February 25, 2025".to_string(),
            status: JobStatus::Accepted,
            notes: "Accepted offer on April 2. Start date: May 1, 2025".to_string(),
            url: Some("https://salesforce.com/careers/".to_string()),
        },
        JobApplication {
            company: "Twitter".to_string(),
            position: "Backend Developer".to_string(),
            date_applied: "March 20, 2025".to_string(),
            status: JobStatus::Withdrawn,
            notes: "Withdrew application after receiving better offer elsewhere".to_string(),
            url: Some("https://twitter.com/careers/".to_string()),
        },
    ]
}

fn main() -> iced::Result {
    // Using the new application API with Wayland compatibility
    application("Job Application Tracker", update, view)
        .theme(|_| Theme::Dark)
        .window_size(Size::new(1200.0, 700.0))
        .antialiasing(true)
        .run_with(|| {
            (
                JobTracker {
                    jobs: load_job_data(),
                    form: FormState::default(),
                    editing_index: None,
                    edit_form: FormState::default(),
                },
                Task::none(),
            )
        })
}
