use std::rc::Rc;
use std::cell::RefCell;

type Node<C> = Rc<RefCell<State<C>>>;

#[derive(Default)]
pub struct Machine<C> {
    start: Option<Node<C>>,
    states: Vec<Node<C>>,
}

impl<C> Machine<C> {
    pub fn add_state(&mut self, func: Box<Fn(&mut C)>) -> usize {
        let state = State::new(func);
        self.states.push(Rc::new(RefCell::new(state)));
        self.states.len() - 1
    }

    pub fn add_goto(&mut self, from: usize, to: usize) {
        // TODO: Check indices.
        let to = self.states[to].clone();
        self.states[from].borrow_mut().set_flow(Flow::Goto(to));
    }

    pub fn add_branch(&mut self, from: usize, tru: usize, fals: usize, pred: Box<Fn(&C) -> bool>) {
        // TODO: Check indices.
        let tru = self.states[tru].clone();
        let fals = self.states[fals].clone();
        self.states[from].borrow_mut().set_flow(Flow::Branch(tru, fals, pred));
    }

    pub fn set_start(&mut self, start: usize) {
        self.start = Some(self.states[start].clone());
    }

    pub fn run(&self, context: &mut C) {
        if let Some(ref start) = self.start {
            start.borrow().step(context)
        } else {
            panic!("no start node");
        }
    }
}

struct State<C> {
    func: Box<Fn(&mut C)>,
    flow: Option<Flow<C>>,
}

impl<C> State<C> {
    fn new(func: Box<Fn(&mut C)>) -> Self {
        State {
            func: func,
            flow: None,
        }
    }

    fn set_flow(&mut self, flow: Flow<C>) {
        self.flow = Some(flow);
    }

    fn step(&self, context: &mut C) {
        (self.func)(context);
        if let Some(ref flow) = self.flow {
            flow.next(context).borrow().step(context);
        }
    }
}

enum Flow<C> {
    Goto(Node<C>),
    Branch(Node<C>, Node<C>, Box<Fn(&C) -> bool>),
}

impl<C> Flow<C> {
    fn next(&self, context: &C) -> &Node<C> {
        match *self {
            Flow::Goto(ref node) => node,
            Flow::Branch(ref t, ref f, ref p) => {
                if p(context) {
                    t
                } else {
                    f
                }
            }
        }
    }
}

#[test]
fn pass_state_and_context() {
    struct AddStep(usize);
    struct End;

    let mut machine = Machine::<usize>::default();
    let iter = machine.add_state(move || Step(0));
    let end = machine.add_state(move || End);
    machine.add_branch(iter, iter, end, move |c, s| *c < 10));
    let mut context = 0;
    machine.set_start(iter);
    machine.run(&mut context);
    assert_eq!(10, context);
}

// #[test]
// fn goto() {
//     let mut machine = Machine::<usize>::default();
//     let a = machine.add_state(Box::new(|c| *c += 1));
//     let b = machine.add_state(Box::new(|c| *c *= 2));
//     machine.add_goto(a, b);
//     let mut context = 1;
//     machine.set_start(a);
//     machine.run(&mut context);
//     assert_eq!(4, context);
// }
//
// #[test]
// fn branch() {
//     let mut machine = Machine::<usize>::default();
//     let iter = machine.add_state(Box::new(|c| *c += 1));
//     let end = machine.add_state(Box::new(|c| println!("END: {:?}", c)));
//     machine.add_branch(iter, iter, end, Box::new(|c| *c < 10));
//     let mut context = 0;
//     machine.set_start(iter);
//     machine.run(&mut context);
//     assert_eq!(10, context);
// }
