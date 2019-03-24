use conrod::{
    Positionable,
    Widget,
    WidgetKind,
    CommonBuilder,
    Backend,
    IndexSlot,
    UpdateArgs,
    Text,
    Colorable,
    color,
};
use evolution::Card;
use debug_widget::DebugWidget;

pub const KIND: WidgetKind = "Deck";

#[derive(Debug)]
pub struct DeckWidget<'a> {
    deck: &'a Vec<Card>,
    common: CommonBuilder,
}

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    label_idx: IndexSlot,
    deck_idx: IndexSlot,
}

impl<'a> DeckWidget<'a> {
    pub fn new(deck: &'a Vec<Card>) -> Self {
        DeckWidget {
            deck: deck,
            common: CommonBuilder::new(),
        }
    }
}

impl <'a> Widget for DeckWidget<'a> {
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
            label_idx: IndexSlot::new(),
            deck_idx: IndexSlot::new(),
        }
    }

    fn style(&self) -> Self::Style {
        ()
    }

    fn update<B: Backend>(self, args: UpdateArgs<Self, B>) {
        let UpdateArgs { idx, state, mut ui, .. } = args;

        let deck_idx = state.view().deck_idx.get(&mut ui);
        DebugWidget::new(self.deck, true)
                    .top_left_of(idx)
                    .graphics_for(idx)
                    .set(deck_idx, &mut ui);

        let label_idx = state.view().label_idx.get(&mut ui);
        Text::new("Deck: ")
             .top_left_with_margins_on(deck_idx, -20.0, 0.0)
             .color(color::WHITE)
             .graphics_for(idx)
             .set(label_idx, &mut ui);
    }
}
