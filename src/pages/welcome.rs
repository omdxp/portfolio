use crate::components::Button;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn WelcomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Portfolio"/>
        <h1>"Welcome"</h1>
        <Button />
    }
}
