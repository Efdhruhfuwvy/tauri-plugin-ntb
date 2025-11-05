const COMMANDS: &[&str] = &[
    "minimize",
    "toggle_maximize",
    "close",
    "drag",
    "move_by",
    "get_drag_behavior",
    "is_maximized",
    "get_window_controls",
    "get_window_control_image",
    "get_title_bar_css",
    "double_click_title_bar",
    "right_click_title_bar",
    "middle_click_title_bar",
    "show_snap_overlay",
    "hide_snap_overlay",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
