use crate::components::page::Page;
use crate::components::post::Post;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Routed> render={switch} />
        </BrowserRouter>
    }
}

#[derive(Clone, Routable, PartialEq)]
enum Routed {
    #[at("/:id")]
    Page { id: String },
    #[at("/post")]
    Post,
}

fn switch(routes: Routed) -> Html {
    match routes {
        Routed::Post => html! { <Post /> },
        Routed::Page { id } => html! { <Page id= { id } /> },
    }
}
