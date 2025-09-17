#![allow(dead_code)]

use druid::{
    AppLauncher, Data, Env, Lens, PlatformError, Widget, WindowDesc, theme, widget::Label,
};

use crate::config;

/// IDE state
#[derive(Clone, Data, Lens)]
pub struct AppState {
    // pub current_file: Option<String>,
    // pub editor_content: String,
    // pub is_modified: bool,
}

/// init & run
pub fn run(_filename: &String) -> Result<(), druid::PlatformError> {
    let initial_state = AppState {
        // current_file: None,
        // editor_content: String::from(
        //     "// Welcome to μIDE\nfn main() {\n    println!(\"Hello, Rust!\");\n}",
    };
    // if let Some(file) = std::fs::File::open(filename).ok() {
    //     let mmap = unsafe { memmap2::Mmap::map(&file) }.unwrap_or_else(|_| {
    //         memmap2::Mmap::map(&std::fs::File::open("/dev/null").unwrap()).unwrap()
    //     });
    //     String::from_utf8_lossy(&mmap).into_owned()
    // } else {            }
    // is_modified: false,

    let main_window = WindowDesc::new(build_ui())
        .window_size(config::GUI::WIN::SIZE)
        .title(config::GUI::WIN::TITLE);

    AppLauncher::with_window(main_window)
        .configure_env(theme)
        .launch(initial_state)
}

fn theme(env: &mut Env, _state: &AppState) {
    env.set(theme::BACKGROUND_DARK, config::GUI::Color::Background);
    env.set(theme::BACKGROUND_LIGHT, config::GUI::Color::Status);
    env.set(theme::TEXT_COLOR, config::GUI::Color::Text);
    env.set(theme::TEXT_SIZE_NORMAL, config::GUI::Font::SIZE);
    env.set(theme::TEXT_SIZE_LARGE, config::GUI::Font::SIZE);
    // env.set(druid::theme::FONT_SIZE, config::gui::FONT_SIZE);
    // env.set(druid::theme::FONT_FAMILY, config::gui::FONT_FAMILY);
}

// fn build_ui() -> impl Widget<AppState> {}
fn build_ui() -> impl Widget<AppState> {
    Label::new("Hello world")
}
