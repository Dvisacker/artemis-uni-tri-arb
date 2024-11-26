use crate::types::{Collector, CollectorStream};
use alloy::rpc::types::Log;
use alloy::{providers::Provider, rpc::types::Filter};
use async_trait::async_trait;
use eyre::Result;
use std::sync::Arc;
use tokio_stream::StreamExt;

/// A collector that listens for new blockchain event logs based on a [Filter](Filter),
/// and generates a stream of [events](Log).
pub struct LogCollector<M> {
    provider: Arc<M>,
    filter: Filter,
}

impl<M> LogCollector<M> {
    pub fn new(provider: Arc<M>, filter: Filter) -> Self {
        Self { provider, filter }
    }
}

/// Implementation of the [Collector](Collector) trait for the [LogCollector](LogCollector).
/// This implementation uses the [PubsubClient](PubsubClient) to subscribe to new logs.
#[async_trait]
impl<M> Collector<Log> for LogCollector<M>
where
    M: Provider,
{
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, Log>> {
        let stream = self.provider.subscribe_logs(&self.filter).await?;
        let stream = stream.into_stream();
        let stream = stream.filter_map(|log| Some(log));
        Ok(Box::pin(stream))
    }
}
