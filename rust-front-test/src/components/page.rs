use gloo_net::http::Request;
use log::info;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageProp {
    pub id: AttrValue,
}

#[function_component]
pub fn Page(props: &PageProp) -> Html {
    let page = use_state(|| "".to_string());
    let page_link = page.clone();
    let id = props.id.clone();

    wasm_bindgen_futures::spawn_local(async move {
        let url = format!("http://localhost:8081/{}", id);
        Request::get(&url)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .map(|x: PageResponse| page_link.set(x.page))
            .expect("not found");
    });

    html! { <p> { page.to_string() } </p> }
}

#[derive(Debug, Deserialize)]
struct PageResponse {
    page: String,
}
