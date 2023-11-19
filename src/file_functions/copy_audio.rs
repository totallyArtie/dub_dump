//todo

// lets clone all of the source folder into a new folder in that folder
use crate::graceful_shutdown;
use fs_extra::dir::copy; // recursive copy
use fs_extra::dir::CopyOptions;
use std::fs::create_dir;
use std::path::Path;

#[must_use]
pub fn copy_audio(master: String) -> String {
    // create the new folder

    // move up a dir from audio folder
    let path = Path::new(&master);
    let parent_path = path.parent().map_or_else(
        || {
            graceful_shutdown(
                "[copy_audio] : no folder above current!"
                    .to_string()
                    .as_str(),
                1,
            )
        },
        |a| a,
    );

    // into string
    let up_path: String = parent_path.to_str().map_or_else(
        || {
            graceful_shutdown(
                "[copy_audio] : could not cast path to string!"
                    .to_string()
                    .as_str(),
                1,
            )
        },
        std::string::ToString::to_string,
    );

    let new_path_str = format!("{up_path}\\Dub_dump\\");
    let new_path = Path::new(&new_path_str);

    debug_log!("{}", format!("target directory at:{}",new_path_str.as_str()));

    // If the path isn't a directory then, then create a new folder
    if !new_path.is_dir(){
        debug_log!("attempting to create target directory");
        match create_dir(new_path) {
            Ok(()) => {}
            Err(err) => graceful_shutdown(
                format!("[copy_audio] : error creating new folder: {err:#?}").as_str(),
                1,
            ),
        }
        debug_log!("directory created")
    } else {
        debug_log!("target directory already exists");
    }



    let mut options: CopyOptions = CopyOptions::new(); //Initialize default values for CopyOptions
    options.content_only = true;
    options.overwrite = true;

    // copy the directory
    match copy(master, &new_path, &options) {
        Ok(_) => {}
        Err(err) => graceful_shutdown(
            format!("[copy_audio] : error copying files to new folder: {err:#?}").as_str(),
            1,
        ),
    }

    return new_path_str;
}
