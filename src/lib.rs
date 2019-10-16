mod connect;
mod decoder;
mod encoder;
mod header;
mod packet;
mod publish;
mod subscribe;
mod utils;

#[cfg(test)]
mod codec_test;
#[cfg(test)]
mod decoder_test;
#[cfg(test)]
mod encoder_test;

pub use crate::{
    connect::{Connack, Connect},
    decoder::decode,
    encoder::encode,
    header::Protocol,
    packet::{Packet, PacketType},
    publish::Publish,
    subscribe::{Suback, Subscribe, SubscribeReturnCodes, SubscribeTopic, Unsubscribe},
    utils::{ConnectReturnCode, LastWill, PacketIdentifier, QoS, QosPid},
};

const MULTIPLIER: usize = 0x80 * 0x80 * 0x80 * 0x80;
const MAX_PAYLOAD_SIZE: usize = 268435455;
