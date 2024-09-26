use substreams::pb::substreams::Clock;
use substreams_antelope::pb::Block;
use substreams_entity_change::tables::Tables;

use crate::utils::{block_date_to_month, block_date_to_year, block_time_to_date};

use super::transactions::insert_transaction;

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L21
pub fn insert_blocks(params: &str, tables: &mut Tables, clock: &Clock, block: &Block) {
    // timestamp
    let timestamp = clock.timestamp.as_ref().expect("missing timestamp");
    let seconds = timestamp.seconds;
    let date = block_time_to_date(timestamp.to_string().as_str());
    let month = block_date_to_month(date.as_str());
    let year = block_date_to_year(date.as_str());
    let block_number = clock.number.to_string();

    // TABLE::Transaction
    let mut is_match = false;
    for transaction in block.transaction_traces() {
        if insert_transaction(&params, tables, clock, &transaction) {
            is_match = true;
        }
    }

    // TABLE::Block
    if is_match {
        tables
            .create_row("Block", clock.id.as_str())
            .set_bigint("number", &block_number.to_string())
            .set_bigint("timestamp", &seconds.to_string())
            .set("date", date)
            .set("month", month)
            .set("year", year);
    }
}
