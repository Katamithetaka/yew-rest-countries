use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct FooterProps {}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    let FooterProps {} = props;
    html! {
        <div></div>
    }
}