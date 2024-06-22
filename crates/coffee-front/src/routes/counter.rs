use std::time::Duration;
use yew::prelude::*;
use shared::OrderInfo;
use crate::api::Client;
use crate::routes::{COFFEE_TYPE, MILK_TYPE, TEMP_TYPE};
// use crate::delay::time;


#[function_component(Counter)]
pub fn counter() -> Html {
    let orders = use_state(|| Vec::new());

    let used = use_state(|| true);

    {
        let orders = orders.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if *used {
                used.set(false);
                loop {
                    let client = Client::fetch().await.unwrap();
                    orders.set(client);
                    wasm_timer::Delay::new(Duration::from_secs(5)).await.unwrap();
                }
            }
        });
    }



    html! {<>

             if orders.is_empty() {


            <h1 class="title">{"No pending orders!"}</h1>
<h2 class="subtitle">{":3"}</h2>
        }
        {



            // orders.clone().into_iter().map(|a|{
             <Vec<OrderInfo> as Clone>::clone(&orders.clone()).into_iter().map(|a|{

            html!{
                <div class="box">
                <div class="columns  is-mobile ">
                // class="column"
                <h1 class="title column">{format!("Order No: {}", a.order_no)}</h1>

                <button class="button is-small column is-danger is-narrow" onclick=

                {
                    let orders = orders.clone();
                    Callback::from(move |_|{

                      wasm_bindgen_futures::spawn_local(async move {

                    Client::complete(a.order_no).await.unwrap();
                      })
                })
                }

                >{"Remove"}</button>
                </div>
                <div class="tags are-medium">
                <span class="tag is-link">{COFFEE_TYPE[a.coffee_info.coffee as usize]}</span>
                <span class="tag is-primary">{MILK_TYPE[a.coffee_info.milk as usize]}</span>
                <span class="tag is-info">{TEMP_TYPE[a.coffee_info.temp as usize]}</span>
                </div>
                </div>
            }
        }).collect::<Vec<Html>>()
        }

    </>}
}