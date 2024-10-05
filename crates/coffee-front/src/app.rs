use crate::routes::counter::Counter;
use crate::routes::home::Home;
use crate::routes::order::Order;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/order")]
    Order,
    #[at("/counter")]
    Counter,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// notfound
// home
// order
// counter

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home />},
        Route::Order => {
            html! {
                <Order/>
            }
        }
        Route::Counter => {
            html! {
            <Counter/>
             }
        }

        Route::NotFound => {
            html! {
                <h1>{"404"}</h1>
            }
        }
    }
}
