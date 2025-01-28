// Re-export submodules
pub mod app; // App state and logic
pub mod input; // Input handling
pub mod layout; // UI layout
pub mod screen;

// Optional: Re-export key structs and enums for easier access
pub use app::App;
pub use input::InputMode;
pub use screen::AppScreen;
// pub use layout::draw_ui;
