fn gui_render_post(
    game: &crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    gui_hidden: bool,
    focused: bool,
) {
    crate::game::punchy::render_gui(game, batch, gui_hidden, focused);
}
