use crate::models::{api::GetFromCacheOrFetch, data::musicbrainz::release::Release};

use super::Recording;

impl Recording {
    pub async fn get_releases(&self) -> color_eyre::Result<Option<Vec<Release>>> {
        let Some(releases) = self.releases.clone() else {return Ok(None);};

        let tasks = releases.into_iter().map(|release_id| {
            tokio::spawn(Release::get_cached_or_fetch(release_id))
        } );

        let mut tasks_awaited = Vec::new();
        for task in tasks {
            tasks_awaited.push(task.await??)
        }

        Ok(Some(tasks_awaited))
    }
}