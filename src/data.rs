use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobStatus {
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
pub struct JobApplication {
    pub company: String,
    pub position: String,
    pub date_applied: String,
    pub status: JobStatus,
    pub notes: String,
    pub url: Option<String>,
}

impl JobApplication {
    pub fn new(
        company: String,
        position: String,
        date_applied: String,
        status: JobStatus,
        notes: String,
        url: Option<String>,
    ) -> Self {
        Self {
            company,
            position,
            date_applied,
            status,
            notes,
            url,
        }
    }

    pub fn load_sample_data() -> Vec<Self> {
        vec![
            Self::new(
                "Accenture Federal Services".to_string(),
                "RedHat Linux Administrator".to_string(),
                "April 3, 2025".to_string(),
                JobStatus::Applied,
                "Secret clearance required; $85K-$138K salary range; Houston, TX location".to_string(),
                Some("https://job-boards.greenhouse.io/accenturefederalservices/jobs/4441555006".to_string()),
            ),
            Self::new(
                "NASA/Amentum".to_string(),
                "FSL Linux Systems Administrator".to_string(),
                "April 3, 2025".to_string(),
                JobStatus::Applied,
                "RHEL experience required; supports ISS and other space programs".to_string(),
                Some("https://jacobs.taleo.net/careersection/ex/jobdetail.ftl?job=ADV000ABM".to_string()),
            ),
            Self::new(
                "Google".to_string(),
                "Site Reliability Engineer".to_string(),
                "April 4, 2025".to_string(),
                JobStatus::OA,
                "Online assessment received; deadline is April 10".to_string(),
                Some("https://careers.google.com/jobs/results/".to_string()),
            ),
            Self::new(
                "Microsoft".to_string(),
                "Cloud Solutions Architect".to_string(),
                "March 28, 2025".to_string(),
                JobStatus::Interview,
                "First interview scheduled for April 10, 2025".to_string(),
                Some("https://careers.microsoft.com/".to_string()),
            ),
            Self::new(
                "Amazon".to_string(),
                "Systems Engineer".to_string(),
                "March 15, 2025".to_string(),
                JobStatus::Rejected,
                "Received rejection email on April 1, 2025".to_string(),
                None,
            ),
            Self::new(
                "LinkedIn".to_string(),
                "DevOps Engineer".to_string(),
                "March 10, 2025".to_string(),
                JobStatus::Offer,
                "Offer received: $145K/year + benefits. Need to respond by April 15".to_string(),
                Some("https://linkedin.com/jobs/".to_string()),
            ),
            Self::new(
                "Salesforce".to_string(),
                "Infrastructure Engineer".to_string(),
                "February 25, 2025".to_string(),
                JobStatus::Accepted,
                "Accepted offer on April 2. Start date: May 1, 2025".to_string(),
                Some("https://salesforce.com/careers/".to_string()),
            ),
            Self::new(
                "Twitter".to_string(),
                "Backend Developer".to_string(),
                "March 20, 2025".to_string(),
                JobStatus::Withdrawn,
                "Withdrew application after receiving better offer elsewhere".to_string(),
                Some("https://twitter.com/careers/".to_string()),
            ),
        ]
    }
}
