#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate shared_library;

use self::libc::{c_void, c_char, uint32_t, size_t, uint64_t, c_float};
use self::shared_library::dynamic_library::DynamicLibrary;
use std::path::{Path};

pub type VkFlags = uint32_t;
pub type VkBool32 = uint32_t;
pub type VkDeviceSize = uint64_t;
pub type VkSampleMask = uint32_t;
pub type VkInstance = *const c_void;
pub type VkPhysicalDevice = *const c_void;
pub type VkDevice = *const c_void;
pub type VkQueue = *const c_void;
pub type VkSemaphore = uint64_t;
pub type VkCommandBuffer = *const c_void;
pub type VkFence = uint64_t;
pub type VkDeviceMemory = uint64_t;
pub type VkBuffer = uint64_t;
pub type VkImage = uint64_t;
pub type VkEvent = uint64_t;
pub type VkQueryPool = uint64_t;
pub type VkBufferView = uint64_t;
pub type VkImageView = uint64_t;
pub type VkShaderModule = uint64_t;
pub type VkPipelineCache = uint64_t;
pub type VkPipelineLayout = uint64_t;
pub type VkRenderPass = uint64_t;
pub type VkPipeline = uint64_t;
pub type VkDescriptorSetLayout = uint64_t;
pub type VkSampler = uint64_t;
pub type VkDescriptorPool = uint64_t;
pub type VkDescriptorSet = uint64_t;
pub type VkFramebuffer = uint64_t;
pub type VkCommandPool = uint64_t;

const VK_LOD_CLAMP_NONE:c_float = 1000.0f32;
const VK_REMAINING_MIP_LEVELS:uint32_t = 0xffffffffu32;
const VK_REMAINING_ARRAY_LAYERS:uint32_t = 0xffffffffu32;
const VK_WHOLE_SIZE:u64 = 0xffffffffffffffffu64;
const VK_ATTACHMENT_UNUSED:uint32_t = 0xffffffffu32;
const VK_TRUE:uint32_t = 1u32;
const VK_FALSE:uint32_t = 0u32;
const VK_QUEUE_FAMILY_IGNORED:uint32_t = 0xffffffffu32;
const VK_SUBPASS_EXTERNAL:uint32_t = 0xffffffffu32;
const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE:size_t = 256usize;
const VK_UUID_SIZE:size_t = 16usize;
const VK_MAX_MEMORY_TYPES:size_t = 32usize;
const VK_MAX_MEMORY_HEAPS:size_t = 16usize;
const VK_MAX_EXTENSION_NAME_SIZE:size_t = 256usize;
const VK_MAX_DESCRIPTION_SIZE:size_t = 256usize;

#[repr(u32)]
pub enum VkStructureType {
    VK_STRUCTURE_TYPE_APPLICATION_INFO = 0,
    VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO = 1,
    VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO = 2,
    VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO = 3,
    VK_STRUCTURE_TYPE_SUBMIT_INFO = 4,
    VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO = 5,
    VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE = 6,
    VK_STRUCTURE_TYPE_BIND_SPARSE_INFO = 7,
    VK_STRUCTURE_TYPE_FENCE_CREATE_INFO = 8,
    VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO = 9,
    VK_STRUCTURE_TYPE_EVENT_CREATE_INFO = 10,
    VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO = 11,
    VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO = 12,
    VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO = 13,
    VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO = 14,
    VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO = 15,
    VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO = 16,
    VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO = 17,
    VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO = 18,
    VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO = 19,
    VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO = 20,
    VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO = 21,
    VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO = 22,
    VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO = 23,
    VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO = 24,
    VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO = 25,
    VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO = 26,
    VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO = 27,
    VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO = 28,
    VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO = 29,
    VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO = 30,
    VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO = 31,
    VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO = 32,
    VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO = 33,
    VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO = 34,
    VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET = 35,
    VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET = 36,
    VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO = 37,
    VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO = 38,
    VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO = 39,
    VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO = 40,
    VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO = 41,
    VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO = 42,
    VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO = 43,
    VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER = 44,
    VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER = 45,
    VK_STRUCTURE_TYPE_MEMORY_BARRIER = 46,
    VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO = 47,
    VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO = 48,
    VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR = 1000001000,
    VK_STRUCTURE_TYPE_PRESENT_INFO_KHR = 1000001001,
    VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR = 1000002000,
    VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR = 1000002001,
    VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR = 1000003000,
    VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR = 1000004000,
    VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR = 1000005000,
    VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR = 1000006000,
    VK_STRUCTURE_TYPE_MIR_SURFACE_CREATE_INFO_KHR = 1000007000,
    VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR = 1000008000,
    VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR = 1000009000,
    VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT = 1000011000,
    VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD = 1000018000,
    VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT = 1000022000,
    VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT = 1000022001,
    VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT = 1000022002,
}

#[repr(u32)]
pub enum VkSystemAllocationScope {
    VK_SYSTEM_ALLOCATION_SCOPE_COMMAND = 0,
    VK_SYSTEM_ALLOCATION_SCOPE_OBJECT = 1,
    VK_SYSTEM_ALLOCATION_SCOPE_CACHE = 2,
    VK_SYSTEM_ALLOCATION_SCOPE_DEVICE = 3,
    VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE = 4
}

pub type VkInstanceCreateFlags = VkFlags;

#[repr(C)]
pub struct VkInstanceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkInstanceCreateFlags,
    pub pApplicationInfo: VkApplicationInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const c_char
}

#[repr(C)]
pub struct VkApplicationInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pApplicationName: *const c_char,
    pub applicationVersion: u32,
    pub pEngineName: *const c_char,
    pub engineVersion: u32,
    pub apiVersion: u32
}

#[repr(C)]
pub struct VkAllocationCallbacks {
    pub pUserData: *const c_void,
    pub pfnAllocation: vkAllocationFunctionFn,
    pub pfnReallocation: vkReallocationFunctionFn,
    pub pfnFree: vkFreeFunctionFn,
    pub pfnInternalAllocation: vkInternalAllocationNotificationFn,
    pub pfnInternalFree: vkInternalFreeNotificationFn
}

#[repr(C)]
pub enum VkInternalAllocationType {
    VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE = 0
}

pub type vkCreateInstanceFn = unsafe extern "stdcall" fn(pCreateInfo: *const VkInstanceCreateInfo, 
                                                         pAllocator: *const VkAllocationCallbacks,
                                                         pInstance: *mut VkInstance);

pub type vkAllocationFunctionFn = unsafe extern "stdcall" fn(pUserData: *mut c_void,
                                                             size: size_t,
                                                             alignment: size_t,
                                                             allocationScope: VkSystemAllocationScope);

pub type vkReallocationFunctionFn = unsafe extern "stdcall" fn(pUserData: *mut c_void,
                                                               pOriginal: *mut c_void,
                                                               size: size_t,
                                                               alignment: size_t,
                                                               allocationScope: VkSystemAllocationScope);

pub type vkFreeFunctionFn = unsafe extern "stdcall" fn(pUserData: *mut c_void,
                                                       pMemory: *mut c_void);

pub type vkInternalAllocationNotificationFn = unsafe extern "stdcall" fn(pUserData: *mut c_void,
                                                                         size: size_t,
                                                                         allocationType: VkInternalAllocationType,
                                                                         allocationScope: VkSystemAllocationScope);

pub type vkInternalFreeNotificationFn = unsafe extern "stdcall" fn(pUserData: *mut c_void,
                                                                   size: size_t,
                                                                   allocationType: VkInternalAllocationType,
                                                                   allocationScope: VkSystemAllocationScope);
/// Core vulkan commands
pub struct Vulkan {
    pub vkCreateInstance: vkCreateInstanceFn
}

impl Vulkan {
    /// Dynamically loads core vulkan commands
    pub fn load() -> Result<Vulkan, String> {
        let lib_name = Path::new("vulkan-1.dll");
        let lib = match DynamicLibrary::open(Some(&lib_name)) {
            Err(err) => return Err(format!("Failed to load {}: {}",lib_name.to_str().unwrap(),err)),
            Ok(lib) => lib,
        };
        unsafe {
            let cinf:VkInstanceCreateInfo = std::mem::zeroed();
            let mut inst:VkInstance = std::mem::zeroed();
            let a: *mut i8 = lib.symbol("vkCreateInstance").unwrap();
            let aa:vkCreateInstanceFn = std::mem::transmute(a);
            //aa(&cinf, std::ptr::null(), &mut inst);
            let n:u64 = std::mem::transmute(a);
            println!("{:?}", n);
        }
        Err("OMG".to_string())
    }
}

#[test]
fn load_test() {
    match Vulkan::load() {
        Err(err) => panic!(err),
        _ => ()
    }
}

