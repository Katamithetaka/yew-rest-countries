
use std::rc::Rc;

use num_format::{CustomFormat, ToFormattedStr, ToFormattedString};
use yew::prelude::*;

use crate::models::country;
#[derive(PartialEq, Properties)]
pub struct CountryDetailsProps {
    pub country: Rc<country::CountryDetailsModel>
}

#[function_component(CountryDetails)]
pub fn country_details(props: &CountryDetailsProps) -> Html {
    let CountryDetailsProps { country } = props;
    html! {
        <div></div>
    }
}