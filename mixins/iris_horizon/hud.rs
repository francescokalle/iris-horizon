// F3 debug HUD details overlay hook.
fn iris_horizon_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    crate::game::iris_horizon_bridge::hud(game, batch, corners);
}
