// Eseguito una volta per fotogramma: aggiornamento pipeline shader e shadow culling
fn iris_horizon_frame(game: &mut crate::game::Game<'_>) {
    crate::game::iris_horizon_bridge::frame(game);
}
