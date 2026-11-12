// A click on the mod's settings page: change settings and apply.
fn iris_horizon_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::iris_horizon as m;
    if mod_id != m::ID {
        return;
    }
    crate::game::iris_horizon_bridge::click(game, button);
}
