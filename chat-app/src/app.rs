use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <img class="logo" src="https://rustacean.net/assets/rustacean-flat-happy.png" alt="Yew logo" />
            <h1>{ "Chat" }</h1>
            <span class="subtitle">{ "from Solid Sedan " }<i class="heart" /></span>
        </main>
    }
}
