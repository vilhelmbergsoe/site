use axum::extract::State;
use axum::response::IntoResponse;
use chrono::{Duration, Utc};
use maud::html;
use std::fmt::Write;

use crate::SharedState;

pub async fn handle_stats(State(state): State<SharedState>) -> impl IntoResponse {
    let read_guard = state.total_views.read().await;
    let total_views: usize = read_guard.values().map(|views_set| views_set.len()).sum();
    let server_uptime = Utc::now() - state.uptime;

    let mut sorted_stats: Vec<_> = read_guard
        .iter()
        .map(|(title, views_set)| (title, views_set.len()))
        .collect();
    sorted_stats.sort_by(|a, b| b.1.cmp(&a.1));

    html! {
        h1 { "Statistics" }
        p style="opacity: 0.7; font-size: 0.9em; margin-top: -1.5em;" {
            "Ephemeral stats, reset on server restart."
        }
        p {
            strong { "Total Unique Views: " }
            (total_views)
        }
        ul {
            @for (title, views_count) in sorted_stats {
                li {
                    (title) ": " (views_count) " view(s)"
                }
            }
        }
        p {
            strong { "Server Uptime: " }
            (format_duration(server_uptime))
        }
    }
}

fn format_duration(duration: Duration) -> String {
    let mut total_seconds = duration.num_seconds();

    if total_seconds <= 0 {
        return "0s".to_string();
    }

    const SECONDS_IN_YEAR: i64 = 31_536_000;
    const SECONDS_IN_DAY: i64 = 86_400;
    const SECONDS_IN_HOUR: i64 = 3_600;
    const SECONDS_IN_MINUTE: i64 = 60;

    // Pre-allocate a reasonable capacity to avoid reallocations
    let mut result = String::with_capacity(25);

    let years = total_seconds / SECONDS_IN_YEAR;
    total_seconds %= SECONDS_IN_YEAR;

    let days = total_seconds / SECONDS_IN_DAY;
    total_seconds %= SECONDS_IN_DAY;

    let hours = total_seconds / SECONDS_IN_HOUR;
    total_seconds %= SECONDS_IN_HOUR;

    let minutes = total_seconds / SECONDS_IN_MINUTE;
    let seconds = total_seconds % SECONDS_IN_MINUTE;

    let mut append_part = |value: i64, unit: &str| {
        if value > 0 {
            if !result.is_empty() {
                result.push(' ');
            }
            write!(&mut result, "{}{}", value, unit).unwrap();
        }
    };

    append_part(years, "y");
    append_part(days, "d");
    append_part(hours, "h");
    append_part(minutes, "m");

    // Always show seconds if it's non-zero or if no other parts were added
    if seconds > 0 || result.is_empty() {
        if !result.is_empty() {
            result.push(' ');
        }
        write!(&mut result, "{}s", seconds).unwrap();
    }

    result
}
