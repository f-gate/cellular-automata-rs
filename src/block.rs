use ndarray::Array3;
use std::*;
use crate::settings as settings;
pub enum Method {
    VonNeumann, 
    Moore
}
/// an item containing all the cells to be passed to rendering and designating rules.
pub struct Block {
    pub method: Method,
    pub edge: i16,
    pub size_factor: f32,
    pub size_bounds: i16,
    pub n_rule: [bool; 27],
    pub b_rule: [bool; 27],
    pub s_rule: i8,
    pub grid: Array3<i8>,
}


impl Block {
    ///initialise the start shape with 1ns, process initial neighbors and spit out the array for processing.
    pub fn init(start_shape: settings::StartShape, block: Block, s_rule: i8) -> Array3::<i8> {
        let mut grid = Block::get_fresh_grid(&block.size_bounds);
        match start_shape.shape {
            settings::Shape::Diamond => {
                //init with one as all thats needed for alive
                //return diamond shape
            },
            settings::Shape::Cube => {
                let edge = (block.size_bounds as f32 * block.size_factor).cbrt().round() as i32;
                if start_shape.is_hollow {
                    //todo
                } else {
                    for x in 0..=edge {
                        for y in 0..=edge {
                            for z in 0..=edge {
                                grid[[x as usize, y as usize, z as usize]] = 0;
                            }
                        }
                    }
                }
                //^^
                //return cube shape
            },
        }
        return grid
    }

    fn get_fresh_grid(size_bounds: &i16) -> Array3<i8> {
        let edge = (*size_bounds as f32).cbrt().ceil() as usize;
        Array3::<i8>::ones((edge, edge, edge))
    }

    
    ///passes in a grid and makes changes based on given rules
    /// n_rule is how many to stay alive, b_rule for to be born, s_rule for hoe many game tics till dead
    pub fn update_grid(&mut self) {

        //because box value 1 is dead
        let s_rule = self.s_rule + 1;
        let edge = self.edge as i8;
        let old_grid  = self.grid.clone();
        for x in 0..= edge as usize {
            for y in 0..= edge as usize {
                for z in 0..= edge as usize {
                    let neighbors: usize = Block::get_neighbors(&old_grid, x, y, z);
                    let x = x as usize;
                    let y = y as usize;
                    let z = z as usize;

                    let grid_val: i8= old_grid[[x, y, z]];
                    
                    //0 = alive
                    //1 = dead
                    //anything more is state

                    match grid_val {
                        0 => {
                            //if wrong amount of neighbors then die
                            if self.n_rule[neighbors] == false {
                                self.grid[[x, y, z]] = s_rule;
                            } 
                        },
                        1 => {
                            //if dead check against rule and maybe come alive
                            if self.b_rule[neighbors] == true {
                                self.grid[[x, y, z]] = 0;
                            }
                        },
                        _ => {
                            //down a val per game tick until 1
                                self.grid[[x, y, z]] = grid_val - 1;
                        },

                    }
                }
            }   
        }
        
    }
    
    //get sum of neighbors that == 0
    pub fn get_neighbors(grid: &Array3::<i8>, x: usize, y:usize, z:usize) -> usize {
        //i think it works lol 19/05
        settings::neighbor_params_moore.iter().filter(|p| grid[[x + (p[0] as usize), y+(p[1] as usize), z+(p[2] as usize)]] == 0).collect::<Vec<&[i8; 3]>>().len()
    }

}