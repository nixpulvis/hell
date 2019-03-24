use conrod::{
    Positionable,
    WidgetMatrix,
    Widget,
    WidgetKind,
    CommonBuilder,
    Backend,
    Sizeable,
    IndexSlot,
    UpdateArgs,
};
use evolution::Game;
use player_widget::PlayerWidget;
use board_widget::BoardWidget;

pub const KIND: WidgetKind = "Game";

#[derive(Debug)]
pub struct GameWidget<'a> {
    game: &'a Game,
    common: CommonBuilder,
}

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    players_idx: IndexSlot,
    board_idx: IndexSlot,
}

impl<'a> GameWidget<'a> {
    pub fn new(game: &'a Game) -> Self {
        GameWidget {
            game: game,
            common: CommonBuilder::new(),
        }
    }
}

impl<'a> Widget for GameWidget<'a> {
    type State = State;
    type Style = ();

    fn common(&self) -> &CommonBuilder {
        &self.common
    }

    fn common_mut(&mut self) -> &mut CommonBuilder {
        &mut self.common
    }

    fn unique_kind(&self) -> WidgetKind {
        KIND
    }

    fn init_state(&self) -> Self::State {
        State {
            board_idx: IndexSlot::new(),
            players_idx: IndexSlot::new(),
        }
    }

    fn style(&self) -> Self::Style {
        ()
    }

    fn update<B: Backend>(self, args: UpdateArgs<Self, B>) {
        let UpdateArgs { idx, state, mut ui, .. } = args;

        let players_idx = state.view().players_idx.get(&mut ui);
        WidgetMatrix::new(self.game.players().len(), 1)
               .each_widget(|n, _, _| PlayerWidget::new(&self.game.players()[n]))
               .top_left_of(idx)
               .graphics_for(idx)
               .set(players_idx, &mut ui);

        let board_idx = state.view().board_idx.get(&mut ui);
        BoardWidget::new(self.game.board())
                    .y_relative_to(players_idx, -0.0)
                    .wh([800.0, 200.0])
                    .graphics_for(idx)
                    .set(board_idx, &mut ui);
    }
}
