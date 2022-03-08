use gnap_cli::models::interact::InteractResponse;
use gnap_cli::models::request::RequestContinuation;
use gnap_cli::models::response::GrantResponse;
use uuid::Uuid;

#[test]
fn request_continuation() {
    let tx_id = Uuid::new_v4().to_string();

    let uri = format!("http://localhost:8000/tx/{}", &tx_id);
    let rc = RequestContinuation::as_uri(&uri.clone());

    let ic = InteractResponse {
        tx_continue: rc,
        redirect: Some(uri),
    };

    let response = GrantResponse {
        instance_id: tx_id,
        interact: Some(ic),
        access: None,
    };

    println!("{}", serde_json::to_string(&response).expect("oops"));
    assert!(true);
}
