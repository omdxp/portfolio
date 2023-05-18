use crate::pages::HomePage;
use crate::pages::ProfilePage;
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
            <main>
                <Routes>
                    <Route path="/" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/profile" view=|cx| view! {cx, <ProfilePage/>} />
                </Routes>
            </main>
        </Router>
    }
}
