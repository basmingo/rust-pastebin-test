use gloo_net::http::Request;
use log::info;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

#[function_component]
pub fn Post() -> Html {
    let change_page = use_state(|| String::from(""));
    let link = use_state(|| String::from(""));
    let page = (*change_page).clone();
    let display_link = (*link).clone();

    let chaga = {
        Callback::from(move |e: Event| {
            let result = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok())
                .unwrap()
                .value();

            info!("{:?}", result);

            change_page.set(result.clone());
        })
    };

    let baga = {
        Callback::from(move |_: MouseEvent| {
            let page_request = PageRequest {
                data: page.clone(),
                user: "user".to_string(),
            };

            let link = link.clone();

            spawn_local(async move {
                let result: LinkResponse = Request::post("http://localhost:8081/page")
                    .json(&page_request)
                    .unwrap()
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                link.set(format!("localhost:8080/{}", result.link));
            })
        })
    };

    html! {
        <div>
            <p> { "write your thoughts..." } </p>
            <textarea type="text" onchange = { chaga }/>
            <button type="button" onclick = { baga }> { "pin" } </button>
            <p> { display_link } </p>
        </div>
    }
}

#[derive(Deserialize)]
struct LinkResponse {
    link: String,
}

#[derive(Serialize)]
struct PageRequest {
    data: String,
    user: String,
}
