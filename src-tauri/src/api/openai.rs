use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OpenAIBalance {
    pub remaining: f64,
    pub used: f64,
}

#[derive(Debug, Deserialize)]
struct UsageResponse {
    total_usage: f64,
}

#[derive(Debug, Deserialize)]
struct CreditResponse {
    total_available: f64,
    total_granted: f64,
}

/// Fetch OpenAI API credit balance
/// Requires an API key with billing access
pub async fn fetch_balance(api_key: &str) -> Result<OpenAIBalance, String> {
    let client = reqwest::Client::new();

    // Get credit grants (available balance)
    let credit_resp = client
        .get("https://api.openai.com/v1/credit_grants")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !credit_resp.status().is_success() {
        let status = credit_resp.status();
        let body = credit_resp.text().await.unwrap_or_default();
        return Err(format!("API error {}: {}", status, body));
    }

    let credit: CreditResponse = credit_resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    // Get usage this month
    let usage_resp = client
        .get("https://api.openai.com/v1/usage")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let used = if usage_resp.status().is_success() {
        let usage: UsageResponse = usage_resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        usage.total_usage
    } else {
        0.0
    };

    Ok(OpenAIBalance {
        remaining: credit.total_available,
        used,
    })
}