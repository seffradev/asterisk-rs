use derive_getters::Getters;
use serde::Deserialize;

#[derive(Debug, Deserialize, Getters)]
pub struct Bridge {
    id: String,
}
