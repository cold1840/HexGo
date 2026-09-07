use bevy::{
    utils::default,
    window::{Window, WindowResizeConstraints, WindowResolution},
};

pub fn primary_window() -> Window {
    let resize_constraints = if cfg!(target_os = "android") {
        // Use the minimum allowed size on Android to effectively disable resize constraints.
        WindowResizeConstraints {
            min_width: 1.0,
            min_height: 1.0,
            ..default()
        }
    } else {
        WindowResizeConstraints {
            min_width: 360.0,
            min_height: 480.0,
            ..default()
        }
    };

    Window {
        title: "HexGo".into(),
        resolution: WindowResolution::new(1280, 800),
        resize_constraints,
        resizable: !cfg!(target_os = "android"),
        ..default()
    }
}
