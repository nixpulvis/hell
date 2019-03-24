#[macro_use]
extern crate conrod;
extern crate piston_window;
extern crate evolution;
extern crate evolution_server;

use conrod::{
    Graphics,
    Theme,
    Widget,
    Positionable,
    Canvas,
};
use piston_window::{
    G2d,
    EventLoop,
    Glyphs,
    PistonWindow,
    UpdateEvent,
    WindowSettings,
};

pub use dealer_widget::DealerWidget;
pub use debug_widget::DebugWidget;
pub use deck_widget::DeckWidget;
pub use board_widget::BoardWidget;
pub use game_widget::GameWidget;
pub use player_widget::PlayerWidget;
pub use species_widget::SpeciesWidget;

type Backend = (<G2d<'static> as Graphics>::Texture, Glyphs);
type Ui = conrod::Ui<Backend>;
type UiCell<'a> = conrod::UiCell<'a, Backend>;

pub fn run<W, F>(widget: F, title: &str)
    where W: Widget,
          F: Fn() -> W
{
    let window: PistonWindow = WindowSettings::new(title, [800, 600])
                                              .exit_on_esc(true)
                                              .build()
                                              .unwrap();
    let mut ui = {
        let font_path = "assets/fonts/FiraMono-Regular.ttf";
        let theme = Theme::default();
        let glyph_cache = Glyphs::new(&font_path, window.factory.borrow().clone());
        Ui::new(glyph_cache.unwrap(), theme)
    };

    for event in window.ups(60) {
        ui.handle_event(&event);
        event.update(|_| ui.set_widgets(|ref mut ui| {
            widget_ids! {
                CANVAS,
                TEXTBOX,
                JSON,
                RECT,
                GAME,
            };

            Canvas::new().set(CANVAS, ui);

            widget().top_left_with_margin_on(CANVAS, 10.0)
                    .set(GAME, ui);
        }));
        event.draw_2d(|c, g| ui.draw(c, g));
    }
}

mod dealer_widget;
mod debug_widget;
mod deck_widget;
mod board_widget;
mod game_widget;
mod player_widget;
mod species_widget;
