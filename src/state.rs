use crate::data::{JobApplication, JobStatus};
use crate::storage;
use chrono::Local;

#[derive(Debug, Default, Clone)]
pub struct FormState {
    pub company: String,
    pub position: String,
    pub date_applied: String,
    pub notes: String,
    pub url: String,
    pub status: Option<JobStatus>,
    pub is_expanded: bool,
}

impl FormState {
    // Check if form has all required fields filled
    pub fn is_valid(&self) -> bool {
        !self.company.is_empty() &&
            !self.position.is_empty() &&
            !self.date_applied.is_empty() &&
            self.status.is_some()
    }

    // Convert form data to a JobApplication
    pub fn to_job(&self) -> Option<JobApplication> {
        if !self.is_valid() {
            return None;
        }

        Some(JobApplication {
            company: self.company.clone(),
            position: self.position.clone(),
            date_applied: self.date_applied.clone(),
            status: self.status.unwrap(),
            notes: self.notes.clone(),
            url: if self.url.is_empty() { None } else { Some(self.url.clone()) },
            last_updated: Some(Local::now().format("%Y-%m-%d %H:%M:%S").to_string()),
        })
    }

    // Create a form state from a JobApplication
    pub fn from_job(job: &JobApplication) -> Self {
        Self {
            company: job.company.clone(),
            position: job.position.clone(),
            date_applied: job.date_applied.clone(),
            notes: job.notes.clone(),
            url: job.url.clone().unwrap_or_default(),
            status: Some(job.status),
            is_expanded: true,
        }
    }
}

#[derive(Default)]
pub struct JobTracker {
    pub jobs: Vec<JobApplication>,
    pub form: FormState,
    pub editing_index: Option<usize>,
    pub edit_form: FormState,
    pub error_message: Option<String>,
    pub last_saved: Option<String>,
    pub search_query: String,
    pub filter_status: Option<JobStatus>,
    pub has_unsaved_changes: bool,  // New field to track actual data changes
}

impl JobTracker {
    pub fn new() -> Self {
        // Load jobs from storage, falling back to empty if error
        let jobs = storage::load_jobs().unwrap_or_else(|err| {
            eprintln!("Error loading jobs: {}", err);
            Vec::new()
        });

        Self {
            jobs,
            has_unsaved_changes: false,
            ..Default::default()
        }
    }

    pub fn save(&mut self) {
        // Update last_saved timestamp
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        match storage::save_jobs(&self.jobs) {
            Ok(_) => {
                self.last_saved = Some(now);
                self.error_message = None;
                self.has_unsaved_changes = false;  // Reset flag after saving
            }
            Err(err) => {
                self.error_message = Some(format!("Error saving: {}", err));
                eprintln!("Error saving jobs: {}", err);
            }
        }
    }

    // Filter jobs based on current search query and status filter
    pub fn filtered_jobs(&self) -> Vec<&JobApplication> {
        self.jobs.iter()
            .filter(|job| {
                // Status filter
                let status_match = match self.filter_status {
                    None | Some(JobStatus::All) => true,
                    Some(status) => job.status == status,
                };

                // Text search
                let search_match = self.search_query.is_empty() ||
                    job.company.to_lowercase().contains(&self.search_query.to_lowercase()) ||
                    job.position.to_lowercase().contains(&self.search_query.to_lowercase()) ||
                    job.notes.to_lowercase().contains(&self.search_query.to_lowercase());

                status_match && search_match
            })
            .collect()
    }
}