pub mod email_service {
    use axum::routing::get;
    use axum::{routing::post, Json, Router};
    use lettre::message::header::ContentType;
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct EmailUserDocument {
        code: String,
        email: String,
    }

    async fn send_email_verification(Json(data): Json<EmailUserDocument>) -> &'static str {
        println!("{:?}\n{:?}", data.code, data.email);

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
            .to(data.email.parse().unwrap())
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
                return "Worked";
            }
            Err(e) => {
                println!("Could not send email: {e:?}");
                return "Couldn't work";
            }
        }
    }

    pub fn routers() -> Router {
        Router::new().route("/createUser", post(send_email_verification))
    }
}
