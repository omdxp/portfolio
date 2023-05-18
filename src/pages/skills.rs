use crate::components::Button;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn SkillsPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Skills"/>
        <h1>"Skills"</h1>
        <Button/>
    }
}
