
use std::rc::Rc;

use gloo_net::http::Request;
use yew::{prelude::*, suspense::use_future};

use super::country_card::CountryCard;
use crate::models::country;
#[derive(PartialEq, Properties)]
pub struct CountryListProps {
}

#[function_component(CountryList)]
pub fn country_list(props: &CountryListProps) -> HtmlResult {
    let CountryListProps {  } = props;
    let countries = use_future(|| async move { 
        Request::get("https://restcountries.com/v3.1/all")
            .send()
            .await?
            .json::<Vec<country::CountryModel>>()
            .await
    });
    let content = |data: &Vec<country::CountryModel>| {
        let mut countries = data.clone();
        countries.sort_by(|a, b| b.population.cmp(&a.population));
        html! {
            {
                countries
                    .iter()
                    .map(|f| { Rc::new(f.clone()) })
                    .map(|f| html! { <CountryCard country={Rc::clone(&f)}/> })
                    .collect::<Html>()
            }
        }
    };
    Ok(html! {
        <div class="country-list">
        {
            match countries?.as_ref() {
                Ok(data) => content(data),
                Err(err) => {
                    log::error!("{}", err); 
                    html! { <p>{"Couldn't load data."}</p> }
                }
            }
        }
        </div>
    })
}