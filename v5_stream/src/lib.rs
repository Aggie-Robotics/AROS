#![no_std]

#[cfg(feature = "std")]
extern crate std;
extern crate alloc;
use crate::serialize_stream::SerializeStream;
use crate::simple_byte_stream::SimpleByteStream;
use crate::multiplexed_stream::{MultiplexedStream, MultiplexPacket, ChannelIndexType, DataPacket};
use v5_traits::{NamedUniversal, UniversalFunctions};
use v5_traits::stream::{DuplexStream, MessageStreamCreator};

pub mod simple_byte_stream;
pub mod composed_stream;
pub mod multiplexed_stream;
pub mod serialize_stream;

pub type SuperStream<UF, S, C> =
MultiplexedStream<
    NamedUniversal<UF>,
    SerializeStream<
        NamedUniversal<UF>,
        MultiplexPacket,
        SimpleByteStream<
            NamedUniversal<UF>,
            S
        >
    >,
    C
>;
pub fn create_super_stream<UF, S, C>(uf: UF, stream: S, creator: &C, num_channels: ChannelIndexType) -> SuperStream<UF, S, C>
    where UF: UniversalFunctions + Clone,
          S: DuplexStream<u8>,
          C: MessageStreamCreator<DataPacket>{
    MultiplexedStream::new(
        NamedUniversal::new(uf.clone(), "MultiplexStream"),
        SerializeStream::new(
            NamedUniversal::new(uf.clone(), "SerializeStream"),
            SimpleByteStream::new(
                NamedUniversal::new(uf, "ByteStream"),
                stream
            )
        ),
        creator,
        num_channels
    )
}

//TODO: Make a binary packet handler
