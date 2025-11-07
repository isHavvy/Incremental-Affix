//! Follower jobs

use bevy::prelude::*;

use crate::incremental::stock::StockPerSecond;
use crate::incremental::stock::producer_consumer::StockSystems;
use crate::incremental::stock::{StockKind, producer_consumer::StockyardProducerConsumer, stockyard::Stockyard};
use crate::incremental::PerSecond;

mod spc;

pub struct JobsPlugin;

impl Plugin for JobsPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<FollowersAssigned>()
        .add_systems(Startup, initialize_jobs)
        .add_systems(FixedUpdate, (
            spc::preconsume.in_set(StockSystems::PreConsume),
            spc::postconsume.in_set(StockSystems::PostConsume),
        ))
        .add_observer(on_assign_follower)
        .add_observer(on_unassign_follower)
        ;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum JobKind {
    ChopWood,
    Hunt,
    RenderCarcass,
    Cook,
}

impl JobKind {
    pub const LIST: &[Self] = &[Self::ChopWood, Self::Hunt, Self::RenderCarcass, Self::Cook];

    /// Returns (production, consumption) for each job.
    /// 
    /// The actual per-second should probably be determined by tools.
    fn base_production_consumption(&self) -> (&'static [StockPerSecond], &'static [StockPerSecond]) {
        match *self {
            JobKind::ChopWood => const { (&[
                StockPerSecond::new(StockKind::Wood, PerSecond(1.0))
            ], &[]) },
            JobKind::Hunt => const { (&[
                StockPerSecond::new(StockKind::Carcass, PerSecond(1.0))
            ], &[]) },
            JobKind::RenderCarcass => const { (&[
                StockPerSecond::new(StockKind::Meat, PerSecond(1.0)),
                StockPerSecond::new(StockKind::Bone, PerSecond(1.0)),
            ], &[
                StockPerSecond::new(StockKind::Carcass, PerSecond(1.0))
            ]) },
            JobKind::Cook => const { (&[
                StockPerSecond::new(StockKind::Food, PerSecond(1.0)),
            ], &[
                StockPerSecond::new(StockKind::Meat, PerSecond(1.0)),
                StockPerSecond::new(StockKind::Wood, PerSecond(1.0)),
            ]) },
        }
    }
}

impl std::fmt::Display for JobKind {    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            JobKind::ChopWood => "Chop Wood",
            JobKind::Hunt => "Hunt",
            JobKind::RenderCarcass => "Render Carcasses",
            JobKind::Cook => "Cook",
        })
    }
}

#[derive(Debug, Component)]
struct Job {
    kind: JobKind,
    followers_assigned: u32,
    produces: Vec<StockPerSecond>,
    consumes: Vec<StockPerSecond>,
}

fn initialize_jobs(
    mut commands: Commands,
) {
    for job_kind in JobKind::LIST.iter().cloned() {
        let (production, consumption) = job_kind.base_production_consumption();

        commands.spawn((
            Job {
                kind: job_kind,
                followers_assigned: 0,
                produces: production.into(),
                consumes: consumption.into(),
            },

            spc::JobSpc,
            StockyardProducerConsumer::default(),
        ));
    }
}

#[derive(Debug, Resource, Default, Deref, DerefMut)]
/// The total number of followers assigned to jobs.
/// 
/// This must always be less than or equal to the number of followers in the stockyard.
struct FollowersAssigned(u32);

#[derive(Debug, Event)]
pub struct AssignFollower {
    pub job_kind: JobKind,
}

fn on_assign_follower(
    event: On<AssignFollower>,

    mut followers_assigned: ResMut<FollowersAssigned>,
    stockyard: Res<Stockyard>,

    mut job_query: Query<&mut Job>,
) {
    // This could be an `==` since the `<` part should never occur.
    // But if somehow it does, this function will not allow assigning infinite followers.
    if stockyard[StockKind::Followers] <= followers_assigned.0 {
        // #[TODO(Havvy)]: Put out an error event.
        return;
    }

    followers_assigned.0 += 1;

    let mut job = job_query.iter_mut().find(|job| job.kind == event.job_kind)
    .expect("There should be an entity with a job for each job kind.");

    job.followers_assigned += 1;
}

#[derive(Debug, Event)]
pub struct UnassignFollower {
    pub job_kind: JobKind,
}

fn on_unassign_follower(
    event: On<UnassignFollower>,

    mut followers_assigned: ResMut<FollowersAssigned>,

    mut job_query: Query<&mut Job>,
) {
    let mut job = job_query.iter_mut().find(|job| job.kind == event.job_kind)
    .expect("There should be an entity with a job for each job kind.");

    if job.followers_assigned == 0 {
        // #[TODO(Havvy)]: Fire an event saying that no followers were unassigned.
        return;
    }

    job.followers_assigned -= 1;
    followers_assigned.0 -= 1;
}