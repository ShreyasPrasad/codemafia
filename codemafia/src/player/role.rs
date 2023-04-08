use std::fmt;

use crate::messages::game::Team;

#[derive(Debug, Eq, PartialEq)]
pub struct CodeMafiaRole {
    pub role_title: Option<CodeMafiaRoleTitle>,
    pub team: Team
}

impl CodeMafiaRole {
    pub fn get_role_str(&self) -> String {
        match &self.role_title {
            Some(title) => {
                title.to_string() + &self.team.to_string()
            },
            None => {
                self.team.to_string()
            }
        }
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
