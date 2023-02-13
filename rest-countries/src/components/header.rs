use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct HeaderProps {
    pub theme_state: UseStateHandle<String>
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let HeaderProps {theme_state} = props;

    let dark_mode = use_state(|| false);
    let onclick = {
            let dark_mode = dark_mode.clone();
            let theme_state = theme_state.clone();
            Callback::from(move |_| {
                let light = !*dark_mode;
                dark_mode.set(!*dark_mode);
                theme_state.set(if light {
                    "dark".to_string()
                } else { "light".to_string() });
            })
        };
    html! {
        <header class="header-band">
            <h1 class="header-logo">{"Where in the world?"}</h1>
            <button class="theme-toggle" {onclick}>{"🌙 Dark mode"}</button>

        </header>
    }
}