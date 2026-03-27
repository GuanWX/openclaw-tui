use serde::Deserialize;

#[derive(Debug)]
pub struct CopilotUsage {
    pub used: u64,
    pub limit: u64,
}

#[derive(Debug, Deserialize)]
struct GitHubUser {
    login: String,
}

#[derive(Debug, Deserialize)]
struct CopilotUsageResponse {
    total_suggestions: u64,
    total_acceptances: u64,
    // GitHub Copilot doesn't have a direct API for usage limits
    // This is a placeholder - actual implementation may need adjustment
}

/// Fetch GitHub Copilot usage statistics
/// Requires a GitHub Personal Access Token with `copilot` scope
pub async fn fetch_usage(github_token: &str) -> Result<CopilotUsage, String> {
    let client = reqwest::Client::new();

    // Verify the token and get user info
    let user_resp = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {}", github_token))
        .header("User-Agent", "AI-Token-Monitor")
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !user_resp.status().is_success() {
        let status = user_resp.status();
        return Err(format!("GitHub API error: {}", status));
    }

    // Try to get Copilot usage (GitHub Enterprise feature)
    // Note: This endpoint may not be available for personal accounts
    let usage_resp = client
        .get("https://api.github.com/copilot/usage")
        .header("Authorization", format!("Bearer {}", github_token))
        .header("User-Agent", "AI-Token-Monitor")
        .send()
        .await;

    // GitHub Copilot doesn't have a public API for personal usage
    // We'll return a placeholder for now
    // In practice, you'd need to scrape the GitHub Copilot settings page
    // or use the Enterprise API if available

    match usage_resp {
        Ok(resp) if resp.status().is_success() => {
            // Parse usage if available
            Ok(CopilotUsage {
                used: 0, // Would parse from response
                limit: 0, // Would parse from response
            })
        }
        _ => {
            // Fallback: show as not available
            // In a real implementation, you might:
            // 1. Use GitHub's GraphQL API
            // 2. Parse the Copilot billing page
            // 3. Show subscription status instead

            // For now, indicate that usage tracking is limited
            Err("Copilot usage API requires GitHub Enterprise. Showing subscription status instead.".to_string())
        }
    }
}

/// Alternative: Check Copilot subscription status
pub async fn check_subscription(github_token: &str) -> Result<String, String> {
    let client = reqwest::Client::new();

    let resp = client
        .get("https://api.github.com/user/settings/copilot")
        .header("Authorization", format!("Bearer {}", github_token))
        .header("User-Agent", "AI-Token-Monitor")
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if resp.status().is_success() {
        Ok("Copilot subscription active".to_string())
    } else {
        Err("Copilot subscription not found or API unavailable".to_string())
    }
}