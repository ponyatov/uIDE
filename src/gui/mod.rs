#![allow(dead_code)]

use druid::{widget::Label, AppLauncher, Data, Env, Lens, PlatformError, Widget, WindowDesc};

use crate::{config, gui::app_state_derived_lenses::is_modified};

/// IDE state
#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub current_file: Option<String>,
    pub editor_content: String,
    pub is_modified: bool,
}

/// init & run
pub fn run(_filename: &String) -> Result<(), druid::PlatformError> {
    let _initial_state = AppState {
        current_file: None,
        editor_content: String::from(
            "// Welcome to μIDE\nfn main() {\n    println!(\"Hello, Rust!\");\n}",
        ),
        // if let Some(file) = std::fs::File::open(filename).ok() {
        //     let mmap = unsafe { memmap2::Mmap::map(&file) }.unwrap_or_else(|_| {
        //         memmap2::Mmap::map(&std::fs::File::open("/dev/null").unwrap()).unwrap()
        //     });
        //     String::from_utf8_lossy(&mmap).into_owned()
        // } else {            }
        is_modified: false,
    };

    let main_window = WindowDesc::new(build_ui())
        .title(config::GUI::WIN::TITLE)
        .window_size(config::GUI::WIN::SIZE);

        let initial_data = ();

    AppLauncher::with_window(main_window)
        // .configure_env(theme)
        .launch(initial_data)
}

fn theme(_env: &mut Env) {
    // env.set(druid::theme::BACKGROUND_COLOR, config::gui::BG_COLOR);
    // env.set(druid::theme::TEXT_COLOR, config::gui::TEXT_COLOR);
    // env.set(druid::theme::FONT_SIZE, config::gui::FONT_SIZE);
    // env.set(druid::theme::FONT_FAMILY, config::gui::FONT_FAMILY);
}

// fn build_ui() -> impl Widget<AppState> {}
fn build_ui() -> impl Widget<()> {
    Label::new("Hello world")
}
