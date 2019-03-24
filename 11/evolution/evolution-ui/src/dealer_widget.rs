use conrod::{
    Positionable,
    Widget,
    WidgetKind,
    CommonBuilder,
    Backend,
    Sizeable,
    Text,
    IndexSlot,
    UpdateArgs,
};
use evolution_server::Dealer;
use game_widget::GameWidget;
use deck_widget::DeckWidget;

pub const KIND: WidgetKind = "Dealer";

#[derive(Debug)]
pub struct DealerWidget<'a> {
    dealer: &'a Dealer,
    common: CommonBuilder,
}

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    game_idx: IndexSlot,
    deck_idx: IndexSlot,
}

impl<'a> DealerWidget<'a> {
    pub fn new(dealer: &'a Dealer) -> Self {
        DealerWidget {
            dealer: dealer,
            common: CommonBuilder::new(),
        }
    }
}

impl<'a> Widget for DealerWidget<'a> {
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
            game_idx: IndexSlot::new(),
            deck_idx: IndexSlot::new(),
        }
    }

    fn style(&self) -> Self::Style {
        ()
    }

    fn update<B: Backend>(self, args: UpdateArgs<Self, B>) {
        let UpdateArgs { idx, state, mut ui, .. } = args;

        let game_idx = state.view().game_idx.get(&mut ui);
        match self.dealer.game() {
            Some(game) => {
                GameWidget::new(game)
                            .top_left_of(idx)
                            .graphics_for(idx)
                            .set(game_idx, &mut ui);
            },
            None => {
                Text::new("No game state yet!")
                      .top_left_of(idx)
                      .graphics_for(idx)
                      .set(game_idx, &mut ui);
            }
        }

        let deck_idx = state.view().deck_idx.get(&mut ui);
        DeckWidget::new(self.dealer.deck())
                    .y_relative_to(game_idx, -220.0)
                    .wh([800.0, 400.0])
                    .graphics_for(idx)
                    .set(deck_idx, &mut ui);
    }
}
