// Pagina delle impostazioni e selezione shader pack in Opzioni > Mod > Iris
fn iris_horizon_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::iris_horizon as m;
    if mod_id != m::ID {
        return;
    }
    buttons.extend(crate::game::iris_horizon_bridge::settings(game).labels());
}
