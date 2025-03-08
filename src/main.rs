use dioxus::prelude::*;

use dioxus_query::prelude::{use_get_query, use_init_query_client, use_query_client, QueryResult, QueryState};
use serde::{Deserialize, Serialize};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize, Serialize)]
enum QueryKeys {
    Data,
    IdData {
        id: i32
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum QueryValue {
    Data(String)
}

#[component]
fn App() -> Element {
    let client = use_init_query_client::<QueryValue, ServerFnError, QueryKeys>();
    let mut ids = use_signal(|| 3);

    let increase = move |_| {
        *ids.write() += 1;
    };

    let decrease = move |_| {
        *ids.write() -= 1;
    };

    let refresh_all = move |_| {
        client.invalidate_queries(&[QueryKeys::Data]);
    };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        button {
            onclick: increase,
            "Increase"
        }

        button {
            onclick: decrease,
            "Decrease"
        }

        button {
            onclick: refresh_all,
            "Refresh all"
        }
        
        for id in 0..ids() {
            Fragment {
                key: "{id}",
                Visualizer {
                    id,
                }
                Visualizer {
                    id,
                }
                Visualizer {
                    id,
                }
                Refresh { 
                    id
                }
            }
        }
    }
}

#[component]
fn Visualizer(id: i32) -> Element {
    let data = use_get_query([QueryKeys::IdData { id }, QueryKeys::Data], fetch_data);

    rsx!(
        match data.result().value() {
            QueryState::Settled(QueryResult::Ok(QueryValue::Data(data))) => rsx!(
                p {
                    "{data}"
                }
            ),
            QueryState::Settled(QueryResult::Err(err)) => rsx!(
                p { "Error:{err}" }
            ),
            QueryState::Loading(Some(QueryValue::Data(data))) => rsx!(
                p { "{data} (loading)" }
            ),
            _ => rsx!(
                p {
                    "Loading..."
                }
            ),
        }
    )
}

#[component]
fn Refresh(id: i32) -> Element {
    let client = use_query_client::<QueryValue, ServerFnError, QueryKeys>();

    let onclick = move |_| {
        client.invalidate_queries(&[QueryKeys::IdData { id }]);
    };

    rsx!(
        button {
            onclick,
            "Refresh {id}"
        }
    )
}

#[server(FetchData)]
async fn fetch_data(keys: Vec<QueryKeys>) -> Result<QueryValue, ServerFnError> {
    use std::time::{SystemTime, UNIX_EPOCH, Duration};
    use tokio::time::sleep;

    if let Some(QueryKeys::IdData { id }) = keys.get(0) {
        sleep(Duration::from_secs(1)).await;
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        Ok(QueryValue::Data(format!("{since_the_epoch:?} (data from {id})")))
    } else {
        Err(ServerFnError::new("Missing query keys"))
    }
}