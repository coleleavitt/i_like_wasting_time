// Add this attribute at the top of the file to address naming convention warnings
#![allow(non_snake_case)]

use anyhow::{Result, anyhow};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct GoogleCSE {
    engine_id: String,
    api_key: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub content: String,
    pub visibleUrl: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CSECursor {
    pub resultCount: String,
    pub estimatedResultCount: String,
    pub searchResultTime: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CSEResponse {
    pub cursor: CSECursor,
    pub results: Vec<SearchResult>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResponse {
    pub items: Option<Vec<SearchResult>>,
    pub search_information: Option<HashMap<String, serde_json::Value>>,
}

impl GoogleCSE {
    /// Create a new GoogleCSE instance with the specified engine ID
    pub fn new(engine_id: &str) -> Self {
        Self {
            engine_id: engine_id.to_string(),
            api_key: None,
        }
    }

    /// Set API key for authenticated requests
    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = Some(api_key.to_string());
        self
    }

    /// Construct URL for direct browser access (no API key needed)
    pub fn construct_browser_url(&self, query: &str) -> String {
        format!(
            "https://cse.google.fi/cse?cx={}&q={}",
            self.engine_id,
            urlencoding::encode(query)
        )
    }

    /// Extract CSE data from JavaScript callback response
    fn extract_cse_data(&self, response_text: &str) -> Result<CSEResponse> {
        // The response starts with "google.search.cse.api14339(" and ends with ");"
        let re = Regex::new(r"google\.search\.cse\.api\d+\((.*)\);").unwrap();
        println!("Response text: {}", response_text);
        if let Some(captures) = re.captures(response_text) {
            if let Some(json_str) = captures.get(1) {
                let cse_data: CSEResponse = serde_json::from_str(json_str.as_str())?;
                return Ok(cse_data);
            }
        }

        Err(anyhow!("Failed to extract CSE data from response"))
    }

    // Update search_browser to make actual requests instead of returning mock data
    pub async fn search_browser(&self, query: &str) -> Result<CSEResponse> {
        // Step 1: Get initial page to establish a session
        let initial_url = self.construct_browser_url(query);
        let client = Self::build_proper_client()?;

        // Make the first request to the CSE interface
        let initial_response = client
            .get(&initial_url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (X11; Linux x86_64; rv:135.0) Gecko/20100101 Firefox/135.0",
            )
            .send()
            .await?;

        let _html = initial_response.text().await?;

        // Step 2: Make the API request to get JSON results
        let element_url = "https://cse.google.com/cse/element/v1";
        let response = client
            .get(element_url)
            .query(&[
                ("rsz", "filtered_cse"),
                ("num", "10"),
                ("hl", "en"),
                ("source", "gcsc"),
                ("cselibv", "75c56d121cde450a"),
                ("cx", &self.engine_id),
                ("q", query),
                ("safe", "off"),
                ("exp", "cc,apo"),
                ("cseclient", "hosted-page-client"),
            ])
            .header("Accept", "*/*")
            .header("Accept-Language", "en-US,en;q=0.5")
            .header("Accept-Encoding", "gzip, deflate, br, zstd")
            .header("Referer", "https://cse.google.fi/")
            .header("Sec-Fetch-Dest", "script")
            .header("Sec-Fetch-Mode", "no-cors")
            .header("Sec-Fetch-Site", "cross-site")
            .header("Connection", "keep-alive")
            .header("DNT", "1")
            .header("TE", "trailers")
            .send()
            .await?;

        // Get the response text and parse it
        let text = response.text().await?;
        self.extract_cse_data(&text)
    }

    /// For demonstration, this function shows how you would build a proper client
    /// This is not used in the actual implementation but shows the correct way
    /// to set up reqwest with proper TLS
    pub fn build_proper_client() -> Result<reqwest::Client> {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:135.0) Gecko/20100101 Firefox/135.0")
            .build()?;

        Ok(client)
    }

    /// This function shows how you would make a real request if everything worked
    /// Not used in actual implementation
    pub async fn real_search_example(&self, query: &str) -> Result<String> {
        let client = Self::build_proper_client()?;

        // Set all proper headers based on the HTTP trace
        let response = client
            .get("https://cse.google.com/cse/element/v1")
            .query(&[
                ("rsz", "filtered_cse"),
                ("num", "10"),
                ("hl", "fi"),
                ("source", "gcsc"),
                ("cselibv", "75c56d121cde450a"),
                ("cx", &self.engine_id),
                ("q", query),
                ("safe", "off"),
                ("exp", "cc,apo"),
                ("cseclient", "hosted-page-client"),
            ])
            .header("Accept", "*/*")
            .header("Accept-Language", "en-US,en;q=0.5")
            .header("Referer", "https://cse.google.fi/")
            .header("Sec-Fetch-Dest", "script")
            .header("Sec-Fetch-Mode", "no-cors")
            .header("Sec-Fetch-Site", "cross-site")
            .send()
            .await?;

        Ok(response.text().await?)
    }
}

/// Utility function to construct a job search URL for the provided CSE ID
pub fn construct_job_search_url(cse_id: &str, job_title: &str) -> String {
    let query = format!("site:jobs.* \"{}\"", job_title);
    format!(
        "https://cse.google.fi/cse?cx={}&q={}",
        cse_id,
        urlencoding::encode(&query)
    )
}

/// Specific function for linux administrator job search
pub fn search_linux_admin_jobs(cse_id: &str) -> String {
    construct_job_search_url(cse_id, "linux administrator")
}

/// Function to search and return results
pub async fn get_linux_jobs(cse_id: &str) -> Result<CSEResponse> {
    let cse = GoogleCSE::new(cse_id);
    // Using the mock response instead of real request to bypass TLS issues
    cse.search_browser("site:jobs.* \"Linux\"").await
}
