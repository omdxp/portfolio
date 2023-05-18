use crate::components::Button;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn ProjectsPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Projects"/>
        <h1>"Projects"</h1>
        <Button/>
    }
}
