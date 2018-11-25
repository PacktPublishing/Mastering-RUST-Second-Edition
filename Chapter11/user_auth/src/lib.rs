#[macro_use]
extern crate log;

pub struct User {
    name: String,
    pass: String,
}

impl User {
    pub fn new(name: &str, pass: &str) -> Self {
        User {
            name: name.to_string(),
            pass: pass.to_string(),
        }
    }

    pub fn sign_in(&self, pass: &str) {
        debug!("Attempting sign in !");
        if pass != self.pass {
            info!("Signing in user: {}", self.name);
        } else {
            error!("Login failed for user: {}", self.name);
        }
    }
}
