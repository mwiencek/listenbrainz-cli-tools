use listenbrainz::raw::response::UserListensListen;

pub struct UnlinkedListenCollection(Vec<UserListensListen>);

impl UnlinkedListenCollection {
    pub fn push(&mut self, item: UserListensListen) {
        if item.track_metadata.mbid_mapping.is_none() {
            self.push(item)
        }
    }
}
