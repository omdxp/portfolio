use leptos::*;
use leptos_dom::console_log;

#[component]
pub fn Button(cx: Scope) -> impl IntoView {
    view! { cx,
        <button on:click=|_| console_log("hi")>"Push"</button>
    }
}
