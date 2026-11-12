// Comando chat: `/iris` o `/shaders` per ricaricare gli shader in tempo reale
fn iris_horizon_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::iris_horizon as m;
    if *ran || !game.native_on(m::ID) || !game.native_window() {
        return;
    }
    *ran = crate::game::iris_horizon_bridge::command(game, command);
}
