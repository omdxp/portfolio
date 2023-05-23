use leptos::*;
use leptos_dom::console_log;

#[component]
pub fn Button(cx: Scope, #[prop(into)] title: String, on_press: fn() -> ()) -> impl IntoView {
    view! { cx,
        <button class="button-49" on:click=move |_| on_press()>{title}</button>
    }
}
