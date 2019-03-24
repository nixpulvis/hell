use serde::de::{SeqVisitor, Visitor, Error};
use serde::ser::impls::TupleVisitor2;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use super::{Configuration, Step4};

/// The input to the `xstep4` test harness.
///
/// The STDIN input consists of an array that contains two arrays: the first is
/// `Configuration` and the second is a `Step4`. The former represents the state of the dealer
/// before `xstep4` is called; the latter is a JSON representation of `xstep4`’s input.
#[derive(Debug)]
pub struct StartRound {
    pub configuration: Configuration,
    pub step_actions: Step4,
}

impl Serialize for StartRound {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let tuple = (&self.configuration, &self.step_actions);
        serializer.serialize_tuple(TupleVisitor2::new(&tuple))
    }
}

impl Deserialize for StartRound {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        deserializer.deserialize(StartRoundVisitor)
    }
}

struct StartRoundVisitor;

impl Visitor for StartRoundVisitor {
    type Value = StartRound;

    fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
        where V: SeqVisitor
    {
        let configuration = try!(visitor.visit());
        let step_actions = try!(visitor.visit());
        try!(visitor.end());
        match (configuration, step_actions) {
            (Some(configuration), Some(step_actions)) => {
                Ok(StartRound {
                    configuration: configuration,
                    step_actions: step_actions,
                })
            },
            _ => Err(Error::custom("invalid StartRound")),
        }
    }
}
