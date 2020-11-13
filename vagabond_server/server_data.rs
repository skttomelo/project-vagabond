use serde::{Deserialize, Serialize};

use crate::geometry::{Point2, Rect};

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone, Debug)]
pub enum RematchStatus {
    Yes,
    No,
    Maybe,
}

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone, Debug)]
pub enum MatchStatus {
    InProgress,
    Over(usize), // player id will go in the usize
    Rematch(RematchStatus),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerGameMatch {
    pub clock: Clock,
    pub server_entities: Vec<ServerEntity>,
    pub match_status: MatchStatus,
}

impl ServerGameMatch {
    pub fn new() -> ServerGameMatch {
        let ent = ServerEntity::new();
        let ent1 = ServerEntity::new();
        let entity_vector = vec![ent, ent1];

        ServerGameMatch {
            clock: Clock::new(),
            server_entities: entity_vector,
            match_status: MatchStatus::InProgress,
        }
    }

    pub fn update_entity(&mut self, id: usize, player: ServerEntity) {
        if self.server_entities[0].hp > 0
            && self.server_entities[1].hp > 0
            && self.clock.current > 0
        {
            // if player.reset == true {
            //     self.server_entities[id].reset = false;
            // }

            let hp = self.server_entities[id].hp;
            self.server_entities[id] = player;
            self.server_entities[id].hp = hp;

            // check if there is a collision
            for index in 0..self.server_entities.len() {
                if id == index {
                    continue;
                }

                self.attack_bound_check(id, index);
            }
        } else { // match is over
            // this does not work because if the player is the winner then it just returns out of it because they are not dead....
            // if self.server_entities[id].hp > 1 {
            //     return ();
            // }

            // if self.server_entities[id].reset == true {
            //     return ();
            // }
            
            // update player redo_status on server
            self.server_entities[id].redo_status = player.get_redo_status();
            
            match self.match_status {
                MatchStatus::InProgress => {
                    // match is over so we need to say who won the fight
                    self.match_status = MatchStatus::Over(self.get_player_id_most_hp() + 1);
                    
                    // match has just finished so we need to change the redo status to Rematch
                    self.server_entities[id].redo_status = MatchStatus::Rematch(RematchStatus::Maybe);
                },
                _ => (),
            };
        }
    }

    fn get_player_id_most_hp(&self) -> usize {
        if self.server_entities[0].hp > self.server_entities[1].hp {
            return 0;
        }

        1
    }

    pub fn update_clock(&mut self, current_time: u16) {
        self.clock.current = 60 - current_time;
    }

    fn attack_bound_check(&mut self, first_entity_id: usize, second_entity_id: usize) {
        if self.server_entities[first_entity_id]
            .get_entity_actions_as_ref()
            .damage_check
            == true
            && self.server_entities[first_entity_id]
                .get_entity_actions_as_ref()
                .blocking
                == false
            && self.server_entities[second_entity_id]
                .get_entity_actions_as_ref()
                .blocking
                == false
        {
            if self.server_entities[first_entity_id]
                .get_attack_bound()
                .check_bounds(&self.server_entities[second_entity_id].get_bound())
                == true
            {
                self.server_entities[second_entity_id].take_damage(1);
            }
        }
        self.server_entities[first_entity_id]
            .get_entity_actions_as_mut_ref()
            .damage_check = false;
    }

    pub fn restart_match(&mut self) {
        for entity in &mut self.server_entities {
            entity.hp = 5;
            entity.reset = true;
        }

        self.match_status = MatchStatus::InProgress;
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ServerEntity {
    pub id: usize,
    hp: i8, // health of entity
    entity_actions: EntityActions,
    attack_animator: ServerAnimator,
    pos: Point2,
    vel: Point2,
    bound: Rect,
    attack_bound: Rect,
    pub redo_status: MatchStatus,
    reset: bool,
}

impl ServerEntity {
    pub fn new() -> ServerEntity {
        ServerEntity {
            id: 0,
            hp: 5,
            entity_actions: EntityActions::new(Action::Right),
            attack_animator: ServerAnimator::new(),
            pos: Point2::new(0.0, 0.0),
            vel: Point2::new(0.0, 0.0),
            bound: Rect::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)),
            attack_bound: Rect::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)),
            redo_status: MatchStatus::InProgress,
            reset: false,
        }
    }
}

// all possible action states for an entity to be in
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Action {
    Left,
    Right,
}

// helper struct for cleaning up Entity struct
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EntityActions {
    pub facing: Action, // Left or Right
    pub moving_left: bool,
    pub moving_right: bool,
    pub can_attack: bool,
    pub attacking: bool,
    pub damage_check: bool,
    pub blocking: bool,
}

impl EntityActions {
    pub fn new(facing: Action) -> EntityActions {
        EntityActions {
            facing: facing,
            moving_left: false,
            moving_right: false,
            can_attack: true,
            attacking: false,
            damage_check: false,
            blocking: false,
        }
    }
}

impl ServerEntity {
    pub fn take_damage(&mut self, dmg: i8) {
        if self.hp > 0 {
            self.hp -= dmg;
        }
    }

    pub fn get_bound(&self) -> Rect {
        self.bound.clone()
    }

    pub fn get_attack_bound(&self) -> Rect {
        self.attack_bound.clone()
    }

    pub fn get_entity_actions_as_ref(&mut self) -> &EntityActions {
        &self.entity_actions
    }

    pub fn get_entity_actions_as_mut_ref(&mut self) -> &mut EntityActions {
        &mut self.entity_actions
    }

    pub fn get_redo_status_as_ref(&self) -> &MatchStatus {
        &self.redo_status
    }
    
    pub fn get_redo_status(&self) -> MatchStatus {
        self.redo_status.clone()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Clock {
    current: u16,
}
impl Clock {
    pub fn new() -> Clock {
        Clock { current: 0 }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerAnimator {
    current_frame: u64,
    current_repeat: i8,
}

impl ServerAnimator {
    pub fn new() -> ServerAnimator {
        ServerAnimator {
            current_frame: 0,
            current_repeat: 0,
        }
    }
}
