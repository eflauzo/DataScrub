
use super::provider::{DataProvider};
use std::sync::Arc;

#[derive(Debug)]
pub struct ChannelId {
    pub data_source: Arc<DataProvider>,
    pub dataset: String,
    pub channel: String,
}