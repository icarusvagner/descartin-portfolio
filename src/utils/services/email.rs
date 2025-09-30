#[cfg(feature = "ssr")]
use lettre::{message::header, Message, SmtpTransport, Transport};
#[cfg(feature = "ssr")]
use maud::html;
#[cfg(feature = "ssr")]
use crate::utils::services::types::{Result, EmailError};

#[cfg(feature = "ssr")]
pub fn generate_email_body(email_add: String, name: String, message: String) -> String {
    html! {
		 head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width";
            meta http-equiv="X-UA-Compatible" content="IE=edge";
            meta name="x-apple-disable-message-reformatting";
            meta name="format-detection" content="telephone=no,address=no,email=no,date=no,url=no";
            meta name="color-scheme" content="light";
            meta name="supported-color-schemes" content="light";

            link rel="preload" as="style" href="https://fonts.googleapis.com/css2?family=Open+Sans:wght@400;700&family=Lato:wght@400;700&display=swap";
            link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Open+Sans:wght@400;700&family=Lato:wght@400;700&display=swap";

            style { r#"
                :root { color-scheme: light; supported-color-schemes: light; }
                body { margin: 0 auto; padding: 0; height: 100%; width: 100%; background-color: #F5F6F8; }
                .heading2 { font-size: 26px; font-family: Open Sans, sans-serif; color: #000; }
                .heading3 { font-size: 19px; font-family: Open Sans, sans-serif; color: #000; }
                .paragraph { font-size: 15px; font-family: Open Sans, sans-serif; color: #5f5f5f; }
            "# }
        }

        body width="100%" style="margin:0; padding:0; background-color:#F5F6F8;" {
                center role="article" aria-roledescription="email" lang="en" style="width:100%; background-color:#F5F6F8;" {
                    table align="center" role="presentation" cellspacing="0" cellpadding="0" border="0" width="640" style="margin:auto;" class="contentMainTable" {
                        tr { td style="background-color:#F5F6F8; line-height:50px; font-size:50px; width:100%;"; }

                        // --- Header
                        tr {
                            td style="background-color:#554e4e; padding:52px 32px 16px 32px; text-align:center;" {
                                p.heading2 style="color:#EFEFEF; font-family:Open Sans, sans-serif;" { "New Message" }
                            }
                        }

                        // --- Divider
                        tr {
                            td style="padding:20px; background-color:#ffffff;" {
                                div style="background:#718096; font-size:1px; line-height:1px;";
                            }
                        }

                        // --- Full Name
                        tr {
                            td style="background-color:#ffffff;" {
                                table class="multi-column" width="640" {
                                    tr {
                                        td width="213" {
                                            p.paragraph style="font-family:Lato, sans-serif; font-size:16px; color:#5f5f5f;" {
                                                span style="font-weight:bold;" { "Full Name" } ":"
                                            }
                                        }
                                        td width="426" {
                                            p.heading3 style="font-family:Lato, sans-serif; font-size:19px; color:#554E4E;" {
                                                (name)
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // --- Email
                        tr {
                            td style="background-color:#ffffff;" {
                                table class="multi-column" width="640" {
                                    tr {
                                        td width="213" {
                                            p.paragraph { span style="font-weight:bold;" { "Email Address:" } }
                                        }
                                        td width="426" {
                                            p.heading3 style="font-family:Open Sans, sans-serif; font-size:19px; color:#554E4E;" {
                                                (email_add)
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // --- Message
                        tr {
                            td style="background-color:#ffffff;" {
                                table class="multi-column" width="640" {
                                    tr {
                                        td width="213" {
                                            p.paragraph { span style="font-weight:bold;" { "Message:" } }
                                        }
                                        td width="426" {
                                            p.paragraph style="font-family:Open Sans, sans-serif; font-size:15px; color:#554E4E;" {
                                                (message)
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // --- Footer Divider
                        tr {
                            td style="padding:20px; background-color:#ffffff;" {
                                div style="background:#718096; font-size:1px; line-height:1px;" { "&nbsp;" }
                            }
                        }

                        // --- Social Icons
                        tr {
                            td style="background-color:#F5F6F8; padding:42px 32px 32px;" {
                                table width="100%" {
                                    tr {
                                        td align="center" {
                                            a href="https://youtube.com/@cnalecoding" target="_blank" {
                                                img src="https://d2u6lzrmbvw8bs.cloudfront.net/assets/social-icons/youtube/youtube-round-solid-color.png"
                                                    width="32" height="32" alt="Youtube";
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
	}.into_string()
}

#[cfg(feature = "ssr")]
pub async fn send_email(email_add: String, name: String, message: String, subject: String) -> Result<String> {
    use lettre::transport::smtp::authentication::Credentials;

    use crate::utils::helpers::get_env_variable;

    let gmail_smtp_host = get_env_variable("GMAIL_SMTP_HOST").expect("GMAIL_SMTP_HOST not found");
    let gmail_smtp_user = get_env_variable("GMAIL_SMTP_USER").expect("GMAIL_SMTP_USER not found");
    let gmail_smtp_pass = get_env_variable("GMAIL_SMTP_PASS").expect("GMAIL_SMTP_PASS not found");
    let email_to = get_env_variable("EMAIL_TO").expect("EMAIL_TO not found");

    let email_body = generate_email_body(email_add.clone(), name, message);

    let email = Message::builder()
        .from(email_add.parse().unwrap())
        .to(email_to.parse().unwrap())
        .subject(subject)
        .header(header::ContentType::TEXT_HTML)
        .body(email_body)
        .unwrap();
    let credentials = Credentials::new(gmail_smtp_user.to_owned(), gmail_smtp_pass.to_owned());
    let mailer = SmtpTransport::relay(&gmail_smtp_host)
        .unwrap()
        .credentials(credentials)
        .build();

    match mailer.send(&email) {
        Ok(_) => Ok("Email sent successfully".to_string()),
        Err(err) => Err(EmailError::FailedToSend(err.to_string())),
    }
}
