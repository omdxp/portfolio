use crate::components::Nav;
use crate::pages::AboutMePage;
use crate::pages::ConsolePage;
use crate::pages::LinksPage;
use crate::pages::ProjectsPage;
use crate::pages::SkillsPage;
use crate::pages::WelcomePage;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Title text="Welcome to Leptos"/>
        <Router>
            <Nav />
            <main>
                <Routes>
                    <Route path="/" view=|cx| view! { cx, <WelcomePage/> }/>
                    <Route path="/about-me" view=|cx| view! {cx, <AboutMePage/>} />
                    <Route path="/skills" view=|cx| view! {cx, <SkillsPage/>} />
                    <Route path="/projects" view=|cx| view! {cx, <ProjectsPage/>} />
                    <Route path="/links" view=|cx| view! {cx, <LinksPage/>} />
                    <Route path="/console" view=|cx| view! {cx, <ConsolePage/>} />
                </Routes>
            </main>
        </Router>
    }
}
