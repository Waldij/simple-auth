use std::{thread, time};

use ansi_term::Colour::Green;
use ansi_term::Colour::Red;
use ansi_term::Colour::Yellow;
use ansi_term::Colour::Blue;

extern crate rpassword;
use rpassword::read_password;

use crate::user;

pub(crate) struct SimpleAuth
{
    number_of_tries: i32,
    current_number_of_tries: i32,
    timeout: time::Duration,
    users_storage: user::UsersStorage,
}

impl SimpleAuth 
{

    pub fn new(number_of_tries: i32, timeout: time::Duration, users_storage: user::UsersStorage) -> SimpleAuth
    {
        if number_of_tries <= 0 { println!("Incorrect number of tries!") }
        let current_number_of_tries = 0;
        SimpleAuth {
            number_of_tries, current_number_of_tries, timeout, users_storage
        }
    }

    fn auth_loop(&mut self)
    {
        loop
        {
            self.check_number_of_tries();
            self.try_to_authorize();
        }
    }

    fn try_to_authorize(&mut self)
    {
        println!("{}", Yellow.paint("+========Authorization=========+"));
        self.current_number_of_tries += 1;
        let login = self.get_auth_login();
        let password = self.get_auth_password();
        let user_to_auth = user::User::new(login, password);
        if self.users_storage.auth(user_to_auth)
        {
            println!("{}", Green.paint("Success!"));
            self.reset_number_of_tries()
        }
        else {
            println!("{}", Red.paint("Authorization failed!"));
        }
    }

    fn check_number_of_tries(&mut self)
    {
        if self.current_number_of_tries >= self.number_of_tries 
        {
            println!("{}", Red.paint("You have exceeded the number of authorization attempts!"));
            println!("{}", Red.paint("Please try again after a while..."));
            thread::sleep(self.timeout);
            self.reset_number_of_tries()
        }
    }

    fn reset_number_of_tries(&mut self){ self.current_number_of_tries = 0; }

    fn set_up_user(&self) -> user::User
    {
        println!("{}", Blue.paint("+=========User=creation========+"));
        let login = self.get_auth_login();
        let password = self.get_auth_password();
        let user = user::User::new(login, password);
        println!("User was created!");
        return user;
    }

    fn create_users(&mut self) 
    {
        loop {
            let created_user = self.set_up_user();
            self.users_storage.add_user(created_user);
            let answer = self.get_more_users_answer();
            let no = "n".to_string();
            if answer.contains(&no) { break; }
        }
    }

    fn get_auth_login(&self) -> String
    {
        return self.read_after_msg("Enter login:".to_string());
    }

    fn get_auth_password(&self) -> String
    {
        println!("Enter password:");
        return read_password().unwrap()
    }

    fn get_more_users_answer(&self) -> String
    {
        return self.read_after_msg("Need more users? (y/n)".to_string());
    }

    fn read_after_msg(&self, msg: String) -> String
    {
        let mut value = String::new();
        println!("{}", msg);
        std::io::stdin().read_line(&mut value).unwrap();
        value.retain(|c| !c.is_whitespace());
        return value;
    }

    pub fn run(&mut self) 
    {
        self.create_users();
        self.auth_loop();
    }
}
