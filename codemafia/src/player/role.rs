use std::fmt;

use crate::messages::game::Team;

/* Trait that designates a player's profile; this can be used for whatever 
purpose the game requires; in our case it will store the player's name and role. */
pub trait Role {
    fn get_role_str(&self) -> String;
}

#[derive(Eq, PartialEq)]
pub struct CodeMafiaRole {
    role_title: CodeMafiaRoleTitle,
    team: Team
}

impl Role for CodeMafiaRole {
    fn get_role_str(&self) -> String {
        self.role_title.to_string() + &self.team.to_string()
    }
}

impl fmt::Display for CodeMafiaRoleTitle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum CodeMafiaRoleTitle {
    SpyMaster,
    Undercover,
    Ally
}
