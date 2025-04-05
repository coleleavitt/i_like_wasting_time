use crate::data::JobStatus;

#[derive(Debug, Clone)]
pub enum Message {
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
