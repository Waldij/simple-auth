use std::{fmt};

use ansi_term::Colour::Green;
use ansi_term::Colour::Red;

#[derive(Clone)]
pub(crate) struct User
{
    login: String,
    password: String,
}

impl User 
{
    pub fn new(login: String, password: String) -> User 
    {
        User { login, password }
    }
}

impl fmt::Display for User
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        write!(f, "user structure with login: {}", self.login)
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.login == other.login && self.password == other.password
    }
}

pub(crate) struct UsersStorage
{
    users: Vec<User>,
}

impl UsersStorage
{
    pub fn new() -> UsersStorage
    {
        let users = Vec::new();
        UsersStorage {users}
    }

    pub fn auth(&mut self, user: User) -> bool { return self.user_exists(user) }

    fn user_exists(&mut self, user: User) -> bool { return self.users.contains(&user); }

    fn login_exists(&mut self, user:User) -> bool
    {
        for current_user in &self.users
        {
            if user.login == current_user.login{ return true; }
        }
        return false;
    }

    pub fn add_user(&mut self, user: User) -> bool
    { 
        if self.login_exists(user.clone())
        {
            println!("{}", Red.paint("User already exists!"));
            return false;
        } 
        else
        {
            self.users.push(user);
            println!("{}", Green.paint("User was created!"));
            return true; 
        }
    }
}
