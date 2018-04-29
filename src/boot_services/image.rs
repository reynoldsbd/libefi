use types::{
    Handle,
    Status,
};

use super::BootServices;


impl BootServices {

    pub fn exit_boot_services(&self, image_handle: Handle, map_key: usize) -> Result<(), Status> {

        (self._exit_boot_services)(image_handle, map_key)
            .as_result()
            .map(|_| ())
    }
}
