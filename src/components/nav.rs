use leptos::*;
use leptos_router::*;

#[component]
pub fn Nav(cx: Scope) -> impl IntoView {
    view! { cx,
        <nav>
            <ul>
                <li><A href="about-me">"About Me"</A></li>
                <li><A href="skills">"Skills"</A></li>
                <li><A href="projects">"Projects"</A></li>
                <li><A href="links">"Links"</A></li>
                <li><A href="console">"Console"</A></li>
            </ul>
        </nav>
    }
}
