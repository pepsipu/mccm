use anyhow::Context;
use serde::Deserialize;

const MODRINTH_API: &str = "https://api.modrinth.com/v2";
const USER_AGENT: &str = "mccm";

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResponse {
    pub hits: Vec<ProjectHit>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProjectHit {
    pub project_id: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub downloads: u64,
    pub icon_url: Option<String>,
    pub date_modified: String,
}

pub async fn search_modpacks(query: Option<&str>) -> anyhow::Result<Vec<ProjectHit>> {
    let client = reqwest::Client::new();
    let facets = r#"[["project_type:modpack"]]"#;

    let url = format!("{MODRINTH_API}/search");
    let mut req = client
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .query(&[("facets", facets), ("index", "downloads"), ("limit", "20")]);

    if let Some(q) = query {
        if !q.trim().is_empty() {
            req = req.query(&[("query", q)]);
        }
    }

    let res = req
        .send()
        .await
        .context("modrinth request failed")?
        .error_for_status()
        .context("modrinth returned an error")?
        .json::<SearchResponse>()
        .await
        .context("failed to parse modrinth response")?;

    Ok(res.hits)
}
