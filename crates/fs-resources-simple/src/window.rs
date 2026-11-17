//! The window: name, the real icon (embedded PNG, own design), the content.
//!
//! Pattern: Adapter: our blocks in, a screen of the fs-components shell out; no widget is painted by hand.

use std::sync::{Arc, Mutex};

use fs_components::blinc::layout::div::div;
use fs_components::blinc::layout::image::image;
use fs_components::blinc::layout::widgets::scroll::{Scroll, ScrollPhysics};
use fs_components_app::{AppError, AppWidget, Element, Menu, MenuEntry, Screen, Status, Surface};
use fs_components_mf2::Mf2Texts;
use fs_components_widgets::{Block, Inline, TextView, styles};
use fs_components_words::{Args, Words};

use crate::content::{coming, inline, start};
use crate::link::Link;

/// The icon of the program, embedded in the binary.
const ICON: &[u8] = include_bytes!("../assets/fs-resources.png");

fn icon_uri() -> String {
    use base64::Engine;
    format!(
        "data:image/png;base64,{}",
        base64::engine::general_purpose::STANDARD.encode(ICON)
    )
}

/// Width of the box of the title, in pixels (the text view spreads over what it gets; a narrow box keeps the title at the icon).
const TITLE_WIDTH: f32 = 340.0;

/// How far the title box is pulled to the icon (Blinc counts margins in units of 4 px; the text view keeps room on its own left).
const TITLE_PULL: f32 = 13.0;

/// Side of the icon in the window, in pixels.
const ICON_SIZE: f32 = 64.0;

/// One screen of the window.
struct Page {
    name: String,
    tagline: String,
    body: Vec<Block>,
    /// The start page carries the header with the icon and the goal; the page "what comes" is the list alone.
    header: bool,
}

impl Surface for Page {
    fn build(&self, width: f32, height: f32) -> Box<dyn Element> {
        // The icon of the program (see packaging/ICON.md), as a data address Blinc loads. `no_fade`: Blinc fades a
        // picture in, and one drawn at once (headless) would show half transparent.
        let icon = image(icon_uri()).size(ICON_SIZE, ICON_SIZE).no_fade();
        let title = div().w(TITLE_WIDTH).ml(-TITLE_PULL).child(
            TextView::new(vec![Block::Heading {
                level: 1,
                text: Inline::plain(self.name.as_str()),
            }])
            .build(),
        );
        let mut column = div()
            .flex_col()
            .gap(4.0)
            .p(24.0)
            .w((width - 40.0).max(200.0));
        if self.header {
            column = column
                .child(
                    div()
                        .flex_row()
                        .items_center()
                        .gap(16.0)
                        .child(icon)
                        .child(title),
                )
                .child(TextView::new(vec![Block::Paragraph(inline(&self.tagline))]).build());
        }
        let column = column.child(TextView::new(self.body.clone()).tasks_locked(0).build());
        // The page scrolls when it is longer than the window, so nothing is cut off at the bottom.
        let physics = Arc::new(Mutex::new(ScrollPhysics::default()));
        Box::new(
            div().w(width).h(height).child(
                Scroll::with_physics(physics)
                    .w(width)
                    .h(height)
                    .content(column),
            ),
        )
    }

    fn styles(&self) -> String {
        styles()
    }
}

/// The window for `link`, with the menus whose entries are all locked ("coming soon").
///
/// # Errors
/// The shell could not be made (a name that is empty, for example).
pub fn app(texts: &Mf2Texts, link: &Link) -> Result<AppWidget, AppError> {
    let say = |id: &str| texts.say(&id.into(), &Args::new()).text().to_owned();
    let soon = say("soon");
    let page = |header: bool, body: Vec<Block>| {
        Arc::new(Page {
            name: say("app-name"),
            tagline: say("app-tagline"),
            body,
            header,
        })
    };
    let widget = AppWidget::new(
        say("app-name"),
        vec![
            Screen::new("start", say("screen-start"), page(true, start(texts, link))),
            Screen::new("comes", say("screen-comes"), page(false, coming(texts))),
        ],
    )?;
    let locked = |id: &str| {
        MenuEntry::new(id, format!("{} ({soon})", say(&format!("menu-{id}")))).disabled()
    };
    widget.set_menus(
        vec![Menu::new(
            say("menu-program"),
            vec![locked("new"), locked("reserve"), locked("search")],
        )],
        |_| {},
    )?;
    widget.set_status(Some(Status::Info(say("status-ready"))));
    Ok(widget)
}
