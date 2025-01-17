use fltk::{
    prelude::*,
    tree::{
        Tree,
        TreeItem
    },
    text::{
        TextBuffer,
    },
    dialog::alert,
    app::screen_size
};
use std::path::Path;

pub fn center() -> (i32, i32) {
    (
        (screen_size().0 / 2.0) as i32,  // Get the horizontal center of the screen
        (screen_size().1 / 2.0) as i32,  // Get the vertical center of the screen
    )
}

pub fn render_file(
    mut folders: Tree,
    mut text_buffer: TextBuffer,
    prefix: String
) {
    let folders_ = folders.clone();
    folders.set_callback(move |item| {
        if let Some(items) = item.get_selected_items() {  // Get the selected items from the Tree widget
            for i in items {  // Iterate over the selected items
                let path: String = prefix.clone() + "/"+ &item.item_pathname(&i).unwrap();  // Build the file path for the selected item
                let file_path: &Path = Path::new(&path);
                match text_buffer.load_file(file_path) {  // Load the file into the TextBuffer widget
                    Ok(_) => {
                        () // If loading the file was successful, do nothing
                    },
                    Err(e) => {
                        alert( // Display an error message dialog at the center of the screen
                            center().0 - 100,
                            center().1 - 100,
                            &format!("Error: {}", e)
                        );
                        let tree_item: TreeItem = TreeItem::new(&folders_, "Avoid select error");
                        let _ = item.select_only(
                            &tree_item,
                            true
                        );
                    }
                };
            }
        }
    })
}
