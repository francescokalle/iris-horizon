// Executed once per frame: pipeline state and shadow synchronization.
fn iris_horizon_frame(game: &mut crate::game::Game<'_>) {
    crate::game::iris_horizon_bridge::frame(game);
}
