#[cfg(test)]
mod tests {
    use crate::{
        enums::ClientRole,
        models::{NewClient, UpdateClient},
        services::clients::{
            create_client, edit_client, get_client, get_client_email, get_clients,
        },
        tests::db_test::tests::setup_test_db,
        utils::verify_hash,
    };

    #[test]
    fn test_create_and_get_client() {
        let mut conn = setup_test_db();

        let new_client = NewClient {
            id: "",
            email: "test@example.com",
            password: "password123",
            role: &ClientRole::Admin,
            created_at: None,
            updated_at: None,
        };

        let client = create_client(&mut conn, &new_client).expect("Failed to create client");
        let client2 = create_client(&mut conn, &new_client);

        let retrieved_client =
            get_client(&mut conn, &client.id).expect("Failed to retrieve client by Id");

        let retrieved_client_by_email =
            get_client_email(&mut conn, &client.email).expect("Failed to retrieve client by email");

        let is_password_valid = verify_hash(&client.password, new_client.password)
            .expect("Failed to verify password hash");

        assert_eq!(client.email, "test@example.com");
        assert_eq!(client.role, ClientRole::Admin);
        assert!(is_password_valid);
        assert!(client.created_at.is_some());
        assert!(client.updated_at.is_some());

        assert_eq!(retrieved_client.id, client.id);
        assert_eq!(retrieved_client.email, "test@example.com");
        assert_eq!(retrieved_client.role, ClientRole::Admin);
        assert!(retrieved_client.created_at.is_some());
        assert!(retrieved_client.updated_at.is_some());

        assert_eq!(retrieved_client_by_email.id, client.id);
        assert_eq!(retrieved_client_by_email.email, "test@example.com");
        assert_eq!(retrieved_client_by_email.role, ClientRole::Admin);
        assert!(retrieved_client_by_email.created_at.is_some());
        assert!(retrieved_client_by_email.updated_at.is_some());

        assert!(client2.is_err())
    }

    #[test]
    fn test_inavlid_email() {
        let mut conn = setup_test_db();

        let new_client = NewClient {
            id: "",
            email: "testexample",
            password: "password123",
            role: &ClientRole::Admin,
            created_at: None,
            updated_at: None,
        };

        let client = create_client(&mut conn, &new_client);
        assert!(client.is_err());
    }

    #[test]
    fn test_get_nonexistent_client() {
        let mut conn = setup_test_db();

        let result = get_client(&mut conn, "nonexistent-id");

        assert!(result.is_err());
    }

    #[test]
    fn test_get_client_with_invalid_id() {
        let mut conn = setup_test_db();

        let result = get_client(&mut conn, "");

        assert!(result.is_err());
    }

    #[test]
    fn test_get_clients() {
        let mut conn = setup_test_db();

        let new_client_1 = NewClient {
            id: "",
            email: "test1@example.com",
            password: "password123",
            role: &ClientRole::Admin,
            created_at: None,
            updated_at: None,
        };

        let new_client_2 = NewClient {
            id: "",
            email: "test2@example.com",
            password: "password456",
            role: &ClientRole::Developer,
            created_at: None,
            updated_at: None,
        };

        let client_1 = create_client(&mut conn, &new_client_1).expect("Failed to create client 1");
        let client_2 = create_client(&mut conn, &new_client_2).expect("Failed to create client 2");

        let clients = get_clients(&mut conn).expect("Failed to retrieve clients");

        assert_eq!(clients.len(), 2);

        assert_eq!(clients[0].id, client_1.id);
        assert_eq!(clients[0].email, "test1@example.com");
        assert_eq!(clients[0].role, ClientRole::Admin);

        assert_eq!(clients[1].id, client_2.id);
        assert_eq!(clients[1].email, "test2@example.com");
        assert_eq!(clients[1].role, ClientRole::Developer);
    }

    #[test]
    fn test_update_client() {
        let mut conn = setup_test_db();

        let new_client = NewClient {
            id: "",
            email: "test@example.com",
            password: "password123",
            role: &ClientRole::Admin,
            created_at: None,
            updated_at: None,
        };

        let created_client =
            create_client(&mut conn, &new_client).expect("Failed to create client");

        let updated_client = UpdateClient {
            email: "updated@example.com",
            role: &ClientRole::Developer,
            updated_at: None,
        };

        let result = edit_client(&mut conn, &created_client.id, &updated_client)
            .expect("Failed to update client");

        assert_eq!(result.email, "updated@example.com");
        assert_eq!(result.role, ClientRole::Developer);
    }
}
