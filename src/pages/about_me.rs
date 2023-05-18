use crate::components::Button;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn AboutMePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="About Me"/>
        <h1>"About Me"</h1>
        <Button/>
    }
}
