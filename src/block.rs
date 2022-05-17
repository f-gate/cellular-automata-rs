use ndarray::Array3;

mod cell;
mod settings;

enum Method {
    VonNeumann, 
    Moore
}



/// an item containing all the cells to be passed to rendering and designating rules.
pub struct Block {
    pub method: Method,
    pub settings: settings::Settings,
}


impl Block {

    ///initialise the start shape with 1ns, process initial neighbors and spit out the array for processing.
    pub fn init(&mut self, start_shape: setting::StartShape, size_factor: f32) -> Array3::<i8> {
        match start_shape {
            settings::StartShape::Diamond => {
                //init with one 
                //recurse for neighbor count
                //return diamond shape
            } ,
            settings::StartShape::Cube => {
                //^^
                //return cube shape
            } ,
        }
    }
    
    pub fn update_grid_moore(&mut grid: Array3::<i8>, n_rule: [bool;27], b_rule: [bool;27], s_rule: [bool;27]) {
        for x in 0..=SIZE {
            for y in 0..=SIZE {
                for z in 0..=SIZE {
                    grid[[x, y, z]] = get_neighbors(grid, x, y, z);
                }
            }   
        }
    }
    
    ///count the number of neighbor a given cell has
    pub fn get_neighbors(grid: Array3::<i8>, x: i8, y:i8, z:i8) -> i8 {
        neighbor_params.iter().filter(|p| grid[[x + p[0], y+p[1], z+p[2]]] > 0).collect().len()
    }

}