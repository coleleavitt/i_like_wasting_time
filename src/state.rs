use crate::data::{JobApplication, JobStatus};

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

    pub fn is_valid(&self) -> bool {
        !self.company.is_empty()
            && !self.position.is_empty()
            && !self.date_applied.is_empty()
            && self.status.is_some()
    }

    pub fn to_job(&self) -> Option<JobApplication> {
        if self.is_valid() {
            Some(JobApplication::new(
                self.company.clone(),
                self.position.clone(),
                self.date_applied.clone(),
                self.status.unwrap(),
                self.notes.clone(),
                if self.url.is_empty() { None } else { Some(self.url.clone()) },
            ))
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct JobTracker {
    pub jobs: Vec<JobApplication>,
    pub form: FormState,
    pub editing_index: Option<usize>,
    pub edit_form: FormState,
}

impl JobTracker {
    pub fn new(jobs: Vec<JobApplication>) -> Self {
        Self {
            jobs,
            form: FormState::default(),
            editing_index: None,
            edit_form: FormState::default(),
        }
    }
}
