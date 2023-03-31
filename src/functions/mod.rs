pub use window::*;
pub use text_editor::*;
pub use folders::*;
pub use terminal_output::*;
pub use terminal_input::*;
pub use commands_for_cargo::*;
pub use commands_for_windows::*;
pub use horizontal_slider::*;
pub use get_root::*;
pub use set_root::*;
pub use btn_add_folder::*;
pub use render_file::*;
pub use get_paths::*;
pub use save_file::*;
pub use write_terminal::*;
pub use run_a_command::*;
pub use get_folders_roots::*;
pub use set_folders_roots::*;

pub mod set_folders_roots;
pub mod get_folders_roots;
pub mod run_a_command;
pub mod write_terminal;
pub mod save_file;
pub mod get_paths;
pub mod render_file;
pub mod btn_add_folder;
pub mod set_root;
pub mod get_root;
pub mod horizontal_slider;
pub mod commands_for_windows;
pub mod commands_for_cargo;
pub mod terminal_input;
pub mod terminal_output;
pub mod folders;
pub mod text_editor;
pub mod window;
