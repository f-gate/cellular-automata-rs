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
    pub fn init(&mut self, start_shape: setting::StartShape, size_factor: f32, s_rule: i8) -> Array3::<i8> {
        match start_shape {
            settings::StartShape::Diamond => {
                //init with one as all thats needed for alive
                //return diamond shape
            } ,
            settings::StartShape::Cube => {
                //^^
                //return cube shape
            } ,
        }
    }
    
    ///passes in a grid and makes changes based on given rules
    /// nrule is how many to stay alive, brule for to be born, srule for hoe many game tics till dead
    pub fn update_grid_moore(&mut grid: Array3::<i8>, n_rule: [bool;27], b_rule: [bool;27], s_rule: i8) {
        for x in 0..=SIZE {
            for y in 0..=SIZE {
                for z in 0..=SIZE {
                    let neighbors: i8 = get_neighbors(grid, x, y, z);
                    let grid_val = grid[[x, y, z]];
                    
                    //0 = alive
                    //1 = dead
                    //anything more is state

                    match grid_val {
                        0 => {
                            //if the right amount of neighbors the stay alive else die
                            //flipped for efficiency
                            if n_rule[neighbors] != true {
                                grid[[x, y, z]] = grid_val;
                            }
                        },
                        1 => {
                            //if dead check against rule and maybe come alive with state max
                            if b_rule[neighbors] == true {
                                grid[[x, y, z]] = s_rule;
                            }
                            continue;
                        },
                        _ => {
                            //down a val per game tick until 1
                                grid[[x, y, z]] = grid_val - 1;
                        },

                    }

                    
                    
                    
                }
            }   
        }
    }
    
    ///count the number of neighbor a given cell has
    pub fn get_neighbors(grid: Array3::<i8>, x: i8, y:i8, z:i8) -> i8 {
        neighbor_params.iter().filter(|p| grid[[x + p[0], y+p[1], z+p[2]]] > 0).collect().len()
    }

}