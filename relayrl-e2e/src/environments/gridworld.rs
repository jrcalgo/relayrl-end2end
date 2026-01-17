use relayrl_framework::prelude::templates::EnvironmentTrainingTrait;

type Position = (usize, usize);

pub struct Actor {
    current_position: Position,

}

impl Actor {
     fn move(state: &GridWorldEnv) -> Position {

     }
}

pub struct GridWorldEnv {

}

impl GridWorldEnv {
    length: usize,
    width: usize,
    walls: usize,
    actors: Vec<Actor>,
}

impl EnvironmentTrainingTrait for GridWorldEnv {
     fn run_environment(&self) -> Result<(), EnvError> {

     }

     fn build_observation(&self) -> Result<Any, EnvError> {

     }

     fn calculate_performance_return(&self) -> Result<Any, EnvError> {

     }
}
