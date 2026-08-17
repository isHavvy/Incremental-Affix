//! Components for events when a total amount of stock is produced

// All the names in here absolutely suck. I was exhausted when I
// made these names.

use bevy::prelude::*;
use bevy::ecs::system::SystemId;

use crate::incremental::stock::{StockKind, stockyard::Stockyard};

#[derive(Debug, Component)]
pub struct OnStockTotalProduced {
    pub stock_kind: StockKind,
    pub total_produced: f64,
    pub on_total_produced: SystemId,
}

pub fn on_stock_total_firer(
    mut commands: Commands,

    stockyard: Res<Stockyard>,

    on_stock_total_producer_query: Query<(Entity, &OnStockTotalProduced)>,
) {
    for (ostp_entity, ostp) in on_stock_total_producer_query.iter() {
        if stockyard[ostp.stock_kind].total_produced >= ostp.total_produced {
            commands.run_system(ostp.on_total_produced);
            commands.entity(ostp_entity).despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::incremental::stock::{StockKind, on_total::OnStockTotalProduced, stockyard::Stockyard};

    #[derive(Debug, Default, Resource)]
    struct SystemIsCalled(bool);

    #[test]
    fn on_stock_total_produced_test() {
        let mut app = App::new();
        app
        .init_resource::<Stockyard>()
        .init_resource::<SystemIsCalled>()
        .add_systems(Startup, ostp_test_setup)
        .add_systems(Update, super::on_stock_total_firer)
        ;

        app.update();

        let world = app.world_mut();
        assert!(world.resource::<SystemIsCalled>().0 == false);
        let mut stockyard = world.resource_mut::<Stockyard>();
        stockyard[StockKind::Wood] += 5.0;

        app.update();
        
        let world = app.world();
        assert!(world.resource::<SystemIsCalled>().0 == true);
    }

    fn ostp_test_setup(
        mut commands: Commands,
    ) {
        let system_id = commands.register_system(on_wood_over_five);
        let ostp = OnStockTotalProduced {
            stock_kind: StockKind::Wood,
            total_produced: 5.0,
            on_total_produced: system_id,
        };
        commands.spawn(ostp);
    }

    fn on_wood_over_five(
        mut system_is_called: ResMut<SystemIsCalled>,
    ) {
        system_is_called.0 = true;
    }
}