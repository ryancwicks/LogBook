mod test {
    use crate::rocket;
    use rocket::local::Client;
    use rocket::http::{Status, ContentType};
    use serde_json;

    use crate::api::StandardResponse;

    #[test]
    fn test_static_pages_get() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response.body_string() != None);

        response = client.get("/static/style.css").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response.body_string() != None);

        response = client.get("/static/favicon.ico").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_get_all() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.post("/api/add_item")
                                                .header(ContentType::JSON)
                                                .body(r#"{"log":"This is a test log."}"#)
                                                .dispatch();
        assert_eq!(response.status(), Status::Ok);
        response = client.get("/api/all")
                                                .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let response_str = response.body_string().expect("Failed to recieve response.");
        let json_resp: StandardResponse = serde_json::from_str(&response_str).expect("Failed to parse json string.");
        assert!(json_resp.success);
        let record = json_resp.records.unwrap();
        assert!(record.len() > 0);
    }

    #[test]
    fn test_add_log(){
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.post("/api/add_item")
                                                .header(ContentType::JSON)
                                                .body(r#"{"log":"This is a test log."}"#)
                                                .dispatch();

        assert_eq!(response.status(), Status::Ok);
        let response_str = response.body_string().expect("Failed to recieve response.");
        let json_resp: StandardResponse = serde_json::from_str(&response_str).expect("Failed to parse json string.");
        println!("{}", response_str);
        assert!(json_resp.success);
        let record = json_resp.records.unwrap();
        assert_eq!(record.len(), 1);
        assert_eq!(record[0].body, "This is a test log.");
    }

    #[test]
    fn test_add_invalid_log() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.post("/api/add_item")
                                        .header(ContentType::JSON)
                                        .body(r#"{"not_a_log":"This won't work."}"#).dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }


}