use substreams::{pb::substreams::Clock, Hex};
use substreams_antelope::pb::{ActionTrace, TransactionTrace};
use substreams_entity_change::tables::Tables;

use crate::{
    index::{collect_action_keys, is_match},
    keys::action_key,
};

use super::authorizations::insert_authorization;

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L525
pub fn insert_action(params: &str, tables: &mut Tables, clock: &Clock, trace: &ActionTrace, transaction: &TransactionTrace) -> Option<String> {
    // trace
    let index = trace.execution_index;
    let action = trace.action.as_ref().expect("action missing");
    let receiver = &trace.receiver;

    // receipt
    let receipt = trace.receipt.as_ref().expect("receipt missing");
    let global_sequence = receipt.global_sequence;

    // action data
    let account = action.account.as_str();
    let name = action.name.as_str();
    let json_data = action.json_data.as_str();
    let raw_data = Hex::encode(action.raw_data.to_vec());
    let console = trace.console.as_str();
    let is_notify = account.ne(receiver);
    let is_input = trace.creator_action_ordinal == 0;

    // transaction
    let tx_hash = transaction.id.as_str();

    // TABLE::Action
    if is_match(collect_action_keys(trace), params) {
        let key = action_key(tx_hash, index);
        tables
            .create_row("Action", key.as_str())
            // pointers
            .set("transaction", tx_hash)
            .set("block", clock.id.as_str())

            // trace
            .set_bigint("index", &index.to_string())
            .set("receiver", receiver)
            .set("isNotify", is_notify)
            .set("isInput", is_input)
            .set("console", console)

            // receipt
            .set_bigint("globalSequence", &global_sequence.to_string())

            // action
            .set("account", account)
            .set("name", name)
            .set("jsonData", json_data)
            .set("rawData", raw_data);
        // TABLE::authorizations
        for authorization in action.authorization.iter() {
            insert_authorization(tables, trace, transaction, authorization);
        }
        return Some(key);
    }
    return None;
}
