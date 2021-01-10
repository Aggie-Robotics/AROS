use crate::raw::pros::apix::*;
use core::marker::PhantomData;
use core::mem::{size_of, forget, MaybeUninit};
use core::time::Duration;
use cty::c_void;
use crate::sync::option_to_timeout;

/// A queue that allows the sending of data across thread boundaries
/// Sends data of type T
#[derive(Debug)]
pub struct Queue<T> where T: 'static + Send{
    queue: queue_t,
    max_length: u32,
    phantom: PhantomData<T>,
}
impl<T> Queue<T> where T: 'static + Send{
    /// Creates a new queue that can store up to max_length messages
    pub fn new(max_length: u32) -> Self{
        Self{
            queue: unsafe{queue_create(max_length, size_of::<T>() as u32)},
            max_length,
            phantom: Default::default(),
        }
    }

    /// Prepends data to the front of the queue
    /// Will wait up to timeout for a spot in the queue
    /// Returns Ok if sent or Err if queue full and timeout reached
    pub fn prepend(&self, item: T, timeout: Option<Duration>) -> Result<(), T>{
        if unsafe { queue_prepend(self.queue, &item as *const T as *const c_void, option_to_timeout(timeout)) }{
            forget(item);
            Ok(())
        }
        else{
            Err(item)
        }
    }
    /// Appends to the queue
    /// Will wait up to timeout for a spot in the queue
    /// Returns Ok if sent or Err if queue full and timeout reached
    pub fn append(&self, item: T, timeout: Option<Duration>) -> Result<(), T>{
        if unsafe { queue_append(self.queue, &item as *const T as *const c_void, option_to_timeout(timeout)) }{
            forget(item);
            Ok(())
        }
        else{
            Err(item)
        }
    }

    /// Pulls an item out of the queue
    /// Returns Some if item pulled or None if timeout reached
    pub fn receive(&self, timeout: Option<Duration>) -> Option<T>{
        let mut out = MaybeUninit::uninit();
        if unsafe { queue_recv(self.queue, out.as_mut_ptr() as *mut c_void, option_to_timeout(timeout)) }{
            Some(unsafe { out.assume_init() })
        }
        else{
            None
        }
    }

    /// The amount of items in the queue
    pub fn len(&self) -> u32{
        unsafe { queue_get_waiting(self.queue) }
    }
    /// The maximum items this queue can store
    pub fn max_len(&self) -> u32{
        self.max_length
    }

    /// Clears all items from the queue dropping each
    pub fn clear(&self){
        while let Some(item) = self.receive(Some(Duration::new(0, 0))){
            drop(item);
        }
    }
}
impl<T> Queue<T> where T: 'static + Send + Copy{
    /// Copies the item at the front of the queue if T implements copy
    /// Will wait up to timeout for an item
    /// Returns some with the copied item or None if timeout reached
    pub fn peek(&self, timeout: Option<Duration>) -> Option<T>{
        let mut out = MaybeUninit::uninit();
        if unsafe { queue_peek(self.queue, out.as_mut_ptr() as *mut c_void, option_to_timeout(timeout)) }{
            Some(unsafe { out.assume_init() })
        }
        else{
            None
        }
    }
}
impl<T> Drop for Queue<T> where T: 'static + Send{
    fn drop(&mut self) {
        self.clear();
        unsafe { queue_delete(self.queue) }
    }
}
unsafe impl<T> Send for Queue<T> where T: 'static + Send{}
unsafe impl<T> Sync for Queue<T> where T: 'static + Send{}

#[cfg(feature = "v5_test")]
pub mod test{
    use crate::sync::queue::Queue;
    use crate::test::{assert, TestItem, TestType};
    use alloc::boxed::Box;
    use alloc::string::ToString;
    use core::time::Duration;

    pub fn queue_test() -> TestItem{
        TestItem::new("queue_test".to_string(), TestType::Parallel(Box::new(|| {
            let queue_length = 100;
            let queue = Queue::new(queue_length);
            assert(queue.max_len() == 100, format!("Queue max_length invalid! Should be: {}, is: {}", queue_length, queue.max_len()))?;
            assert(queue.len() == 0, format!("Queue length invalid at initialization! Should be: {}, is {}", 0, queue.len()))?;
            let insert_val = 1424;
            if let Err(_) = queue.append(insert_val, Some(Duration::from_millis(100))){
                return Err(format!("Could not insert {} into queue", insert_val));
            }
            assert(queue.len() == 1, format!("Queue length invalid! Should be: {}, is {}", 1, queue.len()))?;
            let received = queue.receive(Some(Duration::from_millis(100)));
            assert(received.is_some(), format!("Could not pull from queue"))?;
            assert(received.unwrap() == insert_val, format!("Value from queue wrong! Should be: {}, is: {}", insert_val, received.unwrap()))?;
            assert(queue.len() == 0, format!("Queue length invalid after received! Should be: {}, is {}", 0, queue.len()))?;
            Ok(())
        }), Duration::from_secs(1)))
    }
}
