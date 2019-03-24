use conrod::{self, Positionable, IndexSlot, UpdateArgs};
use evolution::Board;
use debug_widget::DebugWidget;

pub const KIND: conrod::WidgetKind = "Board";

#[derive(Debug)]
pub struct BoardWidget<'a> {
    board: &'a Board,
    common: conrod::CommonBuilder,
}

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    rectangle_idx: IndexSlot,
    json_idx: IndexSlot,
}

impl<'a> BoardWidget<'a> {
    pub fn new(board: &'a Board) -> Self {
        BoardWidget {
            board: board,
            common: conrod::CommonBuilder::new(),
        }
    }
}

impl<'a> conrod::Widget for BoardWidget<'a> {
    type State = State;
    type Style = ();

    fn common(&self) -> &conrod::CommonBuilder {
        &self.common
    }

    fn common_mut(&mut self) -> &mut conrod::CommonBuilder {
        &mut self.common
    }

    fn unique_kind(&self) -> conrod::WidgetKind {
        KIND
    }

    fn init_state(&self) -> Self::State {
        State {
            rectangle_idx: IndexSlot::new(),
            json_idx: IndexSlot::new(),
        }
    }

    fn style(&self) -> Self::Style {
        ()
    }

    fn update<B: conrod::Backend>(self, args: conrod::UpdateArgs<Self, B>) {
        let UpdateArgs { idx, state, mut ui, .. } = args;

        let json_idx = state.view().json_idx.get(&mut ui);
        DebugWidget::new(self.board, true)
                    .middle_of(idx)
                    .graphics_for(idx)
                    .set(json_idx, &mut ui);
    }
}
