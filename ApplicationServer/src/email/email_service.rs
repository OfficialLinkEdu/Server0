pub mod email_service {

    use axum::body::Body;
    use axum::extract::State;
    use axum::response::Response;
    use axum::{routing::post, Json, Router};
    use lettre::message::header::ContentType;
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};

    use crate::endpoints::occ_ip;
    use crate::AppState;
    use reqwest::StatusCode;
    use serde::Deserialize;
    #[derive(Deserialize, Debug)]
    struct EmailUserDocument {
        code: String,
        school_email: String,
        school_code: String,
    }

    async fn send_email_verification(
        State(state): State<AppState>,
        Json(data): Json<EmailUserDocument>,
    ) -> Response {
        // Make sure the respective school email dosen't already exist

        let res: StatusCode = state
            .http_client
            .get(format!(
                "http://{}/{}/helpers/schoolEmailExists/{}",
                occ_ip, data.school_code, data.school_email
            ))
            .send()
            .await
            .unwrap()
            .status();
        if res == StatusCode::OK {
            // email exists alreadyh
            return Response::builder()
                .status(axum::http::StatusCode::CONFLICT)
                .body(axum::body::Body::from(
                    "This email is already associated with an account",
                ))
                .unwrap();
        } else if res == StatusCode::NOT_FOUND {
            let email_body = format!(
                r#"
        <html>
        <head>
            <style>
                /* Add any styling you want for the header here */
                .header {{
                    font-size: 24px;
                    font-weight: bold;
                    color: #3366cc; /* Change the color to your preference */
                }}
            </style>
        </head>
        <body>
            <div class="header">LinkEdu</div>
            <p>Hi there,</p>
            <p>Verify yourself in the app with the following code: <strong>{verification_code}</strong></p>
        </body>
        </html>
        "#,
                verification_code = data.code /* Replace this with the actual verification code variable */
            );

            let email = Message::builder()
                .from("coastlinkedu@gmail.com".parse().unwrap())
                .to(data.school_email.parse().unwrap())
                .subject("Your Verification Code")
                .header(ContentType::TEXT_HTML)
                .body(email_body)
                .unwrap();

            //  let creds = Credentials::new("costlinkedu@gmail.com".to_owned(), "bmvr ysqk fhpx rcio".to_owned());
            let creds: Credentials = Credentials::new(
                "coastlinkedu@gmail.com".to_owned(),
                "bmvr ysqk fhpx rcio".to_owned(),
            );

            let mailer = SmtpTransport::relay("smtp.gmail.com")
                .unwrap()
                .credentials(creds)
                .build();

            // Send the email
            match mailer.send(&email) {
                Ok(_) => {
                    println!("Email sent successfully!");
                    return Response::builder()
                        .status(axum::http::StatusCode::OK)
                        .body(Body::from(""))
                        .unwrap();
                }
                Err(e) => {
                    return Response::builder()
                        .status(axum::http::StatusCode::CONFLICT)
                        .body(Body::from(e.status().unwrap().to_string()))
                        .unwrap();
                }
            }
        } else {
            return Response::builder()
                .status(axum::http::StatusCode::CONFLICT)
                .body(Body::from("Something went wrong"))
                .unwrap();
        }
        //  println!("{res}");
        //   println!("{:?}\n{:?}", data.code, data.school_email);
    }

    pub fn email_routers() -> Router<AppState> {
        Router::new().route("/sendEmailVerification", post(send_email_verification))
    }
}
