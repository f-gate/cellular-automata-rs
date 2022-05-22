pub const TRANSLATIONS_VON: [[i8; 3]; 6] = [
    [1, 0, 0],
    [-1, 0, 0],
    [0, 1, 0],
    [0, -1, 0],
    [0, 0, 1],
    [0, 0, -1],
];


pub const TRANSLATIONS_MOORE: [[i8 ;3];27] = [
    [1, 0, 0],
    [1, 1, 0],
    [1, -1, 0],
    [1, 0, 1],
    [1, 0, -1],
    [1, 1, 1],
    [1, -1, -1],
    [1, 1, -1],
    [1, -1, 1],
    [0, 1, 0],
    [0, -1, 0],
    [0, 1, -1],
    [0, -1, -1],
    [0, 1, 1],
    [0, -1, -1],
    [0, 0, 1],
    [0, 0, -1],
    [0, -1, 1],
    [-1, 0, 0],
    [-1, -1, -1],
    [-1, 1, 1],
    [-1, 0, 1],
    [-1, 0, -1],
    [-1, -1, 1],
    [-1, -1, 0],
    [-1, 1, 0],
    [-1, 1, -1],
];

pub struct StartShape {
    pub shape: Shape,
    pub is_hollow: bool,
    // todo: color
}


pub enum Shape {
    Diamond,
    Cube,
}



