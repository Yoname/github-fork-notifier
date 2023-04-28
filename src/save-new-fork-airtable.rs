use airtable_flows::create_record;
use github_flows::{listen_to_event, EventPayload};
use slack_flows::send_message_to_channel;
#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    let login = "yoname";
    let owner = "yomorun";
    let repo = "presencejs";

    listen_to_event(login, owner, repo, vec!["fork"], handler).await;
}

async fn handler(payload: EventPayload) {
    let account: &str = "github";
    let base_id: &str = "appN337YR9Lt67MxA";
    let table_name: &str = "fork";

    if let EventPayload::ForkEvent(e) = payload {
        let forkee = e.forkee;
        let name = forkee.owner.unwrap().login;
        let html_url = forkee.html_url.unwrap().to_string();
        let time = forkee.created_at.expect("time not found");

        let text = format!("{} forked your {}\n{}", name, html_url, time);
        send_message_to_channel("secondstate", "github-status", text);

        let data = serde_json::json!({
        "Name": name,
        "Repo": html_url,
        "Created": time,
        });
        create_record(account, base_id, table_name, data)
    }
}
