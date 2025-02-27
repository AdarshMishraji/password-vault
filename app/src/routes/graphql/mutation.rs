use async_graphql::Object;

pub struct Mutation;

#[Object]
impl Mutation {
    // ********************* AUTH ************************//
    async fn signup(&self) -> String {
        "Signup!".to_string()
    }

    async fn login(&self) -> String {
        "Login!".to_string()
    }
    // ********************* AUTH ************************//

    // ********************* PASSWORD ************************//
    async fn add_password(&self) -> String {
        "Add Password".to_string()
    }

    async fn delete_password(&self) -> String {
        "Delete Password".to_string()
    }

    async fn update_password(&self) -> String {
        "Update Password".to_string()
    }
    // ********************* PASSWORD ************************//
}
