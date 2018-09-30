extern crate ndarray;
extern crate rand;

use rand::{thread_rng, Rng};
use ndarray::Array2;
use posutils::{Point, get_directions, check_point, get_neighborhood};

pub fn generate_landscape(size: usize, sea_level: usize, range: usize, token_n: usize, token_limit: usize) -> Array2<usize> {
    let land = Array2::<usize>::zeros((size, size));
    let mut executor = CoastlineAgentExecutor::new();
    executor.attach(CoastlineAgent::new(token_n, Point{x: (size/2) as isize, y:(size/2) as isize}, size), token_limit, size);
    executor.apply(&land, sea_level, range)
}

struct CoastlineAgentExecutor{
    agents: Vec<CoastlineAgent>,
}

impl CoastlineAgentExecutor{

    fn new() -> CoastlineAgentExecutor{
        CoastlineAgentExecutor {
            agents: Vec::new()
        }
    }

    fn attach(&mut self, agent: CoastlineAgent, token_limit: usize, size: usize) {
        if agent.token_n > token_limit{
            for _ in 0..2{
                self.attach(CoastlineAgent::from_parent(&agent, size), token_limit, size);
            }
        } else {
            self.agents.push(agent)
        }
    }

    fn apply(&self, land: &Array2<usize>, sea_level: usize, range: usize) -> Array2<usize>{
        let mut active = true;
        let mut agents = self.agents.clone();
        let mut new_land = land.clone();
        let mut active_agents = agents.len();
        while active {
            active = false;
            for i in 0..(agents.len()) {
                if agents[i].active {
                    new_land = match agents[i].apply(&new_land, sea_level, range) {
                        Some(a) => a,
                        None =>{
                            active_agents-=1;
                            new_land
                        } 
                    };
                    active = true;
                }
                //println!("{}", active_agents);
            }
        }
        new_land
    }
}

#[derive(Debug, Copy, Clone)]
struct CoastlineAgent{
    token_n: usize,
    direction: Point,
    position: Point,
    attractor: Point,
    repulsor: Point,
    active: bool
}

impl CoastlineAgent{
    
    fn new(token_n: usize, position: Point, size: usize) -> CoastlineAgent{
        CoastlineAgent{
            token_n,
            direction: {
                *thread_rng().choose(&get_directions()).unwrap()
            },
            position,
            attractor: Point::random(size),
            repulsor: Point::random(size),
            active: true,
        }
    }

    fn score(&self, p: &Point, size: usize) -> f64{
        //println!("{:?}", self.repulsor);
        self.repulsor.distance(p) - self.attractor.distance(p) + 3f64*(p.edge_distance(size) as f64)
    }

    pub fn apply(&mut self, land: &Array2<usize>, sea_level: usize, range: usize) -> Option<Array2<usize>>{
        if self.token_n > 0 {
            let size = land.shape()[0];
            let mut neighbors: Vec<Point> = get_neighborhood(&self.position, size)
                .iter()
                .cloned()
                .filter(|p| land[[p.x as usize, p.y as usize]] < sea_level)
                .collect();
            neighbors.sort_by(|a, b| self.score(a, size).partial_cmp(&self.score(b, size)).unwrap());
            neighbors.reverse();
            if neighbors.len() > 0 {
                let mut new_land = land.clone();
                let p = neighbors[0];
                new_land[[p.x as usize, p.y as usize]] = sea_level + range;
                self.token_n -= 1;
                self.position = p;
                //println!("{}", self.token_n);
                return Some(new_land);
            }
            self.position = self.position.add(&self.direction);
            //println!("{:?}", self.direction);
            return match check_point(size, self.position){
                Some(_) => Some(land.clone()),
                None => {
                    self.active = false;
                    None
                } 
            }
        }
        self.active = false;
        None
    }

    pub fn from_parent(parent: &CoastlineAgent, size: usize) -> CoastlineAgent{
        CoastlineAgent{
            token_n: parent.token_n/2,
            direction: {
                *thread_rng().choose(&get_directions()).unwrap()
            },
            position: {
                let p = *thread_rng().choose(&get_neighborhood(&parent.position, size)).unwrap();
                println!("{:?}", p);
                p
            },
            attractor: Point::random(size),
            repulsor: Point::random(size),
            active: true,
        }
    }
}