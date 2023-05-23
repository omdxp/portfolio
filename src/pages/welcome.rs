use leptos::*;
use leptos_meta::*;

#[component]
pub fn WelcomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Omar"/>
        <h1>"Omar Belghaouti"</h1>
        <h2>"Software Engineer"</h2>
        <p>"Enthusiastic Developer, New Tech Lover, Non-Stop Learner..."</p>
        <a class="button-49" href="mailto:omarbelghaouti@gmail.com">"Get in touch"</a>
    }
}
