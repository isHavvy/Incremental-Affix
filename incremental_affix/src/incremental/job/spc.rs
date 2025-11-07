//! Stockyard producer/consumer for jobs

use bevy::prelude::*;
use super::*;

use crate::incremental::stock::producer_consumer::StockyardProducerConsumer;

#[derive(Debug, Component)]
pub(super) struct JobSpc;

impl Default for JobSpc {
    fn default() -> Self {
        Self
    }
}

pub(super) fn preconsume(
    mut job_query: Query<(&Job, &mut StockyardProducerConsumer)>,
) {
    for (job, mut spc) in job_query.iter_mut() {
        spc.consumes.clear();
        spc.consumes.extend(job.consumes.iter().copied().map(|sps| sps * job.followers_assigned))
    }
}

pub(super) fn postconsume(
    mut job_query: Query<(&Job, &mut StockyardProducerConsumer)>,
) {
    for (job, mut spc) in job_query.iter_mut() {
        let fulfillment = spc.consumption_fullfilled();
        spc.produces.clear();
        spc.produces.extend(job.produces.iter().copied().map(|sps| sps * job.followers_assigned * fulfillment))
    }
}