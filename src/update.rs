use chrono::Local;
use iced::Task;

use crate::message::Message;
use crate::state::{FormState, JobTracker};
use crate::storage;

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
                state.has_unsaved_changes = true;  // Mark as having unsaved changes
            } else {
                state.form.company = value;
            }
            Task::none()
        },
        Message::PositionChanged(value) => {
            if state.editing_index.is_some() {
                state.edit_form.position = value;
                state.has_unsaved_changes = true;
            } else {
                state.form.position = value;
            }
            Task::none()
        },
        Message::DateChanged(value) => {
            if state.editing_index.is_some() {
                state.edit_form.date_applied = value;
                state.has_unsaved_changes = true;
            } else {
                state.form.date_applied = value;
            }
            Task::none()
        },
        Message::NotesChanged(value) => {
            if state.editing_index.is_some() {
                state.edit_form.notes = value;
                state.has_unsaved_changes = true;
            } else {
                state.form.notes = value;
            }
            Task::none()
        },
        Message::UrlChanged(value) => {
            if state.editing_index.is_some() {
                state.edit_form.url = value;
                state.has_unsaved_changes = true;
            } else {
                state.form.url = value;
            }
            Task::none()
        },
        Message::StatusSelected(status) => {
            if state.editing_index.is_some() {
                state.edit_form.status = Some(status);
                state.has_unsaved_changes = true;
            } else {
                state.form.status = Some(status);
            }
            Task::none()
        },
        Message::AddJob => {
            // Only add if required fields are filled
            if state.form.is_valid() {
                if let Some(mut job) = state.form.to_job() {
                    // Add timestamp
                    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                    job.last_updated = Some(now);

                    state.jobs.push(job);
                    state.has_unsaved_changes = true;  // Mark as changed after adding job

                    // Save after adding a job
                    state.save();
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
                    if let Some(mut job) = state.edit_form.to_job() {
                        // Update timestamp
                        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                        job.last_updated = Some(now);

                        state.jobs[index] = job;
                        state.has_unsaved_changes = true;  // Mark as changed after editing

                        // Save after editing
                        state.save();
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
                // Create backup before deletion
                let _ = storage::backup_data();

                state.jobs.remove(index);
                state.has_unsaved_changes = true;  // Mark as changed after deletion

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

                // Save after deleting
                state.save();
            }
            Task::none()
        },
        Message::SaveData => {
            state.save();
            Task::none()
        },
        Message::LoadData => {
            match storage::load_jobs() {
                Ok(jobs) => {
                    state.jobs = jobs;
                    state.error_message = None;
                    state.has_unsaved_changes = false;  // Reset after loading
                },
                Err(err) => {
                    state.error_message = Some(format!("Error loading data: {}", err));
                }
            }
            Task::none()
        },
        Message::ErrorDismissed => {
            state.error_message = None;
            Task::none()
        },
        // Filter-related message handlers (don't affect saved state)
        Message::SearchQueryChanged(query) => {
            state.search_query = query;
            Task::none()
        },
        Message::FilterStatusChanged(status) => {
            state.filter_status = Some(status);
            Task::none()
        },
        Message::ClearFilters => {
            state.search_query = String::new();
            state.filter_status = None;
            Task::none()
        },
    }
}
