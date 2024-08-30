use crate::api::Client;
use shared::OrderInfo;
use shared::{COFFEE_TYPE, MILK_TYPE, TEMP_TYPE};
use std::time::Duration;
use yew::prelude::*;

#[function_component(Counter)]
pub fn counter() -> Html {
    let orders: UseStateHandle<Vec<OrderInfo>> = use_state(Vec::new);

    // UseStateHandle<Vec<OrderInfo>>
    let used = use_state(|| true);

    {
        let orders = orders.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if *used {
                used.set(false);
                loop {
                    let client = Client::fetch().await.unwrap();
                    orders.set(client);
                    fluvio_wasm_timer::Delay::new(Duration::from_secs(5))
                        .await
                        .unwrap();
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
                        <h1 class="title column">{format!("Order Name: {}", a.order_name)}</h1>

                        <button class="button is-small column is-danger is-narrow" onclick=

                        {
                            Callback::from(move |_|{


                              wasm_bindgen_futures::spawn_local(async move {

                            Client::complete(a.id.unwrap()).await.unwrap();
                              })
                        })
                        }

                        >{"Remove"}</button>
                        </div>


                        {
                        match a.coffee_info {
                            shared::OrderPayload::Standard { coffee, milk, temp, sugar } => html!{

                                    <div class="tags are-medium">
                                    <span class="tag is-link">{COFFEE_TYPE[coffee]}</span>
                                    <span class="tag is-primary">{MILK_TYPE[milk]}</span>
                                    <span class="tag is-info">{TEMP_TYPE[temp]}</span>
                                    </div>

                            },
                            shared::OrderPayload::Beth(str) => html!{

                                <p>{str}</p>
                            }
                        }}

      //                  <div class="tags are-medium">
    //                    <span class="tag is-link">{COFFEE_TYPE[a.coffee_info.coffee as usize]}</span>
        //                <span class="tag is-primary">{MILK_TYPE[a.coffee_info.milk as usize]}</span>
          //              <span class="tag is-info">{TEMP_TYPE[a.coffee_info.temp as usize]}</span>
            //            </div>
                        </div>
                    }
                }).collect::<Vec<Html>>()
                }

            </>}
}
