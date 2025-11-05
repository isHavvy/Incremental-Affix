//! Stockyard producer/consumer for jobs

use bevy::prelude::*;

use crate::incremental::{job::JobKind, stock::producer_consumer::StockyardProducerConsumer};

#[derive(Debug, Component)]
struct JobSpc {
    job: JobKind,
}

pub(super) fn initialize_jobs_spc(
    mut commands: Commands,
) {
    for job in JobKind::LIST.iter().copied() {
        commands.spawn((
            JobSpc { job },
            StockyardProducerConsumer::default(),
        ));
    }
}