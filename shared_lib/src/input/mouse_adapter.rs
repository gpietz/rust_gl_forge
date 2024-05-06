pub trait MouseAdapter {
    /// Retrieves the identifier of the window that currently holds mouse focus.
    fn focused_window_id(&self) -> Option<u32>;

    /// Returns `true` if the mouse cursor is currently positioned within the window.
    fn is_mouse_in_window(&self) -> bool;

    fn show_cursor(&self, show: bool);

    fn is_cursor_showing(&self) -> bool;

    /// Capture the mouse and to track input outside the window.
    fn capture_mouse(&self, capture_enabled: bool);

    fn mouse_x(&self) -> i32;

    fn mouse_y(&self) -> i32;

    fn mouse_xy(&self) -> (i32, i32);

    fn mouse_position(&self) -> (i32, i32);

    fn mouse_position_ref(&self, xpos: &mut i32, ypos: &mut i32);

    fn is_mouse_button_pressed(&self, mouse_button: &MouseButton) -> bool;

    fn pressed_mouse_buttons(&self) -> impl Iterator<Item = &MouseButton>;
}

pub enum MouseButton {
    Left,
    Middle,
    Right,
}

impl MouseButton {
    pub fn variants() -> &'static [MouseButton] {
        static VARIANTS: [MouseButton; 3] =
            [MouseButton::Left, MouseButton::Middle, MouseButton::Right];
        &VARIANTS
    }
}
