use std::rc::Rc;
use std::time::Duration;

use gloo_net::http::Request;
use yew::platform::time::sleep;
use yew::prelude::*;
use yew::suspense::{Suspension};
// use crate::components::country_list::_CountryListProps::countries;
use crate::components::{spinner::Spinner, country_details::CountryDetails};
use crate::models::country;
#[derive(PartialEq, Properties)]
pub struct DetailsProps {
    pub id: String
}




#[function_component(Details)]
pub fn details(props: &DetailsProps) -> Html {
    let DetailsProps { id } = props;
    let country_list = use_state(|| None);
    let country_list_clone = country_list.clone();
    let id = id.clone();
    Suspension::from_future(async move { 
        let country_model_list = Request::get(format!("https://restcountries.com/v3.1/alpha/{id}").as_str())
            .send()
            .await
            .unwrap()
            .json::<Vec<country::CountryDetailsModel>>()
            .await
            .unwrap();

        country_list.set(Some(Rc::new(country_model_list[0].clone())));
    });
    let fallback = html! { <Spinner loading_text={ "Loading country...".to_string() } /> };
    html! {
        <main class="main-root">
            {
                match country_list_clone.as_ref() {
                    Some(data) => html! { <CountryDetails country={Rc::clone(data)}/> },
                    None => fallback
                }
            }
        </main>
    }
}
