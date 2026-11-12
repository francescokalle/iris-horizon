// Gestione del click nei bottoni delle opzioni shader (cambio pack, toggle ombre, ecc.)
fn iris_horizon_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::iris_horizon as m;
    if mod_id != m::ID {
        return;
    }
    crate::game::iris_horizon_bridge::click(game, button);
}
