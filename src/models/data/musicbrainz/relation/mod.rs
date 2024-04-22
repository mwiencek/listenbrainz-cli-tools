pub mod caching;
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::models::data::musicbrainz::EntityMBIDType::EntityMBIDType;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Relation {
    end: Option<DateTime<Utc>>,
    attributes: Option<Vec<String>>,
    content: EntityMBIDType,
    attribute_values: Option<HashMap<String, String>>,
    attribute_ids: Option<HashMap<String, String>>,
    target_type: Option<String>,
    target_credit: Option<String>,
    source_credit: Option<String>,
    ended: Option<bool>,
    type_id: String,
    begin: Option<DateTime<Utc>>,
    direction: String,
    relation_type: String,
}