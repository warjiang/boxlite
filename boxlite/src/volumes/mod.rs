//! Volume management for guest and container layers.
//!
//! Provides volume configuration managers:
//! - `GuestVolumeManager` - Manages virtiofs shares and block devices for guest VM
//! - `ContainerVolumeManager` - Manages bind mounts for container namespace
//! - `BlockDeviceManager` - Legacy block device manager (consider using GuestVolumeManager)

mod block_device;
mod container_volume;
mod guest_volume;

pub use block_device::BlockDeviceManager;

#[allow(unused_imports)]
pub use container_volume::{ContainerMount, ContainerVolumeManager};
#[allow(unused_imports)]
pub use guest_volume::{BlockDeviceEntry, FsShareEntry, GuestVolumeManager, VmmMountConfig};
