
use std::rc::Rc;

use num_format::{CustomFormat, ToFormattedStr, ToFormattedString};
use yew::prelude::*;

use crate::models::country;
#[derive(PartialEq, Properties)]
pub struct CountryCardProps {
    pub country: Rc<country::CountryModel>
}

#[function_component(CountryCard)]
pub fn country_card(props: &CountryCardProps) -> Html {
    let CountryCardProps { country } = props;
    let flag = country.flags.png.clone();
    let name = country.name.common.clone();
    let population = country.population.clone();
    let region = country.region.clone();
    let capitals = country.capital.clone();
    let capital_fallback = html! {<p><b class="country-card-list-title">{"Capital"}</b>{": None"}</p>};
    let custom_format = CustomFormat::builder().grouping(num_format::Grouping::Standard).separator(".").build().unwrap();
    let cca2 = format!("./details/{}", country.cca2);
    html! {
            <a class="country-card-container" href={cca2}>
                <img class="country-card-flag-img" src={flag}/>
                <div class="country-card-text-container">
                    <h2 class="country-card-name">{name}</h2>
                    <p class="country-card-list">
                        <p><b class="country-card-list-title">{"Population: "}</b>{population.to_formatted_string(&custom_format)}</p>
                        <p><b class="country-card-list-title">{"Region: "}</b> {region}</p>
                        {
                            match capitals {
                                Some(capitals) => {
                                    if capitals.len() == 0 {
                                        capital_fallback
                                    }
                                    else {
                                        html! {
                                            <p><b class="country-card-list-title">{"Capital: "}</b>{capitals[0].clone()}</p>
                                        }
                                    }
                                },
                                None => capital_fallback
                            }
                        }
                    </p>
                </div>
            </a>
    }
}