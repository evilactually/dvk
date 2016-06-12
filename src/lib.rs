pub type vkCreateInstanceFn = Option<unsafe extern "stdcall" fn(pCreateInfo: *const VkInstanceCreateInfo, 
                                                                pAllocator: *const VkAllocationCallbacks, 
                                                                pInstance: *mut VkInstance)>;

static mut vkCreateInstanceFn: vkCreateInstanceFn = None;

#[derive(Debug)]
//#[derive(Copy)]
#[repr(C)]
pub struct VkInstanceCreateInfo {
    pub a: u32
}

#[derive(Debug)]
//#[derive(Copy)]
#[repr(C)]
pub struct VkAllocationCallbacks {
    pub a: u32
}

#[derive(Debug)]
//#[derive(Copy)]
#[repr(C)]
pub struct VkInstance {
    pub a: u32
}

unsafe fn load_vulkan_core() {

}

unsafe fn load_vulkan_khr_surface() {

}

unsafe fn load_vulkan_khr_win32() {

}

pub unsafe fn vkCreateInstance(pCreateInfo: *const VkInstanceCreateInfo, 
                               pAllocator: *const VkAllocationCallbacks, 
                               pInstance: *mut VkInstance) {
    match vkCreateInstanceFn {
        Some(f) => {f(pCreateInfo, pAllocator, pInstance);},
        None => (),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
