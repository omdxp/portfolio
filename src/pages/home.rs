use crate::components::Button;
use leptos::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <Button />
    }
}
