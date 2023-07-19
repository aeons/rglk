use crate::player::{get_player_input, PlayerInput};
use crate::prelude::*;

pub fn player_movement(
    mut cmd: Commands,
    mut q_player: Query<(Entity, &mut Position, &mut Viewshed), With<Player>>,
    q_combat_stats: Query<&CombatStats, Without<Player>>,
    keys: Res<Input<KeyCode>>,
    map: Res<Map>,
    mut run_state: ResMut<NextState<RunState>>,
) {
    let (entity, mut pos, mut viewshed) = q_player.single_mut();
    let input = get_player_input(&keys);

    if let PlayerInput::Movement { x, y } = input {
        let dst = **pos + Point::new(x, y);

        for potential_target in map.tile_content[map.point2d_to_index(dst)].iter() {
            if let Ok(_target) = q_combat_stats.get(*potential_target) {
                cmd.entity(entity).insert(WantsToMelee {
                    target: *potential_target,
                });
                return;
            }
        }

        if map.is_valid_exit(&dst) {
            pos.0 = dst;
            viewshed.dirty = true;
            println!("moved player to {:?}", dst);
        }

        run_state.set(RunState::Running);
    } else {
        run_state.set(RunState::Paused);
    }
}
