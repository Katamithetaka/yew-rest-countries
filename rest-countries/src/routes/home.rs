use std::rc::Rc;

use gloo_net::http::Request;

use log::warn;
use yew::prelude::*;
use yew::suspense::{Suspension, use_future};
use yew::virtual_dom::VNode;
use crate::components::{spinner::Spinner, country_list::CountryList};
use crate::models::country::{self, CountryModel};
#[derive(PartialEq, Properties)]
pub struct HomeProps {}

use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlSelectElement, HtmlInputElement};



#[function_component(Home)]
pub fn home(props: &HomeProps) -> HtmlResult {
    let HomeProps {} = props;

    let region_filter = use_state(|| 0);
    let search_filter = use_state(|| String::new());

    // let content = |data: &Vec<CountryModel>| {
        
    //     let regions = data.iter().map(|value| value.region.clone()).collect::<std::collections::HashSet<_>>();
    //     let mut regions = regions.iter().map(|c| c.clone()).collect::<Vec<_>>();
    //     regions.sort();
    //     let data = data;
        
    //     let search_filter_clone = search_filter.clone();
    //     let on_search_changed = Callback::from(move |e: Event| {
    //         let target: Option<EventTarget> = e.target();
    //             // Events can bubble so this listener might catch events from child
    //             // elements which are not of type HtmlInputElement
    //             let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
    
    //             if let Some(input) = input {
    //                 match input.text_content() {
    //                     Some(data) => search_filter_clone.set(data),
    //                     None => search_filter_clone.set(String::new())
    //                 };
    //             }
    //     });

    //     let region_filter_clone =  region_filter.clone();
    //     let on_selection_changed = Callback::from(move |e: Event| {
    //         let target: Option<EventTarget> = e.target();
    //             // Events can bubble so this listener might catch events from child
    //             // elements which are not of type HtmlInputElement
    //             let input = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
    
    //             if let Some(input) = input {
    //                 warn!("Here!");
    //                 region_filter_clone.set(input.selected_index())
    //             }
    //     });
    //     html! { 
    //         <>
    //             <div class="search-section">
    //                 <input type="text" name="search_bar" onchange={on_search_changed} />
    //                 <select name="continent_choice" onchange={on_selection_changed}>
    //                     <option selected=true> {"All"} </option>
    //                     {
    //                         regions.iter().map(|f| {
    //                             html! {
    //                                 <option> {f} </option>
    //                             }
    //                         }).collect::<Html>() 
    //                     }
    //                 </select>
    //             </div>
    //             <CountryList/>
    //         </>
    //     }
    // };
    let home_fallback = html! { <Spinner loading_text={ "Loading countries...".to_string() } /> };

    Ok(html! {
        <main class="main-root">
            <Suspense fallback = {home_fallback}>
                <CountryList/>
            </Suspense>
        </main>
    })
}
