// Rendering dettagli su schermo (F3): nome shader pack, profilo attivo, tempo frame shader
fn iris_horizon_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    crate::game::iris_horizon_bridge::hud(game, batch, corners);
}
