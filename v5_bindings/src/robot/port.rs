use init_with::InitWith;

use crate::raw::vex_os::api::vexDeviceGetByIndex;
use crate::raw::vex_os::api_types::V5_DeviceT;

pub type PortType = V5_DeviceT;

#[derive(Debug)]
pub struct Port {
    number: u8,
    device: PortType,
}
impl Port {
    #[cfg(not(feature = "zero_based_ports"))]
    pub(crate) fn get_all() -> [Option<Self>; 22] {
        <[Option<Self>; 22]>::init_with_indices(|i| {
            if i == 0 {
                None
            } else {
                Some(Self {
                    number: i as u8,
                    device: unsafe { vexDeviceGetByIndex(i as u32 - 1) }
                })
            }
        })
    }

    #[cfg(feature = "zero_based_ports")]
    pub(crate) fn get_all() -> [Self; 21] {
        <[Self; 21]>::init_with_indices(|i| { Self{ number: i as u8, device: unsafe { vexDeviceGetByIndex(i as u32) } } })
    }

    pub const fn device(&self) -> PortType {
        self.device
    }

    pub const fn number(&self) -> u8{
        self.number
    }
}
unsafe impl Send for Port{}
unsafe impl Sync for Port{}
