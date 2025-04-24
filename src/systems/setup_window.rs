use bevy::prelude::*;
use bevy::winit::WinitWindows;

/// Plugin to set up the window properties and icon for the application.
pub struct SetupWindowPlugin;

impl Plugin for SetupWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_window, setup_window_icon));
    }
}

/// Sets up the window icon for the application.
fn setup_window_icon(
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

/// Sets up the window properties for the application.
/// This is particularly important for WebAssembly builds to ensure the canvas fits the parent element.
fn setup_window(mut window_query: Query<&mut Window>) {
    if let Ok(mut window) = window_query.get_single_mut() {
        // Make sure these are set for WASM display
        window.fit_canvas_to_parent = true;
        window.canvas = Some("#bevy".to_string());
    }
}
