use ilwt_jobsearch::get_linux_jobs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // CSE ID from the URL provided
    let cse_id = "partner-pub-1447025976836499:5129066185";

    // Get Linux job search results
    let search_results = get_linux_jobs(cse_id).await?;

    // Display job count at top for F-pattern scanning
    println!("Found {} Linux jobs", search_results.cursor.resultCount);

    // Display the top 5 job results
    for (i, result) in search_results.results.iter().take(5).enumerate() {
        println!("\n#{} - {}", i+1, result.title);
        println!("Company: {}", result.visibleUrl);
        println!("Description: {}", result.content);
        println!("Apply at: {}", result.url);
    }

    Ok(())
}
