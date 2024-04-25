use crate::models::data::musicbrainz::HasMbid;

pub trait CacheFromMusicbrainz<MbV>
where
    MbV: Into<Self>,
    Self: Sized,
{
    /// Insert an element with a specific MBID into the cache
    async fn insert_ms_with_id_into_cache(mbid: String, value: MbV) -> color_eyre::Result<()>;

    /// Insert an iterator element with a specific MBID into the cache
    async fn insert_ms_with_id_iter_into_cache<I: IntoIterator<Item = (String, MbV)>>(
        values: I,
    ) -> color_eyre::Result<()> {
        for (mbid, value) in values {
            Self::insert_ms_with_id_into_cache(mbid, value).await?
        }

        Ok(())
    }
}

pub trait CacheFromMusicbrainzAutoId<MbV>: CacheFromMusicbrainz<MbV>
where
    MbV: Into<Self> + HasMbid + Clone,
    Self: Sized,
{
    /// Insert the current item with its own MBID into the cache.
    async fn insert_ms_into_cache(value: MbV) -> color_eyre::Result<()> {
        Self::insert_ms_with_id_into_cache(value.get_mbid().to_string(), value).await
    }

    /// Insert a collection of items with their own MBIDs into the cache.
    async fn insert_ms_iter_into_cache<I: IntoIterator<Item = MbV>>(
        values: I,
    ) -> color_eyre::Result<()> {
        for value in values {
            Self::insert_ms_into_cache(value).await?
        }

        Ok(())
    }

    /// Insert the current item with its own MBID into the cache, as well as an alias MBID.
    /// This is useful incase an item has be been merged, and the alias MBID is only a reference to the original.
    async fn insert_ms_with_alias_into_cache(
        alias_mbid: String,
        value: MbV,
    ) -> color_eyre::Result<()> {
        Self::insert_ms_into_cache(value.clone()).await?;
        Self::insert_ms_with_id_into_cache(alias_mbid, value).await
    }
}

impl<V, MbV> CacheFromMusicbrainzAutoId<MbV> for V
where
    MbV: Into<V> + HasMbid + Clone,
    V: Sized + CacheFromMusicbrainz<MbV>,
{
}
