use relayrl_framework::prelude::templates::{EnvironmentTrainingTrait, EnvironmentError};
use relayrl_framework::prelude::tensor::burn::{Tensor, Backend, Float};

use std::any::Any;
use std::sync::atomic::AtomicBool;

type WorldLength = usize;
type WorldWidth = usize;
type Position = (WorldLength, WorldWidth);
type Reward = f32;

enum Move {
     Up,
     Down,
     Left,
     Right
}

#[derive(Clone)]
pub struct Actor {
    initial_position: Position,
    current_position: Position
}

impl Actor {
     fn move_position<B: Backend, const D: usize, K: TensorKind<B>>(&mut self, state: &GridWorldEnv, act: Tensor<B, D, K>) -> (Position, f32) {
           fn map_act_to_move(_act: Tensor<B, D, K>) -> Move {
              if act.shape().dims.len() == 1 {
               let act_scalar: f32 = _act.scalar_value::<f32>().unwrap();
              } else {
              
              }
           }

           let mut new_position = self.current_position.clone();
           match map_act_to_move(act) {
                 Up => new_position.0 -= 1,
                 Down => new_position.0 += 1,
                 Left => new_position.1 -= 1,
                 Right => new_position.1 += 1
           };

           let end_position: Position = state.end_position.clone();
           let valid: bool = state.validate_move(new_position);
           if valid {
              self.current_position = new_position.clone();
              (new_position, &self.compute_reward(new_position.clone(), end_position, true))
           } else {
              let current_position = self.current_position.clone();
              (current_position, &self.compute_reward(current_position, end_position, false))
           }
     }

     fn compute_reward(position: Position, end_position: Position, valid_move: bool) -> f32 {
         fn calculate_manhattan_distance(position: Position, end_position: Position) -> u32 {
            (position.0 - end_position.0).abs() + (position.1 - end_position.1).abs()
         }
         
         let mut cumulative_reward: f32 = 0.0;

         if valid_move {
            cumulative_reward += 1.0;
         } else {
            cumulative_reward -= 2.0;
         }
     }
}

pub struct GridWorldEnv {
    training: bool,
    length: WorldLength,
    width: WorldWidth,
    wall_positions: Vec<Position>,
    end_position: Position,
    actors: Vec<Actor>,
    running: AtomicBool
}

impl Default for GridWorldEnv {
     fn default() -> Self {
        let actor: Vec<Actor> = vec![Actor { initial_position: (0, 0), current_position: (0, 0)}; 1];
        let wall_positions = Vec::<Position>::new();
        wall_positions.extend([(2, 1), (2, 2), (2, 3), (2, 4), (3, 4), (4, 4), (5, 4), (6, 4), (7, 4), (2, 6), (2, 7), (2, 8)]);
        let end_position: Position = (9, 9);

        Self {
             training: true,
             length: 10 as WorldLength,
             width: 10 as WorldWidth,
             wall_positions,
             end_position,
             actors: actor,
             running: AtomicBool::new(false)
        }
     }
}

impl GridWorldEnv {
     pub fn set_world_state(self, training: bool, length: usize, width: usize, wall_positions: Vec<Position>, end_position: Position, initial_actor_positions: Vec<Position>) -> Result<Self, EnvironmentError> {
         let validation: Result<(), EnvironmentError> = {
               let mut first_pass: Vec<bool> = vec![true; initial_actor_positions.len()];

               for wall in wall_positions.iter() {
                  if wall.0 > length || wall.0 < length {
                     Err(EnvironmentError(format!("Wall at ({:?},{:?}) is out of length bounds of {:?}", wall.0, wall.1, length)))
                  }
                  if wall.1 > width || wall.1 < width {
                     Err(EnvironmentError(format!("Wall at ({:?},{:?}) is out of width bounds of {:?}", wall.0, wall.1, width)))
                  }
                  if wall.0 == end_position.0 && wall.1 == end_position.1 {
                     Err(EnvironmentError(format!("Wall at ({:?},{:?}) conflicts with end position of ({:?},{:?})", wall.0, wall.1, end_position.0, end_position.1)))
                  }
                  for (j, actor) in initial_actor_positions.iter().enumerate() {
                     if first_pass[j] {
                              if actor.0 > length || actor.0 < length {
                                 Err(EnvironmentError(format!("Actor at ({:?},{:?}) is out of length bounds of {:?}", actor.0, actor.1, length)))
                           }
                              if actor.1 > width || actor.1 < width {
                                 Err(EnvironmentError(format!("Actor at ({:?},{:?}) is out of width bounds of {:?}", actor.0, actor.1, width)))
                                             }
                              if actor.0 == end_position.0 && actor.1 == end_position.1 {
                                 Err(EnvironmentError(format!("Actor at ({:?},{:?}) conflicts with end end position of ({:?}, {:?})", actor.0, actor.1, end_position.0, end_position.1)))
                           }
                              first_pass[j] = false;
                        }

                        if wall.0 == actor.0 && wall.1 == actor.1 {
                           Err(EnvironmentError(format!("Actor at ({:?},{:?}) conflicts with wall at ({:?},{:?})", actor.0, actor.1, wall.0, wall.1)))
                        }
                     }
                  }
                  Ok(())
               }?;

         let actors: Vec<Actor> = initial_actor_positions.iter().map(|pos| Actor { initial_position: pos.clone(), current_position: pos.clone() }).collect::<Vec<Actor>>();

         Ok(Self {
              training,
              length,
              width,
              wall_positions,
              end_position,
              actors,
              running: AtomicBool::new(false)
         })
     }

     fn validate_move(&self, new_move: Position) -> bool {
        if new_move
     }
}

impl EnvironmentTrainingTrait for GridWorldEnv {
     fn run_environment(&self) -> Result<(), EnvironmentError> {

     }

     fn build_observation(&self) -> Result<Any, EnvironmentError> {

     }

     fn calculate_performance_return(&self) -> Result<Any, EnvironmentError> {

     }
}
