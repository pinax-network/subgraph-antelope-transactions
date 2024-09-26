use substreams::pb::substreams::Clock;
use substreams_entity_change::tables::Row;

pub fn insert_order_by(row: &mut Row, clock: &Clock) {
    let timestamp = clock.timestamp.as_ref().expect("missing timestamp");
    let seconds = timestamp.seconds;
    let block_number = clock.number.to_string();

    row.set_bigint("block_number", &block_number.to_string()).set_bigint("timestamp", &seconds.to_string());
}
