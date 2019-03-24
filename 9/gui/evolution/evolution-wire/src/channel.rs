use std::ops::{Deref, DerefMut};
use std::fmt::{self, Debug};
use std::io::{Read, Write, Error};
use std::time::Duration;
use std::net::{ToSocketAddrs, TcpStream, TcpListener};
use serde::{Serialize, Deserialize};
use serde_json as json;
use Either;

// HACK: <refactor> We really want our own Error type.
use std::io::ErrorKind;

/// Sending and receiving *whole* wire messages.
pub struct Channel(String, TcpStream);

// TODO: <refactor> Write a macro to abstract the traces.
impl Channel {
    /// Create a stream, and connect to it.
    pub fn connect_to_socket_addr<A: ToSocketAddrs>(info: String, socket_addr: A) -> Result<Channel, Error> {
        let stream = try!(TcpStream::connect(socket_addr));
        Self::connect_to_tcp_stream(info, stream)
    }

    /// Connect to a tcp stream, we'll send the "info" for this channel, we
    /// must get back the response "ok".
    pub fn connect_to_tcp_stream(info: String, stream: TcpStream) -> Result<Channel, Error> {
        let mut channel = Channel(info, stream);
        let info = channel.0.clone();
        let _ack = try!(channel.call::<String, String>(&info));
        // if ack == "ok\n" {
        //     trace!("{:?}::connect", channel);
        //     Ok(channel)
        // } else {
        //     let error = format!("invalid channel ack: {}", ack);
        //     Err(Error::new(ErrorKind::Other, error))
        // }
        Ok(channel)
    }

    pub fn accept_from_socket_addr<A: ToSocketAddrs>(socket_addr: A) -> Result<Channel, Error> {
        let listener = try!(TcpListener::bind(socket_addr));
        let (stream, _) = try!(listener.accept());
        Channel::accept_from_tcp_stream(stream)
    }

    /// Accept from a tcp stream, we must get some "info" and return "ok".
    /// TODO: <question> Boolean to tell channel it was not good?
    pub fn accept_from_tcp_stream(stream: TcpStream) -> Result<Channel, Error> {
        let mut channel = Channel("pending".into(), stream);
        try!(channel.set_read_timeout(Some(Duration::from_secs(2))));
        try!(channel.set_write_timeout(Some(Duration::from_secs(2))));
        let info = try!(channel.recv::<String>());
        try!(channel.send::<String>(&"ok".into()));
        channel.0 = info;
        trace!("Channel::accept => {:?}", channel);
        Ok(channel)
    }

    pub fn info(&self) -> &str {
        &self.0
    }

    pub fn call<D, C>(&mut self, domain: &D) -> Result<C, Error>
        where D: Serialize + Debug,
              C: Deserialize + Debug,
    {
        try!(self.send(&domain));
        let codomain = try!(self.recv());
        Ok(codomain)
    }

    pub fn accept_call<D, C>(&mut self, func: &Fn(D) -> C) -> Result<C, Error>
        where D: Deserialize + Debug,
              C: Serialize + Debug,
    {
        let domain = try!(self.recv());
        // NOTE: We could make this check the `Result` of the call by changing
        // the type of this function to `&Fn(&D) -> Result<C, ...>`.
        let codomain = (func)(domain);
        try!(self.send(&codomain));
        Ok(codomain)
    }

    pub fn send<T: Serialize + Debug>(&mut self, message: &T) -> Result<(), Error> {
        try!(json::to_writer(&mut self.1, message).map_err(|e| {
            warn!("error sending: {}", e);
            Error::new(ErrorKind::Other, e)
        }));
        try!(self.1.write(b"\n"));
        trace!("{:?}.send<T>", self);
        Ok(())
    }

    pub fn recv<T: Deserialize + Debug>(&mut self) -> Result<T, Error> {
        let mut de = json::Deserializer::new(try!(self.1.try_clone()).bytes());
        let message = try!((Deserialize::deserialize(&mut de)).map_err(|e| {
            warn!("error receiving: {}", e);
            Error::new(ErrorKind::Other, e)
        }));
        // HACK: The following *should* work too... wtf serde.
        // let message = try!(json::from_reader(&mut self.1).map_err(|e| {
        //     warn!("error receiving: {}", e);
        //     Error::new(ErrorKind::Other, e)
        // }));
        trace!("{:?}.recv<T>", self);
        Ok(message)
    }

    /// Receive an `Either<T, U>` type from the wire. This provides support for channel consumers
    /// to logically branch based on channel messages.
    pub fn recv_either<T, U>(&mut self) -> Result<Either<T, U>, Error>
        where T: Deserialize + Debug,
              U: Deserialize + Debug,
    {
        let value = try!(self.recv());
        let either = Either::<T, U>::from_value(value);
        either.map_err(|_| Error::new(ErrorKind::InvalidInput, "error creating either type"))
    }
}

impl Deref for Channel {
    type Target = TcpStream;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl DerefMut for Channel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}

impl Debug for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match (self.local_addr(), self.peer_addr()) {
            (Ok(l), Ok(p)) => write!(f, "Channel(info: {}, us: {:?}, them: {:?})", self.0, l, p),
            (Ok(l), Err(p)) => write!(f, "Channel(info: {}, us: {:?}, them: <{:?}>)", self.0, l, p),
            (Err(l), Ok(p)) => write!(f, "Channel(info: {}, us: <{:?}>, them: {:?})", self.0, l, p),
            (Err(l), Err(p)) => write!(f, "Channel(info: {}, us: <{:?}>, them: <{:?}>", self.0, l, p),
        }
    }
}

impl Drop for Channel {
    fn drop(&mut self) {
        trace!("{:?}::drop", self);
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;
    use super::super::*;

    fn accept_channel() -> Channel {
        Channel::accept_from_socket_addr("127.0.0.1:1337").unwrap()
    }

    fn connect_channel() -> Channel {
        Channel::connect_to_socket_addr("info".into(), "127.0.0.1:1337").unwrap()
    }

    #[test]
    fn connect() {
        thread::spawn(move || {
            let result = Channel::accept_from_socket_addr("127.0.0.1:1337");
            assert!(result.is_ok());
        });
        thread::sleep(Duration::from_millis(10));
        thread::spawn(move || {
            let result = Channel::connect_to_socket_addr("info".into(), "127.0.0.1:1337");
            assert!(result.is_ok());
        }).join().unwrap();
    }

    #[test]
    #[ignore]
    fn infinite_length_number() {
        thread::spawn(move || {
            let mut channel = accept_channel();
            let recv: Natural = channel.recv().unwrap();
            // We never get here...
            assert_eq!(1, *recv);
        });

        // Wait for recv thread to spin up.
        thread::sleep(Duration::from_millis(10));

        thread::spawn(move || {
            let mut channel = connect_channel();
            // Write a 1.0, then 0s forever.
            channel.send(&1.0).unwrap();
            loop { channel.send(&0).unwrap() }
        }).join().unwrap();
    }

    #[test]
    fn apex_number() {
        thread::spawn(move || {
            let mut channel = accept_channel();
            let recv: Natural = channel.recv().unwrap();
            // We never get here...
            assert_eq!(1, *recv);
            channel.send(&true).unwrap();
        });

        // Wait for recv thread to spin up.
        thread::sleep(Duration::from_millis(10));

        thread::spawn(move || {
            let mut channel = connect_channel();
            assert_eq!(true, channel.call(&1).unwrap());
        }).join().unwrap();
    }

    #[test]
    fn card_passing() {
        thread::spawn(move || {
            let mut channel = accept_channel();
            let recv: SpeciesCard = channel.recv().unwrap();
            assert_eq!(0, *recv.0);
            let card = SpeciesCard(FoodValue::new(1).unwrap(), Trait::Carnivore);
            channel.send(&card).unwrap();
        });
        thread::sleep(Duration::from_millis(10));
        thread::spawn(move || {
            let mut channel = connect_channel();
            let card = SpeciesCard(FoodValue::new(0).unwrap(), Trait::LongNeck);
            channel.send(&card).unwrap();
            let recv: SpeciesCard = channel.recv().unwrap();
            assert_eq!(1, *recv.0);
        }).join().unwrap();
    }

    #[test]
    fn different_message_types() {
        let receiver = thread::spawn(move || {
            let mut channel = accept_channel();
            let recv: String = channel.recv().unwrap();
            assert_eq!("hello", recv);
            let recv: Vec<i32> = channel.recv().unwrap();
            assert_eq!(vec![1, 2, 3], recv);
        });
        thread::sleep(Duration::from_millis(10));
        thread::spawn(move || {
            let mut channel = connect_channel();
            channel.send(&"hello").unwrap();
            channel.send(&[1, 2, 3]).unwrap();
        });
        receiver.join().unwrap();
    }

    #[test]
    fn receive_either() {
        let receiver = thread::spawn(move || {
            let mut channel = accept_channel();
            let recv: Either<String, u32> = channel.recv_either().unwrap();
            assert_eq!(Either::Left(String::from("hello")), recv);
            let recv: Either<i64, Vec<i32>> = channel.recv_either().unwrap();
            assert_eq!(Either::Right(vec![1, 2, 3]), recv);
        });
        thread::sleep(Duration::from_millis(10));
        thread::spawn(move || {
            let mut channel = connect_channel();
            channel.send(&"hello").unwrap();
            channel.send(&[1, 2, 3]).unwrap();
        });
        receiver.join().unwrap();
    }

    #[test]
    fn string_then_wire_choice() {
        let receiver = thread::spawn(move || {
            let mut channel = accept_channel();
            let recv: String = channel.recv().unwrap();
            assert_eq!("ok", recv);
            let recv: Choice = channel.recv().unwrap();
            assert_eq!(3, *recv.current_player.id);
        });
        thread::sleep(Duration::from_millis(10));
        thread::spawn(move || {
            let mut channel = connect_channel();
            channel.send(&"ok").unwrap();
            let observation = Choice {
                current_player: Player {
                    id: NaturalPlus::new(3).unwrap(),
                    species: vec![
                        Species {
                            food: Nat::new(0).unwrap(),
                            body: Nat::new(0).unwrap(),
                            population: NatPlus::new(1).unwrap(),
                            traits: LOT::new(vec![]).unwrap(),
                            fat_food: None
                        }
                    ],
                    bag: Natural(0),
                    cards: Some(vec![
                        SpeciesCard(FoodValue::new(-2).unwrap(), Trait::Burrowing),
                        SpeciesCard(FoodValue::new(-1).unwrap(), Trait::Burrowing),
                        SpeciesCard(FoodValue::new(0).unwrap(), Trait::Burrowing),
                        SpeciesCard(FoodValue::new(1).unwrap(), Trait::Burrowing)
                    ])
                },
                before: vec![
                    vec![
                        Species {
                            food: Nat::new(0).unwrap(),
                            body: Nat::new(0).unwrap(),
                            population: NatPlus::new(1).unwrap(),
                            traits: LOT::new(vec![]).unwrap(),
                            fat_food: None
                        }
                    ]
                ],
                after: vec![
                    vec![
                        Species {
                            food: Nat::new(0).unwrap(),
                            body: Nat::new(0).unwrap(),
                            population: NatPlus::new(1).unwrap(),
                            traits: LOT::new(vec![]).unwrap(),
                            fat_food: None
                        }
                    ]
                ]
            };
            channel.send(&observation).unwrap();
        });
        receiver.join().unwrap();
    }

    #[test]
    fn step_1() {
        thread::spawn(move || {
            let mut channel = accept_channel();
            let player: Player = Player {
                id: NaturalPlus::new(1).unwrap(),
                species: vec![],
                bag: Natural(0),
                cards: None,
            };
            channel.send(&player).unwrap();
        });
        thread::sleep(Duration::from_millis(10));
        thread::spawn(move || {
            let mut channel = connect_channel();
            let player: Player = channel.recv().unwrap();
            assert_eq!(1, *player.id);
        }).join().unwrap();
    }

    #[test]
    fn step_2_and_3() {
        let server = thread::spawn(move || {
            let mut channel = accept_channel();
            let observation: (remote::LOB, remote::LOB) = (vec![], vec![]);
            channel.send(&observation).unwrap();
            let choice: remote::Action4 = channel.recv().unwrap();
            assert_eq!(0, *choice.0);
            assert!(choice.1.is_empty());
        });
        thread::sleep(Duration::from_millis(10));
        thread::spawn(move || {
            let mut channel = connect_channel();
            let observation: (remote::LOB, remote::LOB) = channel.recv().unwrap();
            assert!(observation.0.is_empty());
            assert!(observation.1.is_empty());
            let choice = remote::Action4(Natural(0), vec![], vec![], vec![], vec![]);
            channel.send(&choice).unwrap();
            server.join().unwrap();
        }).join().unwrap();
    }

    #[test]
    fn step_4() {
        let server = thread::spawn(move || {
            let mut channel = accept_channel();
            let observation: Feeding = Feeding {
                current_player: Player {
                    id: NaturalPlus::new(1).unwrap(),
                    species: vec![],
                    bag: Natural(0),
                    cards: None,
                },
                watering_hole: NaturalPlus::new(3).unwrap(),
                opponents: vec![],
            };
            channel.send(&observation).unwrap();
            let choice: FeedChoice = channel.recv().unwrap();
            assert_eq!(FeedChoice::Abstain, choice);
        });
        thread::sleep(Duration::from_millis(10));
        thread::spawn(move || {
            let mut channel = connect_channel();
            let _: Feeding = channel.recv().unwrap();
            let choice = FeedChoice::Abstain;
            channel.send(&choice).unwrap();
            server.join().unwrap();
        }).join().unwrap();
    }

    #[test]
    fn remote_call() {
        thread::spawn(move || {
            accept_channel().accept_call(&|b: bool| !b).unwrap();
        });
        thread::sleep(Duration::from_millis(10));
        assert_eq!(false, connect_channel().call(&true).unwrap());
    }

    #[test]
    fn remote_calls() {
        thread::spawn(move || {
            let mut channel = accept_channel();
            channel.accept_call(&|b: bool| !b).unwrap();
            channel.accept_call(&|s: bool| format!("{}", s)).unwrap();
        });
        thread::sleep(Duration::from_millis(10));
        let mut channel = connect_channel();
        assert_eq!(false, channel.call(&true).unwrap());
        assert_eq!("false", channel.call::<_, String>(&false).unwrap());
    }
}
