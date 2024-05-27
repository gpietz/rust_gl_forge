pub(crate) fn update_rotation_angle_with_time(
    rotation_paused: bool,
    rotation_angle: &mut f32,
    rotation_speed: f32,
    delta_time: f32,
) {
    if !rotation_paused {
        // Update rotation calculation
        *rotation_angle += rotation_speed * delta_time;
        *rotation_angle %= 360.0;
    }
}
