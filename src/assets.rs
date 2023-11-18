use bevy::prelude::*;
use std::path::{ Path, PathBuf };

#[derive(Resource)]
pub struct AssetPath(pub PathBuf);

pub fn set_asset_path(mut asset_path_res: ResMut<AssetPath>) {
    let cwd = std::env::current_dir().expect("error getting cwd");
    let exe_dir = std::env::current_exe()
        .expect("error getting current exe path")
        .parent()
        .expect("cannot get parent dir of current exe")
        .to_path_buf();

    let resources_from_exe_dir = exe_dir
        .parent()
        .expect("could not get exe_dir parent")
        .join(Path::new("Resources/"));

    let check_dirs = [cwd, exe_dir, resources_from_exe_dir];

    for dir in check_dirs.iter() {
        if !dir.is_dir() {
            continue;
        }

        for item in dir.read_dir().expect("").flatten() {
            let file_name = &item.file_name();
            let filename = file_name.to_string_lossy();
            if filename.contains("assets") {
                asset_path_res.0 = dir.as_path().join(filename.to_string()).to_path_buf();
                return;
            }
        }
    }

    panic!("Could not find assets directory");
}
