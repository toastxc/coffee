use shared::{OrderInfo, OrderPayload};
use yew::prelude::*;

use crate::api::Client;
use shared::{COFFEE_TYPE, MILK_TYPE, SUGAR_TYPE, TEMP_TYPE};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
const SEL: &str = "Select dropdown";

#[function_component(Order)]
pub fn order() -> Html {
    let ui_type: UseStateHandle<usize> = use_state(|| 0);
    let ui = vec!["Standard", "Custom"];

    let coffee_type_custom: UseStateHandle<String> = use_state(String::new);

    let customer_name: UseStateHandle<String> = use_state(String::new);

    let coffee_type: UseStateHandle<usize> = use_state(|| 0);
    let milk_type: UseStateHandle<usize> = use_state(|| 0);
    let temp_type: UseStateHandle<usize> = use_state(|| 0);
    let sugar_type: UseStateHandle<usize> = use_state(|| 0);

    let coffee_ordered: UseStateHandle<Option<u8>> = use_state(|| None);

    let coffee_ordered_text = match coffee_ordered.is_some() {
        true => "is-active",
        false => "",
    };

    let payload = match *ui_type {
        0 => OrderPayload::Standard {
            coffee: *coffee_type,
            milk: *milk_type,
            temp: *temp_type,
            sugar: *sugar_type,
        },

        1 => OrderPayload::Beth(coffee_type_custom.to_string()),
        _ => {
            unreachable!()
        }
    };

    let payload = OrderInfo {
        order_name: customer_name.to_string(),
        coffee_info: payload,
        date: None,
        id: None,
    };
    let html = html! {<div class={"container"}>




             <br/>

         <form onsubmit={
                 let coffee_ordered = coffee_ordered.clone();

                 Callback::from(move |event: SubmitEvent|{

                 event.prevent_default();


                          let payload = payload.clone();
                     let coffee_ordered = coffee_ordered.clone();
                     wasm_bindgen_futures::spawn_local(async move {

                         if let Ok(data) =  Client::order(payload).await {
                             coffee_ordered.set(Some(data));
                         }
                     });

                 })}>
             <DropButton label="Interface" options={ui} selected={ui_type.clone()}/>


           <TextInput label={"Name"} placeholder={None} required={true} value={customer_name.clone()}/>



         if *ui_type == 0 {
             <DropButton label="Coffee type" options={COFFEE_TYPE.into_iter().collect::<Vec<&'static str>>()}selected={coffee_type.clone()}/>
             <DropButton label="Milk type" options={MILK_TYPE.into_iter().collect::<Vec<&'static str>>()} selected={milk_type.clone()}/>
             <DropButton label="Temperature" options={TEMP_TYPE.into_iter().collect::<Vec<&'static str>>()} selected={temp_type.clone()}/>
                <DropButton label="Sugar" options={SUGAR_TYPE.into_iter().collect::<Vec<&'static str>>()} selected={sugar_type.clone()}/>


         }else {


           <TextInput label={"Order"} placeholder={Some("Coffee with extra coffee")} required={true} value={coffee_type_custom}/>


         }
                 <br/>





             <button type={"submit"} class="button is-primary" id={"form"}

         >{"Submit"}</button>


             </form>

             <div class={format!("modal {}", coffee_ordered_text)}>
           <div class="modal-background"></div>
           <div class="modal-card">
             <header class="modal-card-head">
               <p class="modal-card-title">{"Successfully ordered!"}</p>
             </header>
             <section class="modal-card-body">
                 <h1>{format!("Order No {}", &coffee_ordered.unwrap_or_default())}</h1>
             </section>
             <footer class="modal-card-foot">
               <div class="buttons">
                 <button class="button is-success" onclick={Callback::from(move |_|{
                     coffee_ordered.set(None);

                     customer_name.set(Default::default());

                 })}>{"Ok"}</button>
               </div>
             </footer>
           </div>
         </div>

    //     </form>

                  </div>
             };

    html
}

#[derive(Properties, PartialEq)]
pub struct DropProps {
    pub label: String,
    pub options: Vec<&'static str>,
    pub selected: UseStateHandle<usize>,
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
            let selected = if count  == *setter {
                "is-success"
            }else {
                ""
            };

            html!{  <button type={"button"} class={format!("button {selected}")} onclick={Callback::from(move |_|{
                setter.set(count);
            })}>{*value}</button>}
        }).collect::<Html>()
        }
        </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub label: &'static str,
    pub placeholder: Option<&'static str>,
    pub required: bool,
    pub value: UseStateHandle<String>,
}

#[function_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
    let value = props.value.clone();
    let cb = Callback::from(move |event: Event| {
        value.set(
            event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value(),
        );
    });

    html! {
        <div class="field">
        <label class="label">{props.label}</label>
        <div class="control">
        <input
        class="input"
        type="text"
        placeholder={props.placeholder.unwrap_or_default()}
        required={props.required}
        onchange={cb}
        />
      </div>
    </div>
    }
}
