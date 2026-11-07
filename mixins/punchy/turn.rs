fn turn_punchy_orbit(
    game: &crate::game::Game<'_>,
    xo: f64,
    yo: f64,
) -> std::ops::ControlFlow<()> {
    crate::game::punchy::turn_player(game, xo, yo)
}
