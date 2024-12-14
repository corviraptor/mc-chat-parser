use corviraptor_dot_dev::components::button::LinkButton;
use corviraptor_dot_dev::components::{IconType, Section};
use yew::prelude::*;

use corviraptor_dot_dev::components::pages;
use corviraptor_dot_dev::theme::Theme;

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(Theme::new);

    let title = "Minecraft Chat Log Parser".to_string();

    pages::build_single_page_site(
        title,
        None,
        html! {
            <Section content={
                html!{
                    <LinkButton name={ "github" } url={ "https://github.com/corviraptor" } icon={ IconType::NerdFont("nf-fa-github".to_string()) } />
                }
            }/>
        },
        state,
    )
}
