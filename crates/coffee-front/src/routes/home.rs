use crate::app::Route;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();

    let nav = navigator.clone();
    let onclick_order = Callback::from(move |_| nav.push(&Route::Order));
    let onclick_counter = Callback::from(move |_| navigator.push(&Route::Counter));

    html! {
            <>



    <div class="grid is-col-min-10" style="margin-left: 10%;
      margin-right: 10%;">

            <Scard
            image={("https://coffeforus.com/wp-content/uploads/2022/12/Americano-coffee-recipe.jpg".to_string(), "image of coffee".to_string())}
            title="Consumer" description="Order a coffee"
            onclick={onclick_order}
            />

            <Scard
            image={("https://cdn11.bigcommerce.com/s-pmiia69/images/stencil/1030x1030/products/633/3626/BES870XL-2b__34495.1547840804.jpg?c=2".to_string(), "Large coffee machine".to_string())}
            title="Producer"
            description="Manage incoming orders"
            onclick={onclick_counter} />

            </div>

            </>
        }
}

#[derive(Properties, PartialEq)]
pub struct SCardProps {
    // src & alt
    pub image: (String, String),
    pub title: String,
    pub description: String,

    pub onclick: Callback<MouseEvent>,
}

#[function_component(Scard)]
pub fn scard(props: &SCardProps) -> Html {
    html! {


    <div class="card" style="width: 100%; height: 50%" >
      <div class="card-image" >
        <figure class="image is-4by3">
          <img
            src={props.image.0.clone()}
            alt={props.image.1.clone()}
          />
        </figure>
      </div>
      <div class="card-content">
        <div class="media">
          <div class="media-left">
            <figure class="image is-48x48">
              <img
               src={props.image.0.clone()}
            alt={props.image.1.clone()}
              />
            </figure>
          </div>
          <div class="media-content">
            <p class="title is-4">{&props.title}</p>
          </div>
        </div>

        <div class="content">
         {&props.description}
          <br/>
        </div>
      </div>

            <footer class="card-footer">
        <a  class="card-footer-item" onclick={&props.onclick}>{"Go"}</a>
      </footer>
    </div>


        }
}
