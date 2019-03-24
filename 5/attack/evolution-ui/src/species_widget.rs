use conrod::{
    self,
    Positionable,
    IndexSlot,
    UpdateArgs
};
use evolution::object::*;
use debug_widget::DebugWidget;

pub const KIND: conrod::WidgetKind = "Species";

#[derive(Debug)]
pub struct SpeciesWidget<'a> {
    species: &'a Species,
    common: conrod::CommonBuilder,
}

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    population_idx: IndexSlot,
    body_size_idx: IndexSlot,
    food_idx: IndexSlot,
    fat_idx: IndexSlot,
    traits_idx: IndexSlot,
}

impl<'a> SpeciesWidget<'a> {
    pub fn new(species: &'a Species) -> Self {
        SpeciesWidget {
            species: species,
            common: conrod::CommonBuilder::new(),
        }
    }
}

impl<'a> conrod::Widget for SpeciesWidget<'a> {
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
            population_idx: IndexSlot::new(),
            body_size_idx: IndexSlot::new(),
            food_idx: IndexSlot::new(),
            fat_idx: IndexSlot::new(),
            traits_idx: IndexSlot::new(),
        }
    }

    fn style(&self) -> Self::Style {
        ()
    }

    fn update<B: conrod::Backend>(self, args: conrod::UpdateArgs<Self, B>) {
        let UpdateArgs { idx, state, mut ui, .. } = args;

        let population_idx = state.view().population_idx.get(&mut ui);
        DebugWidget::new(&("population", self.species.population()), false)
                    .top_left_of(idx)
                    .graphics_for(idx)
                    .set(population_idx, &mut ui);

        let body_size_idx = state.view().body_size_idx.get(&mut ui);
        DebugWidget::new(&("body_size", self.species.body_size()), false)
                    .y_relative_to(population_idx, -25.0)
                    .graphics_for(idx)
                    .set(body_size_idx, &mut ui);

        let food_idx = state.view().food_idx.get(&mut ui);
        DebugWidget::new(&("food", self.species.food()), false)
                    .y_relative_to(body_size_idx, -25.0)
                    .graphics_for(idx)
                    .set(food_idx, &mut ui);

        let traits_idx = state.view().traits_idx.get(&mut ui);
        DebugWidget::new(&self.species.traits(), false)
                    .y_relative_to(food_idx, -25.0)
                    .graphics_for(idx)
                    .set(traits_idx, &mut ui);

        if self.species.has_trait(Trait::FatTissue) {
            let fat_idx = state.view().fat_idx.get(&mut ui);
            DebugWidget::new(&("fat", self.species.fat()), false)
                        .y_relative_to(traits_idx, -25.0)
                        .graphics_for(idx)
                        .set(fat_idx, &mut ui);
        }
    }
}
