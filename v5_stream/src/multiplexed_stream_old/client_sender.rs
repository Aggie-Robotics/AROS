use core::marker::PhantomData;

use v5_traits::error::CustomError;
use v5_traits::stream::{MessageStreamCreator, SendStream};
use v5_traits::UniversalFunctions;

use crate::multiplexed_stream::identifiable::Identifiable;
use alloc::vec::Vec;
use alloc::format;

#[derive(Debug)]
pub struct ClientSender<U, T, C>
    where U: UniversalFunctions,
          T: Identifiable + Send,
          C: MessageStreamCreator<Vec<u8>> + Send + Sync{
    sender: C::Sender,
    universal_functions: U,
    phantom_t: PhantomData<T>,
}
impl<U, T, C> ClientSender<U, T, C>
    where U: UniversalFunctions,
          T: Identifiable + Send,
          C: MessageStreamCreator<Vec<u8>> + Send + Sync{
    pub fn new(sender: C::Sender, universal_functions: U) -> Self{
        Self{ sender, phantom_t: Default::default(), universal_functions }
    }
}
impl<U, T, C> SendStream<T> for ClientSender<U, T, C>
    where U: UniversalFunctions,
          T: Identifiable + Send + Sync,
          C: MessageStreamCreator<Vec<u8>> + Send + Sync{
    type Error = CustomError;

    fn send(&self, val: T) -> Result<(), Self::Error> {
        let data = match serde_cbor::to_vec(&val){
            Ok(data) => data,
            Err(error) => return Err(CustomError::new(true, format!("Serde Cbor error sending: {}", error)))
        };
        match self.sender.send(data){
            Ok(_) => Ok(()),
            Err(error) => return Err(CustomError::from_error("ClientSender::send", error))
        }
    }

    fn send_vec(&self, data: Vec<T>) -> Result<Option<Vec<T>>, Self::Error> {
        self.send_whole_vec(data)?;
        Ok(None)
    }

    fn send_whole_vec(&self, data: Vec<T>) -> Result<(), Self::Error> {
        let mut values = Vec::with_capacity(data.len());
        for value in data{
            values.push(match serde_cbor::to_vec(&value) {
                Ok(value) => value,
                Err(error) => return Err(CustomError::new(true, format!("Serde Cbor Error send_whole_vec {}", error)))
            })
        }
        match self.sender.send_whole_vec(values){
            Ok(_) => Ok(()),
            Err(error) => Err(CustomError::from_error("ClientSender::send_whole_vec", error))
        }
    }
}
