
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::spinner::Spinner;

use self::home::Home;
use self::details::Details;
mod home;
mod details;
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/index")]
    Index,
    #[at("/home")]
    HomeRedirect,
    #[at("/details/:id")]
    Details { id: String }
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Home | AppRoute::HomeRedirect | AppRoute::Index => html! {
            <Home/>
        },
        AppRoute::Details { id } => html! { <Details id={id}/> }
    }
}