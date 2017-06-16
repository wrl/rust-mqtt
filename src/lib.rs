#[macro_use]
extern crate log;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate error_chain;
extern crate rand;
#[macro_use]
extern crate nom;
extern crate byteorder;
extern crate slab;

mod error;
#[macro_use]
mod topic;
#[macro_use]
mod proto;
mod packet;
mod encode;
mod decode;

pub use proto::{QoS, ClientId};
pub use topic::{Level, Topic, TopicTree, MatchTopic};
pub use packet::{Packet, LastWill, ConnectReturnCode, SubscribeReturnCode};
pub use encode::WritePacketExt;
pub use decode::{ReadPacketExt, read_packet};

// http://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.xhtml
pub const TCP_PORT: u16 = 1883;
pub const SSL_PORT: u16 = 8883;
