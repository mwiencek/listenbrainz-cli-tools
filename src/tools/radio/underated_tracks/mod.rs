use listenbrainz::raw::{response::StatsUserRecordingsResponse, Client};

use crate::models::{api::GetFromCacheOrFetch, cache::global_cache::GlobalCache, data::musicbrainz::recording::Recording};

pub fn get_underated_tracks(username: &str) {
    let listens = GlobalCache::new()
        .get_user_listens_with_refresh(username)
        .expect("Couldn't fetch the new listens");

    let top_thousand = Client::new()
        .stats_user_recordings(username, Some(1000), None, Some("all_time"))
        .unwrap()
        .unwrap();
}

async fn calculate_score_of_recording(
    recording_mbid: String,
    user_top: StatsUserRecordingsResponse,
) -> color_eyre::Result<u64> {
    // Top position
    let position_on_top = user_top.payload.recordings.iter().position(|top_entry| {
        top_entry
            .recording_mbid.as_ref()
            .is_some_and(|top_mbid| top_mbid == &recording_mbid)
    }).unwrap_or(9999);

    let top_score = 1_u64.div_ceil(position_on_top as u64) * 100;


    Ok(top_score)
}

pub async fn calculate_listen_share_score(recording_mbid: String, recording_listen_count: u64) -> color_eyre::Result<u64> {
    // Release group listen shares
    let recording = Recording::get_cached_or_fetch(recording_mbid).await?;
    let Some(releases) = recording.get_releases().await? else {return Ok(0);};
    let Some(target_release_group) = releases.get(0).and_then(|release| release.get_release_group_id()) else {return Ok(0);};

    let Some(release_group_stats) = Client::new().stats_release_group_listeners(&target_release_group, None)? else {return Ok(0);};
    release_group_stats.
}