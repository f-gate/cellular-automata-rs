use ndarray::Array3;
use std::*;
use crate::settings as settings;

#[derive(Debug)]
pub enum Method {
    VonNeumann, 
    Moore
}
/// an item containing all the cells to be passed to rendering and designating rules.
#[derive(Debug)]
pub struct Block {
    pub method: Method,
    pub edge: i16,
    pub step_in: i16,
    pub size_bounds: i16,
    pub n_rule: [bool; 27],
    pub b_rule: [bool; 27],
    pub s_rule: i8,
    pub grid: Array3<i8>,
}


impl Block {
    ///initialise the start shape with 1ns, process initial neighbors and spit out the array for processing.
    pub fn init(start_shape: settings::StartShape, edge:&i16, size_bounds: &i16, step_in: i16) -> Array3::<i8> {
        let mut grid = Block::get_fresh_grid(&size_bounds);
        match start_shape.shape {
            settings::Shape::Diamond => {
                //todo:
                //init with one as all thats needed for alive
                //return diamond shape
            },
            settings::Shape::Cube => {

                let draw_length = (*edge - (step_in * 2)) as i16 ;
                let instep = step_in;

                let max = draw_length + instep; 
                let min = instep;  

                if start_shape.is_hollow {
                    for x in min..max {
                        for y in min..max {
                            for z in min..max {
                                let max = max - 1;
                                //if x or y or z == hollow max or Min then draw
                                if x == min || x == max || y == min 
                                || y == max || z == min || z == max  {
                                    grid[[x as usize, y as usize, z as usize]] = 0;
                                }
                            }
                        }
                    }

                } else {
                    for x in instep..max {
                        for y in instep..max {
                            for z in instep..max {
                                grid[[x as usize, y as usize, z as usize]] = 0;
                            }
                        }
                    }
                }
            },
        }
        println!("{:?}", grid);
        return grid
    }

    ///get fresh grid on ones(dead and ready for respawn)
    //todo:: pass in edge?
    pub fn get_fresh_grid(size_bounds: &i16) -> Array3<i8> {
        let edge = (*size_bounds as f32).cbrt().ceil() as usize;
        Array3::<i8>::ones((edge, edge, edge))
    }

    
    ///passes in a grid and makes changes based on given rules
    /// n_rule is how many to stay alive, b_rule for to be born, s_rule for hoe many game tics till dead
    pub fn update_grid(&mut self) {

        //because box value 1 is dead
        let s_rule = self.s_rule + 1;
        let old_grid  = self.grid.clone();
        for x in 1.. (self.edge - 2) as usize {
            for y in 1.. (self.edge - 2) as usize {
                for z in 1.. (self.edge - 2) as usize {
                    let neighbors: usize = Block::get_neighbors(&old_grid, x, y, z, &self.method);
                    if neighbors == 0 {continue;}
                    let grid_val: i8 = old_grid[[x, y, z]];
                    
                    //0 = alive
                    //1 = dead
                    //anything more is state

                    match grid_val {
                        0 => {
                            //if wrong amount of neighbors then die
                            if self.n_rule[neighbors - 1] == false {
                                self.grid[[x, y, z]] = s_rule;
                            } 
                        },
                        1 => {
                            //if dead check against rule and maybe come alive
                            if self.b_rule[neighbors - 1] == true {
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
    pub fn get_neighbors(grid: &Array3::<i8>, x: usize, y:usize, z:usize, method: &Method) -> usize {
        if y == 0 || x == 0 || z == 0 {
            panic!("Edges of the cube is out of bounds!!")
        }
        match method {
            //filter any neighbors thats value is alive (0) and collect.len()
            Method::Moore => {
                let params  = settings::TRANSLATIONS_MOORE;
             
                params.iter().filter(|p| grid[[(x as i8 +p[0]) as usize, (y as i8+p[1]) as usize, (z as i8+p[2]) as usize ]] == 0).collect::<Vec<&[i8; 3]>>().len()

            },
            Method::VonNeumann => {
                let params  = settings::TRANSLATIONS_VON;

                 params.iter().filter(|p| grid[[(x as i8 +p[0]) as usize, (y as i8+p[1]) as usize, (z as i8+p[2]) as usize ]] == 0).collect::<Vec<&[i8; 3]>>().len()
                
            },
        }



    }


}