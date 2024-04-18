use super::Release;

impl Release {
    pub fn get_mbid(&self) -> &str {
        &self.id
    }

    pub fn get_release_group_id(&self) -> Option<String> {
        self.release_group
    }
}
