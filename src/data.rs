use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobStatus {
    Applied,
    OA,
    Interview,
    Rejected,
    Offer,
    Accepted,
    Withdrawn,
    All, // New variant for filtering all statuses
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
                JobStatus::All => "All", // Add display for "All" status
            }
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobApplication {
    pub company: String,
    pub position: String,
    pub date_applied: String,
    pub status: JobStatus,
    pub notes: String,
    pub url: Option<String>,
    #[serde(default)]
    pub last_updated: Option<String>,  // Add a field to track updates
}
