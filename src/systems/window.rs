use bevy::prelude::*;
use bevy::winit::WinitWindows;

// Add this system to your startup systems
pub fn setup_window_icon(
    windows: Query<Entity, With<bevy::window::PrimaryWindow>>,
    winit_windows: NonSend<WinitWindows>,
) {
    let window_entity = windows.single();

    // Get the actual winit window
    let Some(primary) = winit_windows.get_window(window_entity) else {
        return;
    };

    // Load the icon
    let icon_path = "assets/brainquill_small.png"; // Use PNG for runtime
    let icon_bytes = std::fs::read(icon_path).unwrap_or_else(|_| {
        println!("Failed to load icon");
        Vec::new()
    });

    // Create the icon
    if let Ok(image) = image::load_from_memory(&icon_bytes) {
        let rgba = image.into_rgba8();
        let (width, height) = rgba.dimensions();
        let rgba_bytes = rgba.into_raw();

        if let Ok(icon) = winit::window::Icon::from_rgba(rgba_bytes, width, height) {
            primary.set_window_icon(Some(icon));
            println!("Set window icon successfully!");
        }
    }
}
