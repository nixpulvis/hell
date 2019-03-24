pub use self::board::Board;
pub use self::card::Card;
pub use self::stack::Stack;

macro_rules! create_wrapper {
    ($name:ident) => {
        use std::ops::{Deref, DerefMut};

        #[derive(Debug, Serialize)]
        pub struct $name(take5::$name);

        impl Deref for $name {
            type Target = take5::$name;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl From<take5::$name> for $name {
            fn from(wrapped: take5::$name) -> Self {
                $name(wrapped)
            }
        }

        impl From<$name> for take5::$name {
            fn from(wrapper: $name) -> Self {
                wrapper.0
            }
        }
    };
}

mod board;
mod card;
mod stack;
