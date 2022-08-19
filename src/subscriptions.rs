use rocket::form::Form;
use rocket::http::Status;

#[allow(dead_code, clippy::unnecessary_lazy_evaluations)]
#[derive(FromForm)]
pub struct FormData<'f> {
    email: &'f str,
    name: &'f str,
}

#[post("/subscriptions", data = "<_form_data>")]
pub fn subscribe(_form_data: Form<FormData<'_>>) -> Status {
    Status::Ok
}

#[cfg(test)]
mod tests {
    use crate::rocket;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;

    #[test]
    fn subscribe_returns_200_for_valid_form_data() {
        let client = Client::tracked(rocket()).unwrap();
        let response = client
            .post("/subscriptions")
            .header(ContentType::Form)
            .body("name=le%20guin&email=ursula_le_guin%40gmail.com")
            .dispatch();
        println!("{response:?}");
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn subscribe_returns_a_422_when_data_is_missing() {
        let client = Client::tracked(rocket()).unwrap();
        let test_cases = vec![
            ("name=le%20guin", "missing the email"),
            ("email=ursula_le_guin%40gmail.com", "missing the name"),
            ("", "missing both name and email"),
        ];

        for (invalid_body, error_message) in test_cases {
            let response = client
                .post("/subscriptions")
                .header(ContentType::Form)
                .body(invalid_body)
                .dispatch();

            assert_eq!(
                response.status(),
                Status::UnprocessableEntity,
                "The API did not fail with 422 Unprocessable Entity when the payload was {error_message}"
            );
        }
    }
}
