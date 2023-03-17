use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[function_component]
fn App() -> Html {
    let online_ico = "🟢";
    let offline_ico = "🔴";

    let net_ip = use_state_eq(|| "0.0.0.0".to_string());

    let act_navclass = "nav-link text-white active";
    let deact_navclass = "nav-link text-white";

    let net_status = use_state(|| "🔴");
    let navstate_tab1 = use_state(|| act_navclass);
    let navstate_tab2 = use_state(|| deact_navclass);
    let navstate_tab3 = use_state(|| deact_navclass);
    let navstate_tab4 = use_state(|| deact_navclass);

    let offline = {
        let stat = net_status.clone();
        move || {
            stat.set(offline_ico);
        }
    };

    let online = {
        let stat = net_status.clone();
        move || {
            stat.set(online_ico);
        }
    };

    //Make nav tab 1 active
    let navtab1_update = {
        let navstate_tab1 = navstate_tab1.clone();
        let navstate_tab2 = navstate_tab2.clone();
        let navstate_tab3 = navstate_tab3.clone();
        let navstate_tab4 = navstate_tab4.clone();
        move |_| {
            navstate_tab1.set(act_navclass);
            navstate_tab2.set(deact_navclass);
            navstate_tab3.set(deact_navclass);
            navstate_tab4.set(deact_navclass);
        }
    };
    //Make nav tab 2 active
    let navtab2_update = {
        let navstate_tab1 = navstate_tab1.clone();
        let navstate_tab2 = navstate_tab2.clone();
        let navstate_tab3 = navstate_tab3.clone();
        let navstate_tab4 = navstate_tab4.clone();
        move |_| {
            navstate_tab2.set(act_navclass);
            navstate_tab1.set(deact_navclass);
            navstate_tab3.set(deact_navclass);
            navstate_tab4.set(deact_navclass);
        }
    };
    //Make nav tab 3 active
    let navtab3_update = {
        let navstate_tab1 = navstate_tab1.clone();
        let navstate_tab2 = navstate_tab2.clone();
        let navstate_tab3 = navstate_tab3.clone();
        let navstate_tab4 = navstate_tab4.clone();
        move |_| {
            navstate_tab3.set(act_navclass);
            navstate_tab2.set(deact_navclass);
            navstate_tab1.set(deact_navclass);
            navstate_tab4.set(deact_navclass);
        }
    };
    //Make nav tab 4 active
    let navtab4_update = {
        let navstate_tab1 = navstate_tab1.clone();
        let navstate_tab2 = navstate_tab2.clone();
        let navstate_tab3 = navstate_tab3.clone();
        let navstate_tab4 = navstate_tab4.clone();
        move |_| {
            navstate_tab4.set(act_navclass);
            navstate_tab2.set(deact_navclass);
            navstate_tab3.set(deact_navclass);
            navstate_tab1.set(deact_navclass);
        }
    };

    {
        let net_ip = net_ip.clone();
        use_effect(move || {
                update_ip(net_ip);
        });
    }

    let message = (*net_ip).clone();


    html! {
        <main style="height: 100%;">
            <div class="d-flex flex-column flex-shrink-0 p-3 text-white bg-dark" style="width: 23%; height: 100%;">
                <a class="d-flex align-items-center mb-3 mb-md-0 me-md-auto text-white text-decoration-none">
                    <img src="icon.png" class="bi me-2" width="30" height="30"/>
                    <span class="fs-4">{"NetSecure"}</span>
                </a>
                <hr/>
                <ul class="nav nav-pills flex-column mb-auto">
                    <li class="nav-item">
                        <a class={*navstate_tab1} onclick={navtab1_update} href="#">{"Dashboard"}</a>
                    </li>
                    <li class="nav-item">
                        <a class={*navstate_tab2} onclick={navtab2_update} href="#">{"Network"}</a>
                    </li>
                    <li class="nav-item">
                        <a class={*navstate_tab3} onclick={navtab3_update} href="#">{"Security"}</a>
                    </li>
                    <li class="nav-item">
                        <a class={*navstate_tab4} onclick={navtab4_update} href="#">{"Settings"}</a>
                    </li>
                </ul>
                <hr/>
                    <a class="d-flex flex-column align-items-start text-white text-decoration-none h-auto">
                        <p class="text-white mb-0">{*net_status}{message}</p>
                    </a>
                </div>
                <div id="content" style="width:100%,height:100%">

                </div>
        </main>
     }

}

fn main() {
    yew::Renderer::<App>::new().render();
}

fn update_ip(net_ip: UseStateHandle<String>) {
    {
        let net_ip = net_ip.clone();
        spawn_local(async move {
            // This will call our glue code all the way through to the tauri
            // back-end command and return the `Result<String, String>` as
            // `Result<JsValue, JsValue>`.
            println!("calling function");
            match getnetwork().await {
                Ok(ip) => {
                    net_ip.set(ip.as_string().unwrap().to_owned());
                }
                Err(e) => {
                    let window = window().unwrap();
                    window
                        .alert_with_message(&format!("Error: {:?}", e))
                        .unwrap();
                }
            }
        });
    }
}

#[wasm_bindgen(module = "/public/bridge.js")]
extern "C" {
    #[wasm_bindgen(js_name = invoke_getnetwork, catch)]
    pub async fn getnetwork() -> Result<JsValue, JsValue>;
}