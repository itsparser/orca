pub use sea_orm;

pub mod prelude;
pub mod role;
pub mod role_scope;
pub mod user;
pub mod user_role;
pub mod user_session;

pub mod audit_log;

pub mod activity;
pub mod group;
pub mod profile;
pub mod profile_data;
pub mod test_action;
pub mod test_case;
pub mod test_step;
pub mod user_group;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
