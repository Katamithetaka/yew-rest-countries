use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct SpinnerProps {
    pub loading_text: String
}

#[function_component(Spinner)]
pub fn spinner(props: &SpinnerProps) -> Html {
    let SpinnerProps { loading_text } = props;
    html! {
        <div class="full-container">
            <div class="lds-ellipsis"><div></div><div></div><div></div><div></div></div>
            <p> { loading_text } </p>
        </div>
    }
}