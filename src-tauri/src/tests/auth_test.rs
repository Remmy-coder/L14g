#[cfg(test)]
mod tests {
    use std::env;

    use dotenvy::dotenv;

    use crate::{
        enums::ClientRole,
        models::NewClient,
        services::{
            auth::{login, Auth, LoginCredentials},
            clients::create_client,
        },
        tests::db_test::tests::setup_test_db,
    };

    #[test]
    fn test_login_and_auth() {
        dotenv().ok();
        let test_secret = env::var("TEST_SECRET").expect("TEST_SECRET must be set");
        let mut conn = setup_test_db();
        let auth = Auth::new(test_secret.as_bytes());

        let new_client = NewClient {
            id: "",
            email: "test@example.com",
            password: "password123",
            role: &ClientRole::Admin,
            created_at: None,
            updated_at: None,
        };

        let client = create_client(&mut conn, &new_client).expect("Failed to create client");

        let credentials = LoginCredentials {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let token = login(&mut conn, &auth, &credentials).expect("Failed to login");

        let claims = auth.verify_token(&token).expect("Failed to verify token");
        assert_eq!(claims.email, "test@example.com");
        assert_eq!(claims.sub, client.id);
    }

    #[test]
    fn test_invalid_login() {
        dotenv().ok();
        let test_secret = env::var("TEST_SECRET").expect("TEST_SECRET must be set");
        let mut conn = setup_test_db();
        let auth = Auth::new(test_secret.as_bytes());

        let credentials = LoginCredentials {
            email: "nonexistent@example.com".to_string(),
            password: "wrong".to_string(),
        };

        let result = login(&mut conn, &auth, &credentials);
        assert!(result.is_err());
    }
}
