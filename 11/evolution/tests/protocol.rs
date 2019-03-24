extern crate evolution_wire;
extern crate evolution;

use std::thread;
use std::time::Duration;
use evolution_wire::*;
use evolution::Game;
use evolution::object::{FoodToken, Trait as GameTrait, Placement};
use evolution::interact::{self, Choose, Observe};
use evolution::silly::Silly;

macro_rules! test_action_choose {
    {
        observation_wire_type = $observation_wire_type:ty;
        choice_wire_type = $choice_wire_type:ty;
        channel_name = $channel:ident;
        game = $game:ident;
        server_pre => $server_pre:block
        client => $client:block
    } => {{
        let server = thread::spawn(move || {
            // Establish a channel, we'll accept.
            let mut $channel = Channel::accept_from_socket_addr("127.0.0.1:1337").unwrap();

            // Make a game, used just for the state.
            let mut $game = Game::<Silly>::new(3).unwrap();
            $game.step_deal().expect("failed to deal");

            $server_pre;

            // Make a call.
            let observation: interact::ActionObservation = $game.observe();
            let wire_observation = ToWire::<$observation_wire_type>::to_wire(&observation);
            let wire_choice: $choice_wire_type = $channel.call(&wire_observation).expect("failed to call");
            let actual = interact::ActionChoice::from_wire(wire_choice).expect("invalid action choice");
            let expected = interact::ActionChoice {
                food_card: 0,
                population_growths: vec![
                    interact::Growth {
                        species_index: 1,
                        card_index: 3
                    }
                ],
                body_growths: vec![],
                boards: vec![
                    interact::BoardTrade {
                        card_index: 1,
                        trait_card_indeces: vec![2]
                    }
                ],
                traits: vec![]
            };
            assert_eq!(expected, actual);
        });
        thread::sleep(Duration::from_millis(10));

        // Establish a channel, we'll connect.
        let mut $channel = Channel::connect_to_socket_addr("info".into(), "127.0.0.1:1337").unwrap();

        $client;

        server.join().unwrap();
    }}
}

#[test]
fn action_choose_wire_v1() {
    test_action_choose! {
        observation_wire_type = Choice;
        choice_wire_type = Action4;
        channel_name = channel;
        game = game;

        server_pre => {}
        client => {
            channel.accept_call(&|o: Choice| {
                let observation = interact::ActionObservation::from_wire(o).expect("invalid action observation");
                let choice = Silly.choose(&observation).expect("failed to make choice").expect("no choice");
                ToWire::<Action4>::to_wire(&choice)
            }).expect("failed to accept");
        }
    }
}

#[test]
fn action_choose_wire_v2() {
    test_action_choose! {
        observation_wire_type = (remote::LOB, remote::LOB);
        choice_wire_type = remote::Action4;
        channel_name = channel;
        game = game;

        server_pre => {
            let board = game.board().clone();
            let current_player = game.current_player().clone();
            let start = (board, current_player).observe().to_wire();
            channel.send(&start).expect("failed to send");
        }
        client => {
            let start: remote::Start = channel.recv().expect("failed to accept");
            channel.accept_call(&|o: (remote::LOB, remote::LOB)| {
                let observation = interact::ActionObservation::from_wire((start.clone(), o)).expect("invalid action observation");
                let choice = Silly.choose(&observation).expect("failed to make choice").expect("no choice");
                ToWire::<remote::Action4>::to_wire(&choice)
            }).expect("failed to accept");
        }
    }
}

macro_rules! test_feed_choose {
    ($observation_wire_type:ty, $choice_wire_type:ty) => {{
        let server = thread::spawn(move || {
            // Establish a channel, we'll accept.
            let mut channel = Channel::accept_from_socket_addr("127.0.0.1:1337").unwrap();

            // Make a game.
            let mut game = Game::<Silly>::new(3).unwrap();
            game.board_mut().push_food(FoodToken);
            game.players_mut()[0].domain_mut().add(Placement::Right);
            game.players_mut()[0].domain_mut()[0].evolve(GameTrait::FatTissue).unwrap();
            game.players_mut()[0].domain_mut()[0].grow().unwrap();

            // Make a call.
            let observation: interact::FeedObservation = game.observe();
            let wire_observation = ToWire::<$observation_wire_type>::to_wire(&observation);
            let wire_choice: $choice_wire_type = channel.call(&wire_observation).expect("failed to call");
            let actual = interact::FeedChoice::from_wire(wire_choice).expect("invalid feed choice");
            let expected = interact::FeedChoice::Store(0, 1);
            assert_eq!(expected, actual);
        });
        thread::sleep(Duration::from_millis(10));
        let mut channel = Channel::connect_to_socket_addr("info".into(), "127.0.0.1:1337").unwrap();
        channel.accept_call(&|o: $observation_wire_type| {
            let observation = interact::FeedObservation::from_wire(o).expect("invalid feed observation");
            let choice = Silly.choose(&observation)
                              .expect("failed to make choice")
                              .expect("no choice");
            ToWire::<$choice_wire_type>::to_wire(&choice)
        }).expect("failed to accept");

        server.join().unwrap();
    }}
}

#[test]
fn feed_choose_wire_v1() {
    test_feed_choose!(Feeding, FeedChoice);
}

#[test]
fn feed_choose_wire_v2() {
    test_feed_choose!(remote::State, remote::FeedingChoice);
}
