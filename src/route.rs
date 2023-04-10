use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Home;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
           <Home />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

// #[derive(Clone, Debug, Store, PartialEq, Deserialize, Serialize, Default)]
// #[store(storage = "local")]
// pub struct DarkModeConfig {
// pub is_dark: bool,
// }

#[function_component(Main)]
pub fn app() -> Html {
    // TODO fix dark mode
    // let window = web_sys::window().unwrap();
    // let document = window.document().unwrap();
    // let element = document.document_element().unwrap();
    // let class_list = element.class_list();
    // let (dark_mode_config, _) = yewdux::prelude::use_store::<DarkModeConfig>();
    // if dark_mode_config.is_dark {
    // class_list.add_1("dark").unwrap();
    // } else {
    // class_list.remove_1("dark").unwrap();
    // }

    // TODO fix language
    // let (language_config, _) = yewdux::prelude::use_store::<LanguagesConfigState>();
    // rust_i18n::set_locale(&language_config.config);

    html! {
        <div
            class="bg-white dark:bg-gray-800 text-black dark:text-white min-h-screen"
        >
            <BrowserRouter>
                    <Switch<Route> render={switch}/> // <- must be child of <BrowserRouter>
            </BrowserRouter>
        </div>
    }
}
