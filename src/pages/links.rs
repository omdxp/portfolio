use crate::components::Button;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn LinksPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Links"/>
        <h1>"Links"</h1>
        <Button/>
    }
}
