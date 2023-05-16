use crate::components::Button;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn ProfilePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Profile"/>
        <h1>"Profile"</h1>
        <Button/>
    }
}
