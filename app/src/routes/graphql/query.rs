use async_graphql::Object;

pub struct Query;

#[Object]
impl Query {
    // ********************* AUTH ************************//
    // ********************* AUTH ************************//

    // ********************* PASSWORD ************************//
    async fn all_passwords(&self) -> String {
        "All Passwords".to_string()
    }
    // ********************* PASSWORD ************************//
}
