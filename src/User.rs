use std::fmt;

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

    pub fn auth(&self, user: User) -> bool { return self.users.contains(&user); }

    pub fn add_user(& mut self, user: User) { self.users.push(user); }
}
