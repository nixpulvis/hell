use conrod::{
    self,
    Positionable,
    WidgetMatrix,
    IndexSlot,
    UpdateArgs,
};
use evolution::Player;
use species_widget::SpeciesWidget;
use debug_widget::DebugWidget;

pub const KIND: conrod::WidgetKind = "Player";

#[derive(Debug)]
pub struct PlayerWidget<'a> {
    player: &'a Player,
    common: conrod::CommonBuilder,
}

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    id_idx: IndexSlot,
    domain_idx: IndexSlot,
    bag_idx: IndexSlot,
    hand_idx: IndexSlot,
}

impl<'a> PlayerWidget<'a> {
    pub fn new(player: &'a Player) -> Self {
        PlayerWidget {
            player: player,
            common: conrod::CommonBuilder::new(),
        }
    }
}

impl<'a> conrod::Widget for PlayerWidget<'a> {
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
            id_idx: IndexSlot::new(),
            domain_idx: IndexSlot::new(),
            bag_idx: IndexSlot::new(),
            hand_idx: IndexSlot::new(),
        }
    }

    fn style(&self) -> Self::Style {
        ()
    }

    fn update<B: conrod::Backend>(self, args: conrod::UpdateArgs<Self, B>) {
        let UpdateArgs { idx, state, mut ui, .. } = args;

        let id_idx = state.view().id_idx.get(&mut ui);
        DebugWidget::new(&("id", self.player.id()), false)
                    .top_left_of(idx)
                    .graphics_for(idx)
                    .set(id_idx, &mut ui);

        let bag_idx = state.view().bag_idx.get(&mut ui);
        DebugWidget::new(&("bag", self.player.bag()), false)
                    .y_relative_to(id_idx, -25.0)
                    .graphics_for(idx)
                    .set(bag_idx, &mut ui);

        let hand_idx = state.view().hand_idx.get(&mut ui);
        DebugWidget::new(&("hand", self.player.hand()), false)
                    .y_relative_to(bag_idx, -25.0)
                    .graphics_for(idx)
                    .set(hand_idx, &mut ui);

        let domain_idx = state.view().domain_idx.get(&mut ui);
        WidgetMatrix::new(1, self.player.domain().len())
               .each_widget(|n, _, _| SpeciesWidget::new(&self.player.domain()[n]))
               .y_relative_to(hand_idx, -50.0)
               .graphics_for(idx)
               .set(domain_idx, &mut ui);
    }
}
