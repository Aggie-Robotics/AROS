use init_with::InitWith;

use crate::raw::vex_os::api::vexDeviceGetByIndex;
use crate::raw::vex_os::api_types::V5_DeviceT;

pub type PortType = V5_DeviceT;

#[derive(Debug)]
pub struct Port {
    device: PortType,
}

impl Port {
    #[cfg(not(feature = "zero_based_ports"))]
    pub fn get_all() -> [Option<Self>; 22] {
        <[Option<Self>; 22]>::init_with_indices(|i| {
            if i == 0 {
                None
            } else {
                Some(Self {
                    device: unsafe { vexDeviceGetByIndex(i as u32 - 1) }
                })
            }
        })
    }

    #[cfg(feature = "zero_based_ports")]
    pub fn get_all() -> [Self; 21] {
        <[Self; 21]>::init_with_indices(|i| { Self{ device: unsafe { vexDeviceGetByIndex(i as u32) } } })
    }

    pub const fn device(&self) -> PortType {
        self.device
    }
}
