mod app;

pub fn main() {
    use app::App;
    leptos::mount::mount_to_body(|| leptos::view! { <App/> });
}
