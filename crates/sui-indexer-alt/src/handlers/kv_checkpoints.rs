// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use anyhow::{Context, Result};
use diesel_async::RunQueryDsl;
use sui_indexer_alt_framework::{
    db,
    pipeline::{concurrent::Handler, Processor},
};
use sui_types::full_checkpoint_content::CheckpointData;

use crate::{models::checkpoints::StoredCheckpoint, schema::kv_checkpoints};

pub(crate) struct KvCheckpoints;

impl Processor for KvCheckpoints {
    type Value = StoredCheckpoint;

    const NAME: &'static str = "kv_checkpoints";

    fn process(&self, checkpoint: &Arc<CheckpointData>) -> Result<Vec<Self::Value>> {
        let sequence_number = checkpoint.checkpoint_summary.sequence_number as i64;
        Ok(vec![StoredCheckpoint {
            sequence_number,
            certified_checkpoint: bcs::to_bytes(&checkpoint.checkpoint_summary)
                .with_context(|| format!("Serializing checkpoint {sequence_number} summary"))?,
            checkpoint_contents: bcs::to_bytes(&checkpoint.checkpoint_contents)
                .with_context(|| format!("Serializing checkpoint {sequence_number} contents"))?,
        }])
    }
}

#[async_trait::async_trait]
impl Handler for KvCheckpoints {
    async fn commit(values: &[Self::Value], conn: &mut db::Connection<'_>) -> Result<usize> {
        Ok(diesel::insert_into(kv_checkpoints::table).values(values).on_conflict_do_nothing().execute(conn).await?)
    }
}
