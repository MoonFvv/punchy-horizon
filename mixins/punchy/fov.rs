fn compute_fov(
    game: &crate::game::Game<'_>,
    partial: f32,
    camera: [f64; 3],
    degrees: &mut f32,
) {
    crate::game::punchy::compute_fov(game, partial, camera, degrees);
}
