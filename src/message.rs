#[derive(Debug, Clone)]
pub enum Message {
    OpenUrl(String),
    ToggleForm,
    CompanyChanged(String),
    PositionChanged(String),
    DateChanged(String),
    NotesChanged(String),
    UrlChanged(String),
    StatusSelected(crate::data::JobStatus),
    AddJob,
    ResetForm,
    StartEditing(usize),
    SaveEdit,
    CancelEdit,
    DeleteJob(usize),
    SaveData,
    LoadData,
    // ExportToCSV,  // Optional additional feature
    // ImportFromCSV,  // Optional additional feature
    ErrorDismissed,
    // New filter-related messages
    SearchQueryChanged(String),
    FilterStatusChanged(crate::data::JobStatus),
    ClearFilters,
}
