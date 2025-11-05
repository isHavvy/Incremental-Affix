//! Follower jobs

use bevy::prelude::*;

use crate::incremental::stock::{StockKind, stockyard::Stockyard};

mod spc;

pub struct JobsPlugin;

impl Plugin for JobsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spc::initialize_jobs_spc)
        .add_systems(Startup, initialize_jobs)
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
}

fn initialize_jobs(
    mut commands: Commands,
) {
    for job_kind in JobKind::LIST.iter().cloned() {
        commands.spawn((
            Job {
                kind: job_kind,
                followers_assigned: 0,
            },
        ));
    }
}

#[derive(Debug, Resource)]
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
    if stockyard[StockKind::Followers] < followers_assigned.0 {
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
    job_kind: JobKind,
}

fn on_unassign_follower(
    event: On<AssignFollower>,

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