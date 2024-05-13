use crate::models::data::musicbrainz::entity_enum::MSEntity;

/// The first element is the main one, the second is the extras
type MSEntityResult = (Option<MSEntity>, Vec<MSEntity>);

pub trait IntoMSEntities {
    fn into_ms_entities(self) -> MSEntityResult;
}

impl<T: IntoMSEntities> IntoMSEntities for Vec<T> {
    fn into_ms_entities(self) -> MSEntityResult {
        self.into_iter()
            .flat_map(IntoMSEntities::into_ms_entities)
            .collect_vec()
    }
}

impl<T: IntoMSEntities> IntoMSEntities for Option<T> {
    fn into_ms_entities(self) -> MSEntityResult {
        match self {
            Self::Some(val) => val.into_ms_entities(),
            None => Vec::new(),
        }
    }
}
