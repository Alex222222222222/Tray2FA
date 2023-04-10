use route::Main;

mod app;
mod route;
mod invoke;

fn main() {
    yew::Renderer::<Main>::new().render();
}
