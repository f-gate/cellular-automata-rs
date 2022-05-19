

const SIZE: i16 = 128 + 1;

//todo: the other type
pub const neighbor_params_moore: [[i8 ;3];27] = [
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



