use chrono::TimeZone;
use chrono::{DateTime, TimeDelta, Utc};
use listenbrainz::raw::response::UserListensListen;
use listenbrainz::raw::response::UserListensPayload;
use listenbrainz::raw::response::UserListensResponse;
use listenbrainz::raw::Client;
use timespan::DateTimeSpan;
use timespan::Span;
use vec1::Vec1;

use crate::core::display::progress_bar::ProgressBarCli;
use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::insertable::Insertable;
use crate::models::cli::Cli;
use crate::models::data::listenbrainz::listen::collection::ListenCollection;
use crate::models::data::listenbrainz::listen::Listen;
use crate::utils::extensions::listenbrainz::ClientExt;
use crate::utils::extensions::listenbrainz::FetchDateDirection;
use crate::utils::extensions::listenbrainz::UserListensListenExt;
use crate::utils::extensions::UserListensPayloadExt;
use crate::utils::{println_cli, println_lis};

use super::UserListens;

impl UserListens {
    pub async fn fetch_listens(
        username: &str,
        incremental: bool,
        refresh_unmapped: bool,
    ) -> color_eyre::Result<Self> {
        // We get the total count of listens reported by listenbrainz.
        // If this number doesn't match with our count, this mean some listens got missed
        let target_listen_count = Client::new().user_listen_count(username)?.payload.count;

        Ok(())
    }

    #[must_use]
    pub async fn update_listens(
        cached_listens: &ListenCollection,
        fetched: &mut FetchOperation,
    ) -> color_eyre::Result<Self> {
    }

    pub async fn get_user_with_refresh(username: &str) -> color_eyre::Result<Self> {
        println_cli("Getting new user listens...");
        Self::fetch_latest(username).await?;

        println_cli("Updating unmapped listens...");
        Self::update_unlinked_of_user(username).await?;

        Ok(Self::get_from_cache(username)
            .await
            .expect("Couldn't get listen that were inserted")
            .expect("Couldn't get listen that were inserted"))
    }

    /// Fetch the most listens it can, that have been listened before the provided date. Additionally, save them to the cache
    pub async fn fetch_before(
        user: &str,
        before_date: DateTime<Utc>,
    ) -> color_eyre::Result<UserListensResponse> {
        println_lis(format!(
            "Getting listens from before: {} ({})",
            before_date,
            before_date.timestamp()
        ));

        let result =
            Client::new().user_listens(user, None, Some(before_date.timestamp()), Some(999))?;

        result.insert_into_cache_as(user.to_lowercase()).await?;

        Ok(result)
    }

    /// Fetch the latest listens that aren't yet in the cache. If it fetched more than needed, entries will get updated
    ///
    /// If the cache is empty, then it will fill it completely
    pub async fn fetch_latest(username: &str) -> color_eyre::Result<()> {
        let operation_start = Utc::now();

        let latest_cached_listen_date = Self::get_from_cache(username)
            .await?
            .and_then(|user_listens| user_listens.listens.get_latest_listen())
            .map(|listen| *listen.get_listened_at());

        // Prepare the loop variables
        let mut last_count = 1;
        let mut before_date = operation_start;

        // While we have still items, and that we aren't already reached the cached listens
        while last_count != 0
            && !latest_cached_listen_date.is_some_and(|cache_date| cache_date > before_date)
        {
            // We fetch a page of listens
            let result = Self::fetch_before(username, before_date).await?;

            // We put the new before date as the last listen's
            before_date = result
                .payload
                .get_date_of_oldest_listen_of_payload()
                .unwrap_or(operation_start);
            last_count = result.payload.listens.len();
        }

        Ok(())
    }

    /// Refetch all the unlinked listens of a user and recache them
    pub async fn update_unlinked_of_user(username: &str) -> color_eyre::Result<()> {
        // We first get all the unlinked cached
        let mut unlinkeds = Self::get_from_cache_or_new(username)
            .await?
            .get_unmapped_listens();
        let start_count = unlinkeds.len();

        let progress_bar =
            ProgressBarCli::new(unlinkeds.len() as u64, Some("Updating unmapped listens"));

        while unlinkeds.len() > 0 {
            let refresh_target = unlinkeds
                .get_latest_listen()
                .expect("Couldn't fetch listen");

            let result = Self::fetch_before(
                username,
                refresh_target.listened_at + TimeDelta::new(1, 0).unwrap(),
            )
            .await?
            .payload;

            unlinkeds.remove_timerange(
                &result
                    .get_date_of_oldest_listen_of_payload()
                    .unwrap_or_else(Utc::now),
                &result
                    .get_date_of_latest_listen_of_payload()
                    .unwrap_or_else(Utc::now),
                true,
            );

            progress_bar.set_position((start_count - unlinkeds.len()).try_into().unwrap());
        }

        Ok(())
    }
}

impl Insertable for UserListensResponse {
    async fn insert_into_cache_as(&self, key: String) -> color_eyre::Result<()> {
        let mut user_listens = UserListens::get_cache()
            .get(&key)
            .await?
            .unwrap_or_else(|| UserListens::new(&key));

        user_listens.refresh_timerange(self.payload.clone());

        UserListens::get_cache()
            .set(&key.to_lowercase(), user_listens)
            .await?;
        Ok(())
    }
}

struct FetchOperation {
    username: String,
    start_date: DateTime<Utc>,
    expected_final_listen_count: u64,

    cached_listens: Vec<Listen>,

    listens: Vec<UserListensListen>,
    timeranges: Vec1<DateTimeSpan<Utc>>,
}

impl FetchOperation {
    pub fn create(username: &str, cached_listens: Vec<Listen>) -> color_eyre::Result<Self> {
        let start_date = Utc::now();

        Ok(Self {
            username: username.to_string(),
            start_date,
            listens: Vec::new(),

            cached_listens,

            timeranges: Vec1::new(DateTimeSpan::new(start_date, start_date)?),
            expected_final_listen_count: Client::new().user_listen_count(username)?.payload.count,
        })
    }

    pub async fn fetch_until_count_match(&mut self) -> color_eyre::Result<()> {
        let client = Client::new();
        while self.get_total_count() as u64 != self.expected_final_listen_count {
            let fetched = client.fetch_listens(&self.username, FetchDateDirection::BeforeInclusive(self.timeranges.first().end.timestamp()), Some(999))?;
            self.add_listen_payload(fetched.payload, 999)
        }

        Ok(())
    }

    pub fn get_total_count(&self) -> usize {
        let mut total = self.listens.len();

        for cached_listen in self.cached_listens {
            if !self.listens.iter().any(|refreshed_listen| {refreshed_listen.get_id() == cached_listen.get_id()}) {
                total += 1;
            }
        }

        total
    }

    pub fn add_listen_payload(&mut self, payload: UserListensPayload, count_fetched: u64) {
        // Inputs must not be 0
        if count_fetched == 0 || payload.listens.len() == 0 {
            return;
        }

        let first_listen_date = payload
            .get_date_of_latest_listen_of_payload()
            .expect("Vec shouldn't be zero!");

        let mut listens = payload.listens;
        let last_listen = listens
            .iter()
            .min_by_key(|listen| listen.listened_at)
            .expect("Vec shouldn't be zero!");
        let mut last_timestamp = last_listen.listened_at;

        // If the payload response has been maxed,
        // we can't reliably say that we have all the listens made at the oldest timestamp.
        // So, we trim all the listens made at the last timestamp
        if count_fetched == listens.len() as u64 {
            listens.retain(|listen| listen.listened_at > last_timestamp);
            last_timestamp += 1;
        }

        self.listens.extend(listens);
        self.timeranges.push(
            DateTimeSpan::new(
            first_listen_date,
            Utc
            .timestamp_opt(last_timestamp, 0)
            .single()
            .expect("Cannot convert listened_at timestamp. This shouldn't happen since all the dates are UTC!")
        ).expect("Ordering error!")
        )
    }

    pub fn get_latest(&mut self, timerange: DateTimeSpan<Utc>) {

        // Then cleanup
        let mut dirty = self.timeranges.clone();
        dirty.sort_by_key(|span| span.end);
        let first = dirty.pop()

        for span in dirty {

        }
    }
}


fn merge_timestamps(a: DateTimeSpan<Utc>, b: DateTimeSpan<Utc>) -> Vec1<DateTimeSpan<Utc>> {
    match a.union(&b) {
        Ok(val) => Vec1::new(val),
        Err(_) => vec1::vec1!(a, b)
        
    }
}
