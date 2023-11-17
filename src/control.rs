use bevy::prelude::*;
use crate::entities::{MoveVector, Facing};
use crate::player::Player;
use crate::math::{max, min};

#[derive(Component)]
pub struct ControlScheme {
    pub forward: ScanCode,
    pub backward: ScanCode,
    pub left: ScanCode,
    pub right: ScanCode,
}

impl ControlScheme {
    pub fn wasd() -> Self {
        ControlScheme {
            forward: ScanCode(u32::from_str_radix("11", 16).unwrap()),
            backward: ScanCode(u32::from_str_radix("1F", 16).unwrap()),
            left: ScanCode(u32::from_str_radix("1E", 16).unwrap()),
            right: ScanCode(u32::from_str_radix("20", 16).unwrap()),
        }
    }

    // pub fn arrow() -> Self {
    //     ControlScheme {
    //         backward: KeyCode::Down,
    //         left: KeyCode::Left,
    //         right: KeyCode::Right,
    //     }
    // }
}

pub fn handle_input(keys: Res<Input<ScanCode>>, mut query: Query<(&mut Facing, &mut MoveVector, &ControlScheme), With<Player>>) {
    let (mut facing, mut mv, cs) = query.single_mut();
    if keys.pressed(cs.forward) {
        *facing = Facing::Backward;
        let new_y = min(mv.0.y + 1.0, 1.0);
        mv.0 = Vec2::new(mv.0.x, new_y);
    }
    
    if keys.pressed(cs.backward) {
        *facing = Facing::Forward;
        let new_y = max(mv.0.y - 1.0, -1.0);
        mv.0 = Vec2::new(mv.0.x, new_y);
    }

    if keys.pressed(cs.left) {
        let new_x = max(mv.0.x - 1.0, -1.0);
        mv.0 = Vec2::new(new_x, mv.0.y);
    }

    if keys.pressed(cs.right) {
        let new_x = min(mv.0.x + 1.0, 1.0);
        mv.0 = Vec2::new(new_x, mv.0.y);
    }

    if keys.just_released(cs.forward) {
        let new_y = min(mv.0.y - 1.0, 0.0);
        mv.0 = Vec2::new(mv.0.x, new_y);
    }

    if keys.just_released(cs.backward) {
        let new_y = max(mv.0.y - 1.0, 0.0);
        mv.0 = Vec2::new(mv.0.x, new_y);
    }
    
    if keys.just_released(cs.left) {
        let new_x = min(mv.0.x + 1.0, 0.0);
        mv.0 = Vec2::new(new_x, mv.0.y);
    }

    if keys.just_released(cs.right) {
        let new_x = max(mv.0.x - 1.0, 0.0);
        mv.0 = Vec2::new(new_x, mv.0.y);
    }
}


const KEY_CODE_MAP: std::collections::HashMap<&str, &str> = std::collections::HashMap::from([
    ("Esc", "01"),
    ("1", "02"),
    ("2", "03"),
    ("3", "04"),
    ("4", "05"),
    ("5", "06"),
    ("6", "07"),
    ("7", "08"),
    ("8", "09"),
    ("9", "0A"),
    ("0", "0B"),
    ("-", "0C"),
    ("=", "0D"),
    ("Backspace", "0E"),
    ("Tab", "0F"),
    ("Q", "10"),
    ("W", "11"),
    ("E", "12"),
    ("R", "13"),
    ("T", "14"),
    ("Y", "15"),
    ("U", "16"),
    ("I", "17"),
    ("O", "18"),
    ("P", "19"),
    ("[", "1A"),
    ("]", "1B"),
    ("Enter", "1C"),
    ("Ctrl", "1D"),
    ("A", "1E"),
    ("S", "1F"),
    ("D", "20"),
    ("F", "21"),
    ("G", "22"),
    ("H", "23"),
    ("J", "24"),
    ("K", "25"),
    ("L", "26"),
    (";", "27"),
    ("'", "28"),
    ("`", "29"),
    ("LShift", "2A"),
    ("\\", "2B"),
    ("Z", "2C"),
    ("X", "2D"),
    ("C", "2E"),
    ("V", "2F"),
    ("B", "30"),
    ("N", "31"),
    ("M", "32"),
    (",", "33"),
    (".", "34"),
    ("/", "35"),
    ("R", "36"),
    ("PtScr", "37"),
    ("Alt", "38"),
    ("Spc", "39"),
    ("CpsLk", "3A"),
    ("F1", "3B"),
    ("F2", "3C"),
    ("F3", "3D"),
    ("F4", "3E"),
    ("F5", "3F"),
    ("F6", "40"),
    ("F7", "41"),
    ("F8", "42"),
    ("F9", "43"),
    ("F10", "44"),
    ("Num", "45"),
    ("ScrlLk", "46"),
    ("Home", "47"),
    ("Up", "48"),
    ("Pg", "49"),
    ("Num-", "4A"),
    ("Left", "4B"),
    ("Right", "4D"),
    ("Num+", "4E"),
    ("End", "4F"),
    ("Down", "50"),
    ("PgDown", "51"),
    ("Ins", "52"),
    ("Del", "53"),
]);
