use log::warn;
use reqwasm::http::Response;
use std::env;
use std::ops::Deref;

use shared::OrderPayload;
use yew::prelude::*;

use crate::api;
use crate::api::Client;
use crate::routes::{COFFEE_TYPE, MILK_TYPE, TEMP_TYPE};

const SEL: &'static str = "Select dropdown";




#[function_component(Order)]
pub fn order() -> Html {
    let coffee_type: UseStateHandle<u8> = use_state(|| 0);
    // let coffee_type_e = vec!["Flat white", "Latte", "Mocha"];
    let milk_type: UseStateHandle<u8>  = use_state(|| 0);
    // let milk_type_e = vec!["Cows", "A2", "Almond", "Soy", "None"];
    let temp_type: UseStateHandle<u8>  = use_state(|| 0);
    // let temp_type_e = vec!["Iced", "Meh", "Warm", "Hot", "Extra hot"];

    let coffee_ordered: UseStateHandle<Option<u8>> = use_state(|| None);

    let coffee_ordered_text = match coffee_ordered.is_some() {
        true => "is-active",
        false => "",
    };

    let has_spinner = use_state(|| false);





    html! {<>
            {format!("You want a {} with {} milk, and {}?",
                COFFEE_TYPE[*coffee_type as usize],
                MILK_TYPE[*milk_type as usize],
                TEMP_TYPE[*temp_type as usize],
            )}


            <br/>
            <DropButton label="Coffee type" options={COFFEE_TYPE.into_iter().collect::<Vec<&'static str>>()} selected={coffee_type.clone()}/>
            <DropButton label="Milk type" options={MILK_TYPE.into_iter().collect::<Vec<&'static str>>()} selected={milk_type.clone()}/>
            <DropButton label="Temperature" options={TEMP_TYPE.into_iter().collect::<Vec<&'static str>>()} selected={temp_type.clone()}/>
            <br/>

            <div class="columns">
            <button class="button is-primary" onclick={
                let has_spinner = has_spinner.clone();
             let coffee_ordered = coffee_ordered.clone();
                Callback::from(move |_|{
                      let coffee_ordered = coffee_ordered.clone();
                let has_spinner = has_spinner.clone();
                let (c, m, t) = ( *coffee_type.clone(), *milk_type.clone(),*temp_type.clone() );
                wasm_bindgen_futures::spawn_local(async move {
                    has_spinner.set(true);

                    if let Ok(data) =  Client::order(OrderPayload {coffee: c,milk: m ,temp: t}).await {
                            coffee_ordered.set(Some(data));


                    }
                     has_spinner.set(false);


                        // coffee_ordered.set();
                });

            })}>{"Submit"}</button>
            if *has_spinner {


            }


            </div>
        <div class={format!("modal {}", coffee_ordered_text)}>
      <div class="modal-background"></div>
      <div class="modal-card">
        <header class="modal-card-head">
          <p class="modal-card-title">{"Successfully ordered!"}</p>
          // <button class="delete" aria-label="close"></button>
        </header>
        <section class="modal-card-body">
            <h1>{format!("Order No {}", coffee_ordered.unwrap_or_default())}</h1>
        </section>
        <footer class="modal-card-foot">
          <div class="buttons">
            <button class="button is-success" onclick={Callback::from(move |_|{
                coffee_ordered.set(None);
            })}>{"Ok"}</button>
          </div>
        </footer>
      </div>
    </div>


             </>
        }
}

#[derive(Properties, PartialEq)]
pub struct DropProps {
    pub label: String,
    pub options: Vec<&'static str>,
    pub selected: UseStateHandle<u8>,
}

#[function_component(Drop)]
pub fn dropdown(props: &DropProps) -> Html {
    html! {
        <div class="field">
        <label class="label">{&props.label}</label>
        <div class="control">
        <div class="select">
        <select>

        <option selected=true>{SEL}</option>
        {
            props.options.iter().map(|a| {
            html!{ <option> {*a}</option>}
        }).collect::<Html>()
        }
        </select>
        </div>
        </div>
        </div>
    }
}

#[function_component(DropButton)]
pub fn dropbutton(props: &DropProps) -> Html {
    html! {
        <div class="field">
        <label class="label">{&props.label}</label>

        <div class="buttons has-addons">
        {
            props.options.iter().enumerate().map(|(count, value)| {
            let setter = props.selected.clone();
            let selected = if count as u8 == *setter {
                "is-success"
            }else {
                ""
            };

            html!{  <button class={format!("button {selected}")} onclick={Callback::from(move |_|{
                setter.set(count as u8);
            })}>{*value}</button>}
        }).collect::<Html>()
        }
        </div>
        </div>
    }
}
