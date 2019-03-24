use std::fmt::Debug;
use conrod::{
    color,
    Colorable,
    Positionable,
    Widget,
    Text,
    Backend,
    WidgetKind,
    CommonBuilder,
    IndexSlot,
    UpdateArgs,
};

pub const KIND: WidgetKind = "Debug";

#[derive(Debug)]
pub struct DebugWidget<'a> {
    pretty: bool,
    debug: &'a Debug,
    common: CommonBuilder,
}

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    debug_idx: IndexSlot,
}

impl<'a> DebugWidget<'a> {
    pub fn new(debug: &'a Debug, pretty: bool) -> Self {
        DebugWidget {
            pretty: pretty,
            debug: debug,
            common: CommonBuilder::new(),
        }
    }
}

impl<'a> Widget for DebugWidget<'a> {
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
            debug_idx: IndexSlot::new(),
        }
    }

    fn style(&self) -> Self::Style {
        ()
    }

    fn update<B: Backend>(self, args: UpdateArgs<Self, B>) {
        let UpdateArgs { idx, state, mut ui, .. } = args;

        let text = if self.pretty {
            format!("{:#?}", self.debug)
        } else {
            format!("{:?}", self.debug)
        };

        let debug_idx = state.view().debug_idx.get(&mut ui);
        Text::new(&text)
             .top_left_of(idx)
             .font_size(15)
             .graphics_for(idx)
             .color(color::WHITE)
             .set(debug_idx, &mut ui);
    }
}
