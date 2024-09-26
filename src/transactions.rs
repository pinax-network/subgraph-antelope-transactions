use std::collections::HashSet;

use substreams::pb::substreams::Clock;
use substreams_antelope::pb::TransactionTrace;
use substreams_entity_change::tables::Tables;

use crate::{db_ops::collapse_db_ops, order_by::insert_order_by};

use super::{actions::insert_action, db_ops::insert_db_op};

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L525
pub fn insert_transaction(params: &str, tables: &mut Tables, clock: &Clock, transaction: &TransactionTrace) -> bool {
    let hash = transaction.id.as_str();
    let index = transaction.index;
    let elapsed = transaction.elapsed;
    let net_usage = transaction.net_usage;
    let scheduled = transaction.scheduled;

    // only include successful transactions
    let header = transaction.receipt.as_ref().expect("receipt missing");
    if header.status != 1 {
        return false;
    }

    // TABLE::Action
    let mut is_matched = false;
    let mut action_keys = HashSet::new();
    for trace in transaction.action_traces.iter() {
        match insert_action(params, tables, clock, trace, transaction) {
            Some(action_key) => {
                action_keys.insert(action_key);
                is_matched = true;
            }
            None => {}
        }
    }

    // collapse overlapping db_ops per transactions (usually spam related contracts)
    let collapsed_db_ops = collapse_db_ops(transaction);

    // TABLE::DbOps
    for db_op_ext in collapsed_db_ops.iter() {
        if insert_db_op(params, tables, clock, &db_op_ext.db_op, transaction, db_op_ext.index, &action_keys) {
            is_matched = true;
        }
    }

    // TABLE::Transaction
    if is_matched {
        let row = tables
            .create_row("Transaction", hash)
            // pointers
            .set("block", clock.id.as_str())
            // block
            .set_bigint("index", &index.to_string())
            .set_bigint("elapsed", &elapsed.to_string())
            .set_bigint("netUsage", &net_usage.to_string())
            .set("scheduled", scheduled);
        insert_order_by(row, clock);

        return true;
    }
    return false;
}
