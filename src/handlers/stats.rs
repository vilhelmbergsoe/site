use axum::extract::State;
use axum::response::{Html, IntoResponse};
use chrono::Utc;

use crate::SharedState;

pub async fn handle_stats(State(state): State<SharedState>) -> impl IntoResponse {
    let read_guard = state.total_views.read().await;
    let total_views: usize = read_guard.values().map(|views_set| views_set.len()).sum();
    let server_uptime = Utc::now() - state.uptime;

    let stats_list = read_guard
        .iter()
        .map(|(title, views_set)| format!("<li>{}: {} views</li>", title, views_set.len()))
        .collect::<Vec<_>>()
        .join("");

    Html(format!(
        r#"
        <h1>Statistics</h1>
        <p><strong>Total Views:</strong> {}</p>
        <ul>{}</ul>
        <p><strong>Server Uptime:</strong> {} seconds</p>
        "#,
        total_views,
        stats_list,
        server_uptime.num_seconds(),
    ))
}
