// Re-export submodules
pub mod app; // App state and logic
pub mod input; // Input handling
pub mod layout; // UI layout

// Optional: Re-export key structs and enums for easier access
pub use app::{InputMode, TuiApp};
// pub use layout::draw_ui;
