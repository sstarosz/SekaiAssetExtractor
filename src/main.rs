use std::fs;
use serde_json::Result;
use serde::{Serialize, Deserialize};
use serde_json::Value;



#[derive(Debug, Serialize, Deserialize)]
struct EventRankingReward {
    id: u32,
    #[serde(rename = "eventRankingRewardRangeId")]
    event_ranking_reward_range_id: u32,
    #[serde(rename = "resourceBoxId")]
    resource_box_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct EventRankingRewardRange {
    id: u32,
    #[serde(rename = "eventId")]
    event_id: u32,
    #[serde(rename = "fromRank")]
    from_rank: u32,
    #[serde(rename = "toRank")]
    to_rank: u32,
    #[serde(rename = "isToRankBorder")]
    is_to_rank_border: bool,
    #[serde(rename = "eventRankingRewards")]
    event_ranking_rewards: Vec<EventRankingReward>,
}


#[derive(Debug, Serialize, Deserialize)]
struct Event {
    id: u32,
    #[serde(rename = "eventType")]
    event_type: String,
    name: String,
    #[serde(rename = "assetbundleName")]
    assetbundle_name: String,
    #[serde(rename = "bgmAssetbundleName")]
    bgm_assetbundle_name: String,
    #[serde(rename = "eventOnlyComponentDisplayStartAt")]
    event_only_component_display_start_at: u64,
    #[serde(rename = "startAt")]
    start_at: u64,
    #[serde(rename = "aggregateAt")]
    aggregate_at: u64,
    #[serde(rename = "rankingAnnounceAt")]
    ranking_announce_at: u64,
    #[serde(rename = "distributionStartAt")]
    distribution_start_at: u64,
    #[serde(rename = "eventOnlyComponentDisplayEndAt")]
    event_only_component_display_end_at: u64,
    #[serde(rename = "closedAt")]
    closed_at: u64,
    #[serde(rename = "distributionEndAt")]
    distribution_end_at: u64,
    #[serde(rename = "virtualLiveId")]
    virtual_live_id: Option<u32>,
    unit: String,
    #[serde(rename = "eventRankingRewardRanges")]
    event_ranking_reward_ranges: Vec<EventRankingRewardRange>
}


fn main() {
    println!("Hello, world!");
    println!("Current directory: {:?}", std::env::current_dir().unwrap());
    


    let data = fs::read_to_string("master-db/events.json")
        .expect("Unable to read file");


    let events: Vec<Event> = serde_json::from_str(&data)
        .expect("JSON was not well-formatted");

    println!("Loaded {} events", events.len());


    println!("Event ID: {}", events[0].id);
    println!("Event Type: {}", events[0].event_type);
    println!("Event Name: {}", events[0].name);
    println!("Event Asset Bundle Name: {}", events[0].assetbundle_name);
    println!("Event BGM Asset Bundle Name: {}", events[0].bgm_assetbundle_name);
    //println!("Event Ranking Reward Ranges: {:?}", events[0].eventRankingRewardRanges);
    


}
