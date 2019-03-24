/// Objects with mutable knowledge of the context they live in.
///
/// We use this to represent a `Species` within the context of a `Game` for
/// example. Here the mutable context is used to implement actions which
/// effect other objects in the game outside ourselves.
///
/// The main abstraction this provides is a way to create back references
/// within the ownership model in Rust. For example a player owns a collection
/// of species, however using a game context we can write `species.player()`
/// to get a reference to the player who owns the species.
///
/// Since everything is mutable here, and we're relying under the hood on
/// indices to manage the back references, a change which effects the indices
/// of any of the data will break all objects relying on those indices.
///
/// It stands to reason this interface could be tweaked with `Rc` and
/// `Weak` pointers to actually make strong guarantees about the access
/// of objects within a context.
pub trait Context<'a, T> {
    /// Return the context of this object.
    fn context(&self) -> &T;

    /// Return the mutable context of this object.
    fn context_mut(&mut self) -> &mut T;
}

mod board;
pub use self::board::Board;

mod player;
pub use self::player::Player;

mod situation;
pub use self::situation::Situation;

mod species;
pub use self::species::Species;
