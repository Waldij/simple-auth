use core::time;

pub mod user;
pub mod simple_auth;

fn main() {
    let users_storage = user::UsersStorage::new();
    let mut auth = simple_auth::SimpleAuth::new(
        5,
        time::Duration::from_secs(10), 
        users_storage);
    auth.run()
}
