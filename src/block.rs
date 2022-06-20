use ndarray::Array3;
use std::*;
use super::settings;
use cgmath::{prelude::*, num_traits::{Num, Pow}};
#[path = "graphics/wgpud/instance.rs"] mod instance;

#[derive(Debug, Clone)]
pub enum Method {
    VonNeumann, 
    Moore
}
/// an item containing all the cells to be passed to rendering and designating rules.
#[derive(Debug, Clone)]
pub struct Block {
    pub method: Method,
    pub edge_max: i16,
    pub step_in: i16,
    pub n_rule: [bool; 27],
    pub b_rule: [bool; 27],
    pub s_rule: i8,
    pub grid: Vec<i8>,
    pub instances: Vec<instance::Instance>,
    pub space_between_blocks: f32, 
}


impl Block {
    fn get_index(x: i16, y: i16, z: i16, edge_size: i16) -> usize {
        let x = x as i32;
        let z = z as i32;
        let y = y as i32;
        let edge_size = edge_size as i32;

        (x + edge_size * (y + edge_size * z)) as usize
    }

    ///initialise the start shape with 1ns, process initial neighbors and spit out the tuple for processing.
    pub fn get_fresh_grid(start_shape: settings::StartShape, edge:i16, step_in: i16, s_rule: i8, space_between: f32) -> (Vec<i8>, Vec<instance::Instance>) {
        let edge_usize = edge as usize;
        let mut grid: Vec<i8> = vec![0; edge_usize.pow(3)];
        let mut instances: Vec<instance::Instance> = vec![];
        match start_shape.shape {
            settings::Shape::Diamond => {
                //todo:
                //init with one as all thats needed for alive
                //return diamond shape
            },
            settings::Shape::Cube => {

                let draw_length = (edge - (step_in * 2)) as i16 ;

                let max = draw_length + step_in; 
                let min = step_in;  

                if start_shape.is_hollow {
                    for x in min..max {
                        for y in min..max {
                            for z in min..max {
                                //for indexing
                                let max = max - 1;
                                //if x or y or z == hollow max or Min then draw
                                if x == min || x == max || y == min 
                                || y == max || z == min || z == max  {
                                    let index = Block::get_index(x,y,z, edge); 

                                     grid[index] = s_rule;
                                     let x = space_between * (x as f32 - edge as f32 / 2.0);
                                     let z = space_between * (z as f32 - edge as f32 / 2.0);
                                     let y = space_between * (y as f32 - edge as f32 / 2.0);

                                     instances.push(instance::Instance {
                                        position: cgmath::Vector3 {x: x as f32,y:  y as f32,z:  z as f32},
                                        color: cgmath::Vector3 {x: x as f32,y:  y as f32,z:  z as f32}.into(),  
                                        index,
                                     })
                                }
                            }
                        }
                    }

                } else {
                    //fill the whole thing
                    for x in step_in..max {
                        for y in step_in..max {
                            for z in step_in..max {
                                let index = Block::get_index(x,y,z, edge); 
                                grid[index] = s_rule;
                                println!("{}", index);
                                
                                let x = space_between * (x as f32 - edge as f32 / 2.0);
                                let z = space_between * (z as f32 - edge as f32 / 2.0);
                                let y = space_between * (y as f32 - edge as f32 / 2.0);
                                instances.push(instance::Instance {
                                    position: cgmath::Vector3 {x: x as f32,y:  y as f32,z:  z as f32},
                                    color: cgmath::Vector3 {x: x as f32,y:  y as f32,z:  z as f32}.into(),  
                                    index,
                                 })
                            }
                        }
                    }
                }
            },
        }
    
        (grid, instances)
    }

    ///passes in a grid and makes changes based on given rules
    /// n_rule is how many to stay alive, b_rule for to be born, s_rule for how many game tics till dead
    pub fn update_grid(&mut self)  {

        let s_rule = self.s_rule - 1;
        let old_grid  = self.grid.clone();

        for x  in 1 .. (self.edge_max - 2)  {
            for y in 1 .. (self.edge_max - 2) {
                for z in 1.. (self.edge_max - 2) {
                    let mut index_remove : Vec<usize> = vec![];
                    let mut index_add : Vec<usize> = vec![];

                    let index = Block::get_index(x, y, z, self.edge_max);
                    let neighbors: usize = Block::get_neighbors(&old_grid, x, y, z, &self.method, self.edge_max);
                    let grid_val: i8 = old_grid[Block::get_index(x,y,z, self.edge_max)];


                    //if survival rule is false then take the grid val down per tick
                    if self.n_rule[neighbors] == false && grid_val != 0 {
                        self.grid[Block::get_index(x,y,z, self.edge_max)] = (grid_val - 1) as i8;
                    }
                    
                    //if born rule is true then be born
                    if self.b_rule[neighbors] == true {
                        self.grid[Block::get_index(x,y,z, self.edge_max)] = s_rule;
                        if grid_val == 0 {
                            index_add.push(index);
                        }
                    }

                    if grid_val == 0 {
                        index_remove.push(index);
                    }
                    let x = self.space_between_blocks * (x as f32 - self.edge_max as f32 / 2.0);
                    let z = self.space_between_blocks * (z as f32 - self.edge_max as f32 / 2.0);
                    let y = self.space_between_blocks * (y as f32 - self.edge_max as f32 / 2.0);
                    self.instances.iter_mut().filter(|i| !i.index == index );
                    self.instances.push( instance::Instance {
                                     position: cgmath::Vector3 {x: x as f32,y:  y as f32,z:  z as f32},
                                    color: cgmath::Vector3 {x: x as f32,y:  y as f32,z:  z as f32}.into(),  
                                    index,
                    })                    
                }
            }   
        }
    }
    
    //get sum of neighbors that == 0
    pub fn get_neighbors(grid: &Vec<i8>, x: i16, y:i16, z:i16, method: &Method, edge_size: i16) -> usize {
        if y == 0 || x == 0 || z == 0 {
            panic!("Edges of the cube is out of bounds!!")
        }
        match method {
            //filter any neighbors thats value is alive (0) and collect.len()
            Method::Moore => {
                let params  = settings::TRANSLATIONS_MOORE;
                //messy?                            grid[get_index((x as i8 +p[0]) as usize, (y as i8+p[1]) as usize, (z as i8+p[2]) as usize)]
                params.iter().filter(|p| grid[Block::get_index(x +p[0], y +p[1], z +p[2] , edge_size)] > 0).collect::<Vec<&[i16; 3]>>().len()

            },
            Method::VonNeumann => {
                let params  = settings::TRANSLATIONS_VON;

                params.iter().filter(|p| grid[Block::get_index(x +p[0], y +p[1], z +p[2] , edge_size)] > 0).collect::<Vec<&[i16; 3]>>().len()
                
            },
        }
    }
}