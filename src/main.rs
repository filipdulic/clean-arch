use clean_arch::domain::entity::{
    signup_process::{Id as SpId, SignupProcess},
    user::{Email, User, UserName},
};

use uuid::Uuid;

fn main() {
    let sp_id = SpId::new(Uuid::new_v4());
    let username = UserName::new("Fica".to_string());
    let email = Email::new("fica@ja.com".to_string());

    let signup_process = SignupProcess::new(sp_id, username);
    let signup_process = signup_process.add_email(email);
    let signup_process = signup_process.complete();

    let user: User = User::from(signup_process);
    println!("{:?}", user);
}
