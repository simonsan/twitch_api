// tests/credentials.rs
// Export hardcoded example credentials and read them back in

extern crate libtwitch_rs;

use libtwitch_rs::Credentials;

#[test]
fn credential_export() {
    let cred = Credentials {
        client_id: "13211542".to_string(),
        // channel_id: "31244131".to_string(),
        token: "OAuth:1839213891u389u1389183139".to_string(),
    };

    Credentials::write_to_file(
        &cred,
        "tests/example_credentials.toml".to_string(),
    );
}

#[test]
fn credential_import() {
    let cred = Credentials::set_from_file(
        "tests/example_credentials.toml".to_string(),
    );
    assert_eq!(cred.client_id, "13211542".to_string());
    assert_eq!(cred.token, "OAuth:1839213891u389u1389183139".to_string());
}
