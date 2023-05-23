use leptos::*;
use leptos_router::*;

#[component]
pub fn Nav(cx: Scope) -> impl IntoView {
    view! { cx,
        <nav class="container">
            <ul class="flexnav" data-breakpoint="800">
                <li><A href="#">"OMDXP"</A></li>
                <li><A href="skills">"Skills"</A></li>
                <li><A href="projects">"Projects"</A></li>
                <li><A href="links">"Links"</A></li>
                <li><A href="console">"Console"</A></li>
            </ul>
        </nav>
    }
}
