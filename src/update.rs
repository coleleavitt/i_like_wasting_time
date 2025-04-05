use iced::Task;

use crate::message::Message;
use crate::state::{FormState, JobTracker};

pub fn update(state: &mut JobTracker, message: Message) -> Task<Message> {
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
            if state.form.is_valid() {
                if let Some(job) = state.form.to_job() {
                    state.jobs.push(job);
                }

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
                state.edit_form = FormState::from_job(&state.jobs[index]);
                state.editing_index = Some(index);
            }
            Task::none()
        },
        Message::SaveEdit => {
            if let Some(index) = state.editing_index {
                if index < state.jobs.len() && state.edit_form.is_valid() {
                    // Update the job with the edited values
                    if let Some(job) = state.edit_form.to_job() {
                        state.jobs[index] = job;
                    }
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
