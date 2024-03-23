
use super::provider::{DataProvider};
use std::sync::Arc;

#[derive(Debug)]
#[derive(Clone)]
pub struct ChannelId {
    pub data_source: Arc<DataProvider>,
    pub dataset: String,
    pub channel: String,
}

impl PartialEq for ChannelId{
    fn eq(&self, other: &Self) -> bool {
        let same_data_source =  self.data_source.base_url == other.data_source.base_url;
        let same_dataset =  self.dataset == other.dataset;
        let same_channel =  self.channel == other.channel;
        return same_data_source && same_dataset && same_channel;
    }
}


// impl Copy for ChannelId {
//     fn copy(&self) -> ChannelId {
//         *self
//     }
// }
