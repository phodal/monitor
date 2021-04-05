use image::{Rgb};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};
use std::path::Path;
use image::ImageBuffer;

use serde::{Serialize, Deserialize};
use reqwest::Client;

extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
extern crate google_calendar3 as calendar3;

use calendar3::api::Channel;
use std::default::Default;
use calendar3::{CalendarHub, Error};
use hyper::{Response, Body};


#[tokio::main]
async fn main() {
    // Get an ApplicationSecret instance by some means. It contains the `client_id` and
    // `client_secret`, among other things.
    let mut secret: oauth2::ApplicationSecret = Default::default();
    secret.client_id = "".to_string();
    secret.client_secret = "".to_string();
    secret.auth_uri = "https://accounts.google.com/o/oauth2/auth".to_string();
    secret.project_id = Some("".to_string());
    secret.auth_provider_x509_cert_url = Some("https://www.googleapis.com/oauth2/v1/certs".to_string());
    secret.redirect_uris = vec!["http://localhost".to_string()];

    // Instantiate the authenticator. It will choose a suitable authentication flow for you,
    // unless you replace  `None` with the desired Flow.
    // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
    // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
    // retrieve them from storage.
    let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    ).build().await.unwrap();
    let mut hub = CalendarHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
    // As the method needs a request, you would usually fill it with the desired information
    // into the respective structure. Some of the parts shown here might not be applicable !
    // Values shown here are possibly random and not representative !
    let mut req = Channel::default();

    // You can configure optional parameters by calling the respective setters at will, and
    // execute the final call using `doit()`.
    // Values shown here are possibly random and not representative !
    let result = hub.calendar_list()
        .list()
        .doit()
        .await;
        // .doit().await;

    match result {
        Err(e) => match e {
            // The Error enum provides details about what exactly happened.
            // You can also just use its `Debug`, `Display` or `Error` traits
            Error::HttpError(_)
            | Error::Io(_)
            | Error::MissingAPIKey
            | Error::MissingToken(_)
            | Error::Cancelled
            | Error::UploadSizeLimitExceeded(_, _)
            | Error::Failure(_)
            | Error::BadRequest(_)
            | Error::FieldClash(_)
            | Error::JsonDecodeError(_, _) => {
                println!("{}", e);
            },
        },
        Ok(res) => {
            println!("{:?}", res);
        },
    };
}

fn write_text() {
    let path = Path::new("monitor.bmp");

    let mut image = ImageBuffer::from_pixel(1280, 825, Rgb([255, 255, 255]));

    let font = Vec::from(include_bytes!("wqy-microhei.ttc") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let height = 80.0;
    let scale = Scale {
        x: height,
        y: height,
    };

    let text = "你好，世界。";
    draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), 0, 0, scale, &font, text);
    let (w, h) = text_size(scale, &font, text);
    println!("Text size: {}x{}", w, h);

    let _ = image.save(path).unwrap();
}
