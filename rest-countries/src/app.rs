use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::*;
use crate::components::*;

#[function_component(App)]
pub fn app() -> Html {
    let theme = use_state(|| "light".to_string());
    html! {
        <BrowserRouter>
            <div class={(*theme).clone()} id="content-root">
                <header::Header theme_state={theme}/>
                <Switch<AppRoute> render={switch} />
                <footer::Footer />
            </div>
        </BrowserRouter>
    }
}
