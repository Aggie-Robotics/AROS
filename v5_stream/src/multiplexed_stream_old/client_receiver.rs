use core::marker::PhantomData;
use alloc::format;

use v5_traits::error::CustomError;
use v5_traits::stream::{MessageStreamCreator, ReceiveStream};

use crate::multiplexed_stream::identifiable::Identifiable;
use v5_traits::UniversalFunctions;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct ClientReceiver<U, T, C>
    where U: UniversalFunctions,
          T: Identifiable + Send + Sync,
          C: MessageStreamCreator<Vec<u8>> + Send + Sync{
    receiver: C::Receiver,
    universal_functions: U,
    phantom_t: PhantomData<T>
}
impl<U, T, C> ClientReceiver<U, T, C>
    where U: UniversalFunctions,
          T: Identifiable + Send + Sync,
          C: MessageStreamCreator<Vec<u8>> + Send + Sync{
    pub fn new(receiver: C::Receiver, universal_functions: U) -> Self{
        Self{ receiver, universal_functions, phantom_t: Default::default() }
    }
}
impl<U, T, C> ReceiveStream<T> for ClientReceiver<U, T, C>
    where U: UniversalFunctions,
          T: Identifiable + Send + Sync,
          C: MessageStreamCreator<Vec<u8>> + Send + Sync{
    type Error = CustomError;

    fn try_receive(&self) -> Result<Option<T>, Self::Error> {
        match self.receiver.try_receive() {
            Ok(received) => match received{
                None => Ok(None),
                Some(data) => match serde_cbor::from_slice(&data){
                    Ok(value) => Ok(Some(value)),
                    Err(error) => Err(CustomError::new(true, format!("Could not deserialize: {}", error))),
                }
            }
            Err(error) => Err(CustomError::from_error("ReceiveStream::try_receive", error))
        }
    }

    fn receive(&self) -> Result<T, Self::Error> {
        match self.receiver.receive() {
            Ok(data) =>  match serde_cbor::from_slice(&data){
                Ok(value) => Ok(value),
                Err(error) => Err(CustomError::new(true, format!("Could not deserialize: {}", error))),
            }
            Err(error) => Err(CustomError::from_error("ReceiveStream::receive", error))
        }
    }

    fn receive_vec(&self, limit: usize) -> Result<Vec<T>, Self::Error> {
        match self.receiver.receive_vec(limit){
            Ok(datas) => {
                let mut out = Vec::with_capacity(datas.len());
                for data in datas{
                    match serde_cbor::from_slice(&data){
                        Ok(value) => out.push(value),
                        Err(error) => return Err(CustomError::new(true, format!("Serde cbor error: {}", error))),
                    }
                }
                Ok(out)
            }
            Err(error) => Err(CustomError::from_error("ReceiveStream::receive_vec", error))
        }
    }

    fn receive_whole_vec(&self, vec: &mut Vec<T>, limit: usize) -> Result<(), Self::Error> {
        let mut buffer = Vec::with_capacity(vec.len());
        match self.receiver.receive_whole_vec(&mut buffer, limit){
            Ok(()) => {
                for data in buffer{
                    match serde_cbor::from_slice(&data){
                        Ok(value) => vec.push(value),
                        Err(error) => return Err(CustomError::new(true, format!("Serde cbor error: {}", error))),
                    }
                }
                Ok(())
            }
            Err(error) => Err(CustomError::from_error("ReceiveStream::receive_vec", error))
        }
    }
}
