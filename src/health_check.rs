use rocket::http::Status;

#[get("/health_check")]
pub fn health_check() -> Status {
    Status::Ok
}

#[cfg(test)]
mod tests {
    use crate::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn health_check_ok() {
        let client = Client::tracked(rocket()).unwrap();
        let response = client.get("/health_check").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response.into_string().is_none());
    }
}
