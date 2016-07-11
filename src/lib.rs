#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate shared_library;

#[cfg(windows)]
static VULKAN_LIBRARY: &'static str = "vulkan-1.dll";

#[cfg(unix)]
static VULKAN_LIBRARY: &'static str = "libvulkan-1.so";

/// A call to vkGetInstanceProcAddr wrapped in a try block, returns an error message or function pointer
macro_rules! load_command {
    ($commands:expr,$instance:expr,$name:expr) => (
        {
            let fn_ptr = ($commands.vkGetInstanceProcAddr.as_ref().unwrap())($instance, CString::new($name).unwrap().as_ptr());
            try!(
                if fn_ptr != ::std::ptr::null() {
                    Ok(fn_ptr)
                } else {
                    Err(format!("Failed to load {}",$name))
                })
        }
    );
}

/// Call to a stored command with error reporting for unloaded commands
macro_rules! invoke_command {
    ($commands:expr,$command:ident,$($x:ident),*) => {
        {
            if let Some($command) = $commands.$command.as_ref() {
                $command($($x,)*)
            } else {
                panic!(concat!("Command not loaded: ", stringify!($command)));
            }
        }
    }
}

/// Simplified variant of bitflags! for defining placeholder flags
macro_rules! reserved_bitflags {
    ($(#[$attr:meta])* pub flags $BitFlags:ident: $T:ty;) => {
        #[derive(Copy, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
        $(#[$attr])*
        pub struct $BitFlags {
            bits: $T,
        }

        impl $BitFlags {
            /// Returns an empty set of flags.
            #[inline]
            pub fn empty() -> $BitFlags {
                $BitFlags { bits: 0 }
            }
        }
    }
}

#[macro_use]
pub mod core {
    use ::libc::{c_void, c_char, uint32_t, size_t, uint64_t, c_float, int32_t, uint8_t};
    use ::shared_library::dynamic_library::DynamicLibrary;
    use ::std::path::{Path};
    use ::std::ffi::CString;
    use ::std::mem::transmute;
    use ::VULKAN_LIBRARY;

    #[macro_export]
    macro_rules! VK_MAKE_VERSION {
        ($major:expr, $minor:expr, $patch:expr) => ((($major) << 22) | (($minor) << 12) | ($patch));
    }

    pub const VK_API_VERSION_1_0: uint32_t = VK_MAKE_VERSION!(1,0,0);

    #[macro_export]
    macro_rules! VK_VERSION_MAJOR {
        ($version:expr) => ($version >> 22);
    }

    #[macro_export]
    macro_rules! VK_VERSION_MINOR {
        ($version:expr) => (($version >> 12) & 0x3ff);
    }

    #[macro_export]
    macro_rules! VK_VERSION_PATCH {
        ($version:expr) => ($version & 0xfff);
    }

    #[macro_export]
    macro_rules! VK_DEFINE_NON_DISPATCHABLE_HANDLE {
        ($name:ident) => (
            #[derive(Clone)] 
            #[derive(Copy)] 
            #[repr(C)]
            pub struct $name(uint64_t);
            impl $name {
                pub fn null() -> $name {
                    $name(0)
                }
                pub fn is_null(&self) -> bool {
                    self.0 == 0
                }
            }
        );
    }

    #[macro_export]
    macro_rules! VK_DEFINE_HANDLE {
        ($name:ident) => (
            #[derive(Clone)] 
            #[derive(Copy)] 
            #[repr(C)]
            pub struct $name(*const c_void);
            impl $name {
                pub fn null() -> $name {
                    $name(::std::ptr::null())
                }
                pub fn is_null(&self) -> bool {
                    self.0 == ::std::ptr::null()
                }
            }
        );
    }

    pub type VkFlags = uint32_t;
    pub type VkBool32 = uint32_t;
    pub type VkDeviceSize = uint64_t;
    pub type VkSampleMask = uint32_t;

    VK_DEFINE_HANDLE!(VkInstance);
    VK_DEFINE_HANDLE!(VkPhysicalDevice);
    VK_DEFINE_HANDLE!(VkDevice);
    VK_DEFINE_HANDLE!(VkQueue);
    VK_DEFINE_NON_DISPATCHABLE_HANDLE!(VkSemaphore);
    VK_DEFINE_HANDLE!(VkCommandBuffer);
    VK_DEFINE_NON_DISPATCHABLE_HANDLE!(VkFence);
    VK_DEFINE_NON_DISPATCHABLE_HANDLE!(VkDeviceMemory);
    VK_DEFINE_NON_DISPATCHABLE_HANDLE!(VkBuffer);
    VK_DEFINE_NON_DISPATCHABLE_HANDLE!(VkImage);
    VK_DEFINE_NON_DISPATCHABLE_HANDLE!(VkEvent);
    VK_DEFINE_NON_DISPATCHABLE_HANDLE!(VkQueryPool);
    VK_DEFINE_NON_DISPATCHABLE_HANDLE!(VkBufferView);
    VK_DEFINE_NON_DISPATCHABLE_HANDLE!(VkImageView);
    VK_DEFINE_NON_DISPATCHABLE_HANDLE!(VkShaderModule);
    VK_DEFINE_NON_DISPATCHABLE_HANDLE!(VkPipelineCache);
    VK_DEFINE_NON_DISPATCHABLE_HANDLE!(VkPipelineLayout);
    VK_DEFINE_NON_DISPATCHABLE_HANDLE!(VkRenderPass);
    VK_DEFINE_NON_DISPATCHABLE_HANDLE!(VkPipeline);
    VK_DEFINE_NON_DISPATCHABLE_HANDLE!(VkDescriptorSetLayout);
    VK_DEFINE_NON_DISPATCHABLE_HANDLE!(VkSampler);
    VK_DEFINE_NON_DISPATCHABLE_HANDLE!(VkDescriptorPool);
    VK_DEFINE_NON_DISPATCHABLE_HANDLE!(VkDescriptorSet);
    VK_DEFINE_NON_DISPATCHABLE_HANDLE!(VkFramebuffer);
    VK_DEFINE_NON_DISPATCHABLE_HANDLE!(VkCommandPool);

    pub const VK_LOD_CLAMP_NONE:c_float = 1000.0f32;
    pub const VK_REMAINING_MIP_LEVELS:uint32_t = !0x0u32;
    pub const VK_REMAINING_ARRAY_LAYERS:uint32_t = !0x0u32;
    pub const VK_WHOLE_SIZE:uint64_t = !0x0u64;
    pub const VK_ATTACHMENT_UNUSED:uint32_t = !0x0u32;
    pub const VK_TRUE:uint32_t = 1u32;
    pub const VK_FALSE:uint32_t = 0u32;
    pub const VK_QUEUE_FAMILY_IGNORED:uint32_t = !0x0u32;
    pub const VK_SUBPASS_EXTERNAL:uint32_t = !0x0u32;
    pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE:size_t = 256usize;
    pub const VK_UUID_SIZE:size_t = 16usize;
    pub const VK_MAX_MEMORY_TYPES:size_t = 32usize;
    pub const VK_MAX_MEMORY_HEAPS:size_t = 16usize;
    pub const VK_MAX_EXTENSION_NAME_SIZE:size_t = 256usize;
    pub const VK_MAX_DESCRIPTION_SIZE:size_t = 256usize;

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkPipelineCacheHeaderVersion {
        VK_PIPELINE_CACHE_HEADER_VERSION_ONE = 1
    }

    #[repr(i32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkResult {
        VK_SUCCESS = 0,
        VK_NOT_READY = 1,
        VK_TIMEOUT = 2,
        VK_EVENT_SET = 3,
        VK_EVENT_RESET = 4,
        VK_INCOMPLETE = 5,
        VK_ERROR_OUT_OF_HOST_MEMORY = -1,
        VK_ERROR_OUT_OF_DEVICE_MEMORY = -2,
        VK_ERROR_INITIALIZATION_FAILED = -3,
        VK_ERROR_DEVICE_LOST = -4,
        VK_ERROR_MEMORY_MAP_FAILED = -5,
        VK_ERROR_LAYER_NOT_PRESENT = -6,
        VK_ERROR_EXTENSION_NOT_PRESENT = -7,
        VK_ERROR_FEATURE_NOT_PRESENT = -8,
        VK_ERROR_INCOMPATIBLE_DRIVER = -9,
        VK_ERROR_TOO_MANY_OBJECTS = -10,
        VK_ERROR_FORMAT_NOT_SUPPORTED = -11,
        VK_ERROR_SURFACE_LOST_KHR = -1000000000,
        VK_ERROR_NATIVE_WINDOW_IN_USE_KHR = -1000000001,
        VK_SUBOPTIMAL_KHR = 1000001003,
        VK_ERROR_OUT_OF_DATE_KHR = -1000001004,
        VK_ERROR_INCOMPATIBLE_DISPLAY_KHR = -1000003001,
        VK_ERROR_VALIDATION_FAILED_EXT = -1000011001,
        VK_ERROR_INVALID_SHADER_NV = -1000012000
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
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
        VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT = 1000022002
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkSystemAllocationScope {
        VK_SYSTEM_ALLOCATION_SCOPE_COMMAND = 0,
        VK_SYSTEM_ALLOCATION_SCOPE_OBJECT = 1,
        VK_SYSTEM_ALLOCATION_SCOPE_CACHE = 2,
        VK_SYSTEM_ALLOCATION_SCOPE_DEVICE = 3,
        VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE = 4
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkInternalAllocationType {
        VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE = 0
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkFormat {
        VK_FORMAT_UNDEFINED = 0,
        VK_FORMAT_R4G4_UNORM_PACK8 = 1,
        VK_FORMAT_R4G4B4A4_UNORM_PACK16 = 2,
        VK_FORMAT_B4G4R4A4_UNORM_PACK16 = 3,
        VK_FORMAT_R5G6B5_UNORM_PACK16 = 4,
        VK_FORMAT_B5G6R5_UNORM_PACK16 = 5,
        VK_FORMAT_R5G5B5A1_UNORM_PACK16 = 6,
        VK_FORMAT_B5G5R5A1_UNORM_PACK16 = 7,
        VK_FORMAT_A1R5G5B5_UNORM_PACK16 = 8,
        VK_FORMAT_R8_UNORM = 9,
        VK_FORMAT_R8_SNORM = 10,
        VK_FORMAT_R8_USCALED = 11,
        VK_FORMAT_R8_SSCALED = 12,
        VK_FORMAT_R8_UINT = 13,
        VK_FORMAT_R8_SINT = 14,
        VK_FORMAT_R8_SRGB = 15,
        VK_FORMAT_R8G8_UNORM = 16,
        VK_FORMAT_R8G8_SNORM = 17,
        VK_FORMAT_R8G8_USCALED = 18,
        VK_FORMAT_R8G8_SSCALED = 19,
        VK_FORMAT_R8G8_UINT = 20,
        VK_FORMAT_R8G8_SINT = 21,
        VK_FORMAT_R8G8_SRGB = 22,
        VK_FORMAT_R8G8B8_UNORM = 23,
        VK_FORMAT_R8G8B8_SNORM = 24,
        VK_FORMAT_R8G8B8_USCALED = 25,
        VK_FORMAT_R8G8B8_SSCALED = 26,
        VK_FORMAT_R8G8B8_UINT = 27,
        VK_FORMAT_R8G8B8_SINT = 28,
        VK_FORMAT_R8G8B8_SRGB = 29,
        VK_FORMAT_B8G8R8_UNORM = 30,
        VK_FORMAT_B8G8R8_SNORM = 31,
        VK_FORMAT_B8G8R8_USCALED = 32,
        VK_FORMAT_B8G8R8_SSCALED = 33,
        VK_FORMAT_B8G8R8_UINT = 34,
        VK_FORMAT_B8G8R8_SINT = 35,
        VK_FORMAT_B8G8R8_SRGB = 36,
        VK_FORMAT_R8G8B8A8_UNORM = 37,
        VK_FORMAT_R8G8B8A8_SNORM = 38,
        VK_FORMAT_R8G8B8A8_USCALED = 39,
        VK_FORMAT_R8G8B8A8_SSCALED = 40,
        VK_FORMAT_R8G8B8A8_UINT = 41,
        VK_FORMAT_R8G8B8A8_SINT = 42,
        VK_FORMAT_R8G8B8A8_SRGB = 43,
        VK_FORMAT_B8G8R8A8_UNORM = 44,
        VK_FORMAT_B8G8R8A8_SNORM = 45,
        VK_FORMAT_B8G8R8A8_USCALED = 46,
        VK_FORMAT_B8G8R8A8_SSCALED = 47,
        VK_FORMAT_B8G8R8A8_UINT = 48,
        VK_FORMAT_B8G8R8A8_SINT = 49,
        VK_FORMAT_B8G8R8A8_SRGB = 50,
        VK_FORMAT_A8B8G8R8_UNORM_PACK32 = 51,
        VK_FORMAT_A8B8G8R8_SNORM_PACK32 = 52,
        VK_FORMAT_A8B8G8R8_USCALED_PACK32 = 53,
        VK_FORMAT_A8B8G8R8_SSCALED_PACK32 = 54,
        VK_FORMAT_A8B8G8R8_UINT_PACK32 = 55,
        VK_FORMAT_A8B8G8R8_SINT_PACK32 = 56,
        VK_FORMAT_A8B8G8R8_SRGB_PACK32 = 57,
        VK_FORMAT_A2R10G10B10_UNORM_PACK32 = 58,
        VK_FORMAT_A2R10G10B10_SNORM_PACK32 = 59,
        VK_FORMAT_A2R10G10B10_USCALED_PACK32 = 60,
        VK_FORMAT_A2R10G10B10_SSCALED_PACK32 = 61,
        VK_FORMAT_A2R10G10B10_UINT_PACK32 = 62,
        VK_FORMAT_A2R10G10B10_SINT_PACK32 = 63,
        VK_FORMAT_A2B10G10R10_UNORM_PACK32 = 64,
        VK_FORMAT_A2B10G10R10_SNORM_PACK32 = 65,
        VK_FORMAT_A2B10G10R10_USCALED_PACK32 = 66,
        VK_FORMAT_A2B10G10R10_SSCALED_PACK32 = 67,
        VK_FORMAT_A2B10G10R10_UINT_PACK32 = 68,
        VK_FORMAT_A2B10G10R10_SINT_PACK32 = 69,
        VK_FORMAT_R16_UNORM = 70,
        VK_FORMAT_R16_SNORM = 71,
        VK_FORMAT_R16_USCALED = 72,
        VK_FORMAT_R16_SSCALED = 73,
        VK_FORMAT_R16_UINT = 74,
        VK_FORMAT_R16_SINT = 75,
        VK_FORMAT_R16_SFLOAT = 76,
        VK_FORMAT_R16G16_UNORM = 77,
        VK_FORMAT_R16G16_SNORM = 78,
        VK_FORMAT_R16G16_USCALED = 79,
        VK_FORMAT_R16G16_SSCALED = 80,
        VK_FORMAT_R16G16_UINT = 81,
        VK_FORMAT_R16G16_SINT = 82,
        VK_FORMAT_R16G16_SFLOAT = 83,
        VK_FORMAT_R16G16B16_UNORM = 84,
        VK_FORMAT_R16G16B16_SNORM = 85,
        VK_FORMAT_R16G16B16_USCALED = 86,
        VK_FORMAT_R16G16B16_SSCALED = 87,
        VK_FORMAT_R16G16B16_UINT = 88,
        VK_FORMAT_R16G16B16_SINT = 89,
        VK_FORMAT_R16G16B16_SFLOAT = 90,
        VK_FORMAT_R16G16B16A16_UNORM = 91,
        VK_FORMAT_R16G16B16A16_SNORM = 92,
        VK_FORMAT_R16G16B16A16_USCALED = 93,
        VK_FORMAT_R16G16B16A16_SSCALED = 94,
        VK_FORMAT_R16G16B16A16_UINT = 95,
        VK_FORMAT_R16G16B16A16_SINT = 96,
        VK_FORMAT_R16G16B16A16_SFLOAT = 97,
        VK_FORMAT_R32_UINT = 98,
        VK_FORMAT_R32_SINT = 99,
        VK_FORMAT_R32_SFLOAT = 100,
        VK_FORMAT_R32G32_UINT = 101,
        VK_FORMAT_R32G32_SINT = 102,
        VK_FORMAT_R32G32_SFLOAT = 103,
        VK_FORMAT_R32G32B32_UINT = 104,
        VK_FORMAT_R32G32B32_SINT = 105,
        VK_FORMAT_R32G32B32_SFLOAT = 106,
        VK_FORMAT_R32G32B32A32_UINT = 107,
        VK_FORMAT_R32G32B32A32_SINT = 108,
        VK_FORMAT_R32G32B32A32_SFLOAT = 109,
        VK_FORMAT_R64_UINT = 110,
        VK_FORMAT_R64_SINT = 111,
        VK_FORMAT_R64_SFLOAT = 112,
        VK_FORMAT_R64G64_UINT = 113,
        VK_FORMAT_R64G64_SINT = 114,
        VK_FORMAT_R64G64_SFLOAT = 115,
        VK_FORMAT_R64G64B64_UINT = 116,
        VK_FORMAT_R64G64B64_SINT = 117,
        VK_FORMAT_R64G64B64_SFLOAT = 118,
        VK_FORMAT_R64G64B64A64_UINT = 119,
        VK_FORMAT_R64G64B64A64_SINT = 120,
        VK_FORMAT_R64G64B64A64_SFLOAT = 121,
        VK_FORMAT_B10G11R11_UFLOAT_PACK32 = 122,
        VK_FORMAT_E5B9G9R9_UFLOAT_PACK32 = 123,
        VK_FORMAT_D16_UNORM = 124,
        VK_FORMAT_X8_D24_UNORM_PACK32 = 125,
        VK_FORMAT_D32_SFLOAT = 126,
        VK_FORMAT_S8_UINT = 127,
        VK_FORMAT_D16_UNORM_S8_UINT = 128,
        VK_FORMAT_D24_UNORM_S8_UINT = 129,
        VK_FORMAT_D32_SFLOAT_S8_UINT = 130,
        VK_FORMAT_BC1_RGB_UNORM_BLOCK = 131,
        VK_FORMAT_BC1_RGB_SRGB_BLOCK = 132,
        VK_FORMAT_BC1_RGBA_UNORM_BLOCK = 133,
        VK_FORMAT_BC1_RGBA_SRGB_BLOCK = 134,
        VK_FORMAT_BC2_UNORM_BLOCK = 135,
        VK_FORMAT_BC2_SRGB_BLOCK = 136,
        VK_FORMAT_BC3_UNORM_BLOCK = 137,
        VK_FORMAT_BC3_SRGB_BLOCK = 138,
        VK_FORMAT_BC4_UNORM_BLOCK = 139,
        VK_FORMAT_BC4_SNORM_BLOCK = 140,
        VK_FORMAT_BC5_UNORM_BLOCK = 141,
        VK_FORMAT_BC5_SNORM_BLOCK = 142,
        VK_FORMAT_BC6H_UFLOAT_BLOCK = 143,
        VK_FORMAT_BC6H_SFLOAT_BLOCK = 144,
        VK_FORMAT_BC7_UNORM_BLOCK = 145,
        VK_FORMAT_BC7_SRGB_BLOCK = 146,
        VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK = 147,
        VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK = 148,
        VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK = 149,
        VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK = 150,
        VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK = 151,
        VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK = 152,
        VK_FORMAT_EAC_R11_UNORM_BLOCK = 153,
        VK_FORMAT_EAC_R11_SNORM_BLOCK = 154,
        VK_FORMAT_EAC_R11G11_UNORM_BLOCK = 155,
        VK_FORMAT_EAC_R11G11_SNORM_BLOCK = 156,
        VK_FORMAT_ASTC_4x4_UNORM_BLOCK = 157,
        VK_FORMAT_ASTC_4x4_SRGB_BLOCK = 158,
        VK_FORMAT_ASTC_5x4_UNORM_BLOCK = 159,
        VK_FORMAT_ASTC_5x4_SRGB_BLOCK = 160,
        VK_FORMAT_ASTC_5x5_UNORM_BLOCK = 161,
        VK_FORMAT_ASTC_5x5_SRGB_BLOCK = 162,
        VK_FORMAT_ASTC_6x5_UNORM_BLOCK = 163,
        VK_FORMAT_ASTC_6x5_SRGB_BLOCK = 164,
        VK_FORMAT_ASTC_6x6_UNORM_BLOCK = 165,
        VK_FORMAT_ASTC_6x6_SRGB_BLOCK = 166,
        VK_FORMAT_ASTC_8x5_UNORM_BLOCK = 167,
        VK_FORMAT_ASTC_8x5_SRGB_BLOCK = 168,
        VK_FORMAT_ASTC_8x6_UNORM_BLOCK = 169,
        VK_FORMAT_ASTC_8x6_SRGB_BLOCK = 170,
        VK_FORMAT_ASTC_8x8_UNORM_BLOCK = 171,
        VK_FORMAT_ASTC_8x8_SRGB_BLOCK = 172,
        VK_FORMAT_ASTC_10x5_UNORM_BLOCK = 173,
        VK_FORMAT_ASTC_10x5_SRGB_BLOCK = 174,
        VK_FORMAT_ASTC_10x6_UNORM_BLOCK = 175,
        VK_FORMAT_ASTC_10x6_SRGB_BLOCK = 176,
        VK_FORMAT_ASTC_10x8_UNORM_BLOCK = 177,
        VK_FORMAT_ASTC_10x8_SRGB_BLOCK = 178,
        VK_FORMAT_ASTC_10x10_UNORM_BLOCK = 179,
        VK_FORMAT_ASTC_10x10_SRGB_BLOCK = 180,
        VK_FORMAT_ASTC_12x10_UNORM_BLOCK = 181,
        VK_FORMAT_ASTC_12x10_SRGB_BLOCK = 182,
        VK_FORMAT_ASTC_12x12_UNORM_BLOCK = 183,
        VK_FORMAT_ASTC_12x12_SRGB_BLOCK = 184
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkImageType {
        VK_IMAGE_TYPE_1D = 0,
        VK_IMAGE_TYPE_2D = 1,
        VK_IMAGE_TYPE_3D = 2
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkImageTiling {
        VK_IMAGE_TILING_OPTIMAL = 0,
        VK_IMAGE_TILING_LINEAR = 1
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkPhysicalDeviceType {
        VK_PHYSICAL_DEVICE_TYPE_OTHER = 0,
        VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU = 1,
        VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU = 2,
        VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU = 3,
        VK_PHYSICAL_DEVICE_TYPE_CPU = 4
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkQueryType {
        VK_QUERY_TYPE_OCCLUSION = 0,
        VK_QUERY_TYPE_PIPELINE_STATISTICS = 1,
        VK_QUERY_TYPE_TIMESTAMP = 2
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkSharingMode {
        VK_SHARING_MODE_EXCLUSIVE = 0,
        VK_SHARING_MODE_CONCURRENT = 1
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkImageLayout {
        VK_IMAGE_LAYOUT_UNDEFINED = 0,
        VK_IMAGE_LAYOUT_GENERAL = 1,
        VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL = 2,
        VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL = 3,
        VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL = 4,
        VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL = 5,
        VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL = 6,
        VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL = 7,
        VK_IMAGE_LAYOUT_PREINITIALIZED = 8,
        VK_IMAGE_LAYOUT_PRESENT_SRC_KHR = 1000001002
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkImageViewType {
        VK_IMAGE_VIEW_TYPE_1D = 0,
        VK_IMAGE_VIEW_TYPE_2D = 1,
        VK_IMAGE_VIEW_TYPE_3D = 2,
        VK_IMAGE_VIEW_TYPE_CUBE = 3,
        VK_IMAGE_VIEW_TYPE_1D_ARRAY = 4,
        VK_IMAGE_VIEW_TYPE_2D_ARRAY = 5,
        VK_IMAGE_VIEW_TYPE_CUBE_ARRAY = 6
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkComponentSwizzle {
        VK_COMPONENT_SWIZZLE_IDENTITY = 0,
        VK_COMPONENT_SWIZZLE_ZERO = 1,
        VK_COMPONENT_SWIZZLE_ONE = 2,
        VK_COMPONENT_SWIZZLE_R = 3,
        VK_COMPONENT_SWIZZLE_G = 4,
        VK_COMPONENT_SWIZZLE_B = 5,
        VK_COMPONENT_SWIZZLE_A = 6
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkVertexInputRate {
        VK_VERTEX_INPUT_RATE_VERTEX = 0,
        VK_VERTEX_INPUT_RATE_INSTANCE = 1
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkPrimitiveTopology {
        VK_PRIMITIVE_TOPOLOGY_POINT_LIST = 0,
        VK_PRIMITIVE_TOPOLOGY_LINE_LIST = 1,
        VK_PRIMITIVE_TOPOLOGY_LINE_STRIP = 2,
        VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST = 3,
        VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP = 4,
        VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN = 5,
        VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY = 6,
        VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY = 7,
        VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY = 8,
        VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY = 9,
        VK_PRIMITIVE_TOPOLOGY_PATCH_LIST = 10
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkPolygonMode {
        VK_POLYGON_MODE_FILL = 0,
        VK_POLYGON_MODE_LINE = 1,
        VK_POLYGON_MODE_POINT = 2
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkFrontFace {
        VK_FRONT_FACE_COUNTER_CLOCKWISE = 0,
        VK_FRONT_FACE_CLOCKWISE = 1
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkCompareOp {
        VK_COMPARE_OP_NEVER = 0,
        VK_COMPARE_OP_LESS = 1,
        VK_COMPARE_OP_EQUAL = 2,
        VK_COMPARE_OP_LESS_OR_EQUAL = 3,
        VK_COMPARE_OP_GREATER = 4,
        VK_COMPARE_OP_NOT_EQUAL = 5,
        VK_COMPARE_OP_GREATER_OR_EQUAL = 6,
        VK_COMPARE_OP_ALWAYS = 7
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkStencilOp {
        VK_STENCIL_OP_KEEP = 0,
        VK_STENCIL_OP_ZERO = 1,
        VK_STENCIL_OP_REPLACE = 2,
        VK_STENCIL_OP_INCREMENT_AND_CLAMP = 3,
        VK_STENCIL_OP_DECREMENT_AND_CLAMP = 4,
        VK_STENCIL_OP_INVERT = 5,
        VK_STENCIL_OP_INCREMENT_AND_WRAP = 6,
        VK_STENCIL_OP_DECREMENT_AND_WRAP = 7
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkLogicOp {
        VK_LOGIC_OP_CLEAR = 0,
        VK_LOGIC_OP_AND = 1,
        VK_LOGIC_OP_AND_REVERSE = 2,
        VK_LOGIC_OP_COPY = 3,
        VK_LOGIC_OP_AND_INVERTED = 4,
        VK_LOGIC_OP_NO_OP = 5,
        VK_LOGIC_OP_XOR = 6,
        VK_LOGIC_OP_OR = 7,
        VK_LOGIC_OP_NOR = 8,
        VK_LOGIC_OP_EQUIVALENT = 9,
        VK_LOGIC_OP_INVERT = 10,
        VK_LOGIC_OP_OR_REVERSE = 11,
        VK_LOGIC_OP_COPY_INVERTED = 12,
        VK_LOGIC_OP_OR_INVERTED = 13,
        VK_LOGIC_OP_NAND = 14,
        VK_LOGIC_OP_SET = 15
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkBlendFactor {
        VK_BLEND_FACTOR_ZERO = 0,
        VK_BLEND_FACTOR_ONE = 1,
        VK_BLEND_FACTOR_SRC_COLOR = 2,
        VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR = 3,
        VK_BLEND_FACTOR_DST_COLOR = 4,
        VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR = 5,
        VK_BLEND_FACTOR_SRC_ALPHA = 6,
        VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA = 7,
        VK_BLEND_FACTOR_DST_ALPHA = 8,
        VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA = 9,
        VK_BLEND_FACTOR_CONSTANT_COLOR = 10,
        VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR = 11,
        VK_BLEND_FACTOR_CONSTANT_ALPHA = 12,
        VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA = 13,
        VK_BLEND_FACTOR_SRC_ALPHA_SATURATE = 14,
        VK_BLEND_FACTOR_SRC1_COLOR = 15,
        VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR = 16,
        VK_BLEND_FACTOR_SRC1_ALPHA = 17,
        VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA = 18
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkBlendOp {
        VK_BLEND_OP_ADD = 0,
        VK_BLEND_OP_SUBTRACT = 1,
        VK_BLEND_OP_REVERSE_SUBTRACT = 2,
        VK_BLEND_OP_MIN = 3,
        VK_BLEND_OP_MAX = 4
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkDynamicState {
        VK_DYNAMIC_STATE_VIEWPORT = 0,
        VK_DYNAMIC_STATE_SCISSOR = 1,
        VK_DYNAMIC_STATE_LINE_WIDTH = 2,
        VK_DYNAMIC_STATE_DEPTH_BIAS = 3,
        VK_DYNAMIC_STATE_BLEND_CONSTANTS = 4,
        VK_DYNAMIC_STATE_DEPTH_BOUNDS = 5,
        VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK = 6,
        VK_DYNAMIC_STATE_STENCIL_WRITE_MASK = 7,
        VK_DYNAMIC_STATE_STENCIL_REFERENCE = 8
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkFilter {
        VK_FILTER_NEAREST = 0,
        VK_FILTER_LINEAR = 1,
        VK_FILTER_CUBIC_IMG = 1000015000
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkSamplerMipmapMode {
        VK_SAMPLER_MIPMAP_MODE_NEAREST = 0,
        VK_SAMPLER_MIPMAP_MODE_LINEAR = 1
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkSamplerAddressMode {
        VK_SAMPLER_ADDRESS_MODE_REPEAT = 0,
        VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT = 1,
        VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE = 2,
        VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER = 3,
        VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE = 4
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkBorderColor {
        VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK = 0,
        VK_BORDER_COLOR_INT_TRANSPARENT_BLACK = 1,
        VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK = 2,
        VK_BORDER_COLOR_INT_OPAQUE_BLACK = 3,
        VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE = 4,
        VK_BORDER_COLOR_INT_OPAQUE_WHITE = 5
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkDescriptorType {
        VK_DESCRIPTOR_TYPE_SAMPLER = 0,
        VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER = 1,
        VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE = 2,
        VK_DESCRIPTOR_TYPE_STORAGE_IMAGE = 3,
        VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER = 4,
        VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER = 5,
        VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER = 6,
        VK_DESCRIPTOR_TYPE_STORAGE_BUFFER = 7,
        VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC = 8,
        VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC = 9,
        VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT = 10
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkAttachmentLoadOp {
        VK_ATTACHMENT_LOAD_OP_LOAD = 0,
        VK_ATTACHMENT_LOAD_OP_CLEAR = 1,
        VK_ATTACHMENT_LOAD_OP_DONT_CARE = 2
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkAttachmentStoreOp {
        VK_ATTACHMENT_STORE_OP_STORE = 0,
        VK_ATTACHMENT_STORE_OP_DONT_CARE = 1
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkPipelineBindPoint {
        VK_PIPELINE_BIND_POINT_GRAPHICS = 0,
        VK_PIPELINE_BIND_POINT_COMPUTE = 1
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkCommandBufferLevel {
        VK_COMMAND_BUFFER_LEVEL_PRIMARY = 0,
        VK_COMMAND_BUFFER_LEVEL_SECONDARY = 1
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkIndexType {
        VK_INDEX_TYPE_UINT16 = 0,
        VK_INDEX_TYPE_UINT32 = 1
    }

    #[repr(u32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkSubpassContents {
        VK_SUBPASS_CONTENTS_INLINE = 0,
        VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS = 1
    }

    reserved_bitflags! { 
        pub flags VkInstanceCreateFlags: VkFlags;
    }

    bitflags! {
        pub flags VkFormatFeatureFlags: VkFlags {
            const VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT = 0x00000001,
            const VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT = 0x00000002,
            const VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT = 0x00000004,
            const VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT = 0x00000008,
            const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT = 0x00000010,
            const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT = 0x00000020,
            const VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT = 0x00000040,
            const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT = 0x00000080,
            const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT = 0x00000100,
            const VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT = 0x00000200,
            const VK_FORMAT_FEATURE_BLIT_SRC_BIT = 0x00000400,
            const VK_FORMAT_FEATURE_BLIT_DST_BIT = 0x00000800,
            const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT = 0x00001000,
            const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG = 0x00002000
        }
    }

    bitflags! {
        pub flags VkImageUsageFlags: VkFlags {
            const VK_IMAGE_USAGE_TRANSFER_SRC_BIT = 0x00000001,
            const VK_IMAGE_USAGE_TRANSFER_DST_BIT = 0x00000002,
            const VK_IMAGE_USAGE_SAMPLED_BIT = 0x00000004,
            const VK_IMAGE_USAGE_STORAGE_BIT = 0x00000008,
            const VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT = 0x00000010,
            const VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT = 0x00000020,
            const VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT = 0x00000040,
            const VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT = 0x00000080
        }
    }

    bitflags! {
        pub flags VkImageCreateFlags: VkFlags {
            const VK_IMAGE_CREATE_SPARSE_BINDING_BIT = 0x00000001,
            const VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT = 0x00000002,
            const VK_IMAGE_CREATE_SPARSE_ALIASED_BIT = 0x00000004,
            const VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT = 0x00000008,
            const VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT = 0x00000010
        }
    }

    bitflags! {
        pub flags VkSampleCountFlags: VkFlags {
            const VK_SAMPLE_COUNT_1_BIT = 0x00000001,
            const VK_SAMPLE_COUNT_2_BIT = 0x00000002,
            const VK_SAMPLE_COUNT_4_BIT = 0x00000004,
            const VK_SAMPLE_COUNT_8_BIT = 0x00000008,
            const VK_SAMPLE_COUNT_16_BIT = 0x00000010,
            const VK_SAMPLE_COUNT_32_BIT = 0x00000020,
            const VK_SAMPLE_COUNT_64_BIT = 0x00000040
        }
    }

    bitflags! {
        pub flags VkQueueFlags: VkFlags {
            const VK_QUEUE_GRAPHICS_BIT = 0x00000001,
            const VK_QUEUE_COMPUTE_BIT = 0x00000002,
            const VK_QUEUE_TRANSFER_BIT = 0x00000004,
            const VK_QUEUE_SPARSE_BINDING_BIT = 0x00000008
        }
    }

    bitflags! {
        pub flags VkMemoryPropertyFlags: VkFlags {
            const VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT = 0x00000001,
            const VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT = 0x00000002,
            const VK_MEMORY_PROPERTY_HOST_COHERENT_BIT = 0x00000004,
            const VK_MEMORY_PROPERTY_HOST_CACHED_BIT = 0x00000008,
            const VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT = 0x00000010
        }
    }

    bitflags! {
        pub flags VkMemoryHeapFlags: VkFlags {
            const VK_MEMORY_HEAP_DEVICE_LOCAL_BIT = 0x00000001
        }
    }

    reserved_bitflags! {
        pub flags VkDeviceCreateFlags: VkFlags;
    }

    reserved_bitflags! { 
        pub flags VkDeviceQueueCreateFlags: VkFlags;
    }

    bitflags! {
        pub flags VkPipelineStageFlags: VkFlags {
            const VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT = 0x00000001,
            const VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT = 0x00000002,
            const VK_PIPELINE_STAGE_VERTEX_INPUT_BIT = 0x00000004,
            const VK_PIPELINE_STAGE_VERTEX_SHADER_BIT = 0x00000008,
            const VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT = 0x00000010,
            const VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT = 0x00000020,
            const VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT = 0x00000040,
            const VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT = 0x00000080,
            const VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT = 0x00000100,
            const VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT = 0x00000200,
            const VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT = 0x00000400,
            const VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT = 0x00000800,
            const VK_PIPELINE_STAGE_TRANSFER_BIT = 0x00001000,
            const VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT = 0x00002000,
            const VK_PIPELINE_STAGE_HOST_BIT = 0x00004000,
            const VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT = 0x00008000,
            const VK_PIPELINE_STAGE_ALL_COMMANDS_BIT = 0x00010000
        }
    }

    reserved_bitflags! { 
        pub flags VkMemoryMapFlags: VkFlags;
    }

    bitflags! {
        pub flags VkImageAspectFlags: VkFlags {
            const VK_IMAGE_ASPECT_COLOR_BIT = 0x00000001,
            const VK_IMAGE_ASPECT_DEPTH_BIT = 0x00000002,
            const VK_IMAGE_ASPECT_STENCIL_BIT = 0x00000004,
            const VK_IMAGE_ASPECT_METADATA_BIT = 0x00000008
        }
    }

    bitflags! {
        pub flags VkSparseImageFormatFlags: VkFlags {
            const VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT = 0x00000001,
            const VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT = 0x00000002,
            const VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT = 0x00000004
        }
    }

    bitflags! {
        pub flags VkSparseMemoryBindFlags: VkFlags {
            const VK_SPARSE_MEMORY_BIND_METADATA_BIT = 0x00000001
        }
    }

    bitflags! {
        pub flags VkFenceCreateFlags: VkFlags {
            const VK_FENCE_CREATE_SIGNALED_BIT = 0x00000001
        }
    }

    reserved_bitflags! { 
        pub flags VkSemaphoreCreateFlags: VkFlags;
    }
    reserved_bitflags! { 
        pub flags VkEventCreateFlags: VkFlags;
    }
    reserved_bitflags! { 
        pub flags VkQueryPoolCreateFlags: VkFlags;
    }

    bitflags! {
        pub flags VkQueryPipelineStatisticFlags: VkFlags {
            const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT = 0x00000001,
            const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT = 0x00000002,
            const VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT = 0x00000004,
            const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT = 0x00000008,
            const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT = 0x00000010,
            const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT = 0x00000020,
            const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT = 0x00000040,
            const VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT = 0x00000080,
            const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT = 0x00000100,
            const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT = 0x00000200,
            const VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT = 0x00000400
        }
    }

    bitflags! {
        pub flags VkQueryResultFlags: VkFlags {
            const VK_QUERY_RESULT_64_BIT = 0x00000001,
            const VK_QUERY_RESULT_WAIT_BIT = 0x00000002,
            const VK_QUERY_RESULT_WITH_AVAILABILITY_BIT = 0x00000004,
            const VK_QUERY_RESULT_PARTIAL_BIT = 0x00000008
        }
    }

    bitflags! {
        pub flags VkBufferCreateFlags: VkFlags {
            const VK_BUFFER_CREATE_SPARSE_BINDING_BIT = 0x00000001,
            const VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT = 0x00000002,
            const VK_BUFFER_CREATE_SPARSE_ALIASED_BIT = 0x00000004
        }
    }

    bitflags! {
        pub flags VkBufferUsageFlags: VkFlags {
            const VK_BUFFER_USAGE_TRANSFER_SRC_BIT = 0x00000001,
            const VK_BUFFER_USAGE_TRANSFER_DST_BIT = 0x00000002,
            const VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT = 0x00000004,
            const VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT = 0x00000008,
            const VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT = 0x00000010,
            const VK_BUFFER_USAGE_STORAGE_BUFFER_BIT = 0x00000020,
            const VK_BUFFER_USAGE_INDEX_BUFFER_BIT = 0x00000040,
            const VK_BUFFER_USAGE_VERTEX_BUFFER_BIT = 0x00000080,
            const VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT = 0x00000100
        }
    }

    reserved_bitflags! { 
        pub flags VkBufferViewCreateFlags: VkFlags;
    }
    reserved_bitflags! { 
        pub flags VkImageViewCreateFlags: VkFlags;
    }
    reserved_bitflags! { 
        pub flags VkShaderModuleCreateFlags: VkFlags;
    }
    reserved_bitflags! { 
        pub flags VkPipelineCacheCreateFlags: VkFlags;
    }

    bitflags! {
        pub flags VkPipelineCreateFlags: VkFlags {
            const VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT = 0x00000001,
            const VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT = 0x00000002,
            const VK_PIPELINE_CREATE_DERIVATIVE_BIT = 0x00000004
        }
    }

    reserved_bitflags! { 
        pub flags VkPipelineShaderStageCreateFlags: VkFlags;
    }

    bitflags! {
        pub flags VkShaderStageFlags: VkFlags {
            const VK_SHADER_STAGE_VERTEX_BIT = 0x00000001,
            const VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT = 0x00000002,
            const VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT = 0x00000004,
            const VK_SHADER_STAGE_GEOMETRY_BIT = 0x00000008,
            const VK_SHADER_STAGE_FRAGMENT_BIT = 0x00000010,
            const VK_SHADER_STAGE_COMPUTE_BIT = 0x00000020,
            const VK_SHADER_STAGE_ALL_GRAPHICS = 0x0000001F,
            const VK_SHADER_STAGE_ALL = 0x7FFFFFFF
        }
    }

    reserved_bitflags! { 
        pub flags VkPipelineVertexInputStateCreateFlags: VkFlags;
    }
    reserved_bitflags! { 
        pub flags VkPipelineInputAssemblyStateCreateFlags: VkFlags;
    }
    reserved_bitflags! { 
        pub flags VkPipelineTessellationStateCreateFlags: VkFlags;
    }
    reserved_bitflags! { 
        pub flags VkPipelineViewportStateCreateFlags: VkFlags;
    }
    reserved_bitflags! { 
        pub flags VkPipelineRasterizationStateCreateFlags: VkFlags;
    }

    bitflags! {
        pub flags VkCullModeFlags: VkFlags {
            const VK_CULL_MODE_NONE = 0,
            const VK_CULL_MODE_FRONT_BIT = 0x00000001,
            const VK_CULL_MODE_BACK_BIT = 0x00000002,
            const VK_CULL_MODE_FRONT_AND_BACK = 0x00000003
        }
    }

    reserved_bitflags! { 
        pub flags VkPipelineMultisampleStateCreateFlags: VkFlags;
    }
    reserved_bitflags! { 
        pub flags VkPipelineDepthStencilStateCreateFlags: VkFlags;
    }
    reserved_bitflags! { 
        pub flags VkPipelineColorBlendStateCreateFlags: VkFlags;
    }

    bitflags! {
        pub flags VkColorComponentFlags: VkFlags {
            const VK_COLOR_COMPONENT_R_BIT = 0x00000001,
            const VK_COLOR_COMPONENT_G_BIT = 0x00000002,
            const VK_COLOR_COMPONENT_B_BIT = 0x00000004,
            const VK_COLOR_COMPONENT_A_BIT = 0x00000008
        }
    }

    reserved_bitflags! { 
        pub flags VkPipelineDynamicStateCreateFlags: VkFlags;
    }
    reserved_bitflags! { 
        pub flags VkPipelineLayoutCreateFlags: VkFlags;
    }
    reserved_bitflags! { 
        pub flags VkSamplerCreateFlags: VkFlags;
    }
    reserved_bitflags! { 
        pub flags VkDescriptorSetLayoutCreateFlags: VkFlags;
    }

    bitflags! {
        pub flags VkDescriptorPoolCreateFlags: VkFlags {
            const VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT = 0x00000001
        }
    }

    reserved_bitflags! { 
        pub flags VkDescriptorPoolResetFlags: VkFlags;
    }
    reserved_bitflags! { 
        pub flags VkFramebufferCreateFlags: VkFlags;
    }
    reserved_bitflags! { 
        pub flags VkRenderPassCreateFlags: VkFlags;
    }

    bitflags! {
        pub flags VkAttachmentDescriptionFlags: VkFlags {
            const VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT = 0x00000001
        }
    }

    reserved_bitflags! { 
        pub flags VkSubpassDescriptionFlags: VkFlags;
    }

    bitflags! {
        pub flags VkAccessFlags: VkFlags {
            const VK_ACCESS_INDIRECT_COMMAND_READ_BIT = 0x00000001,
            const VK_ACCESS_INDEX_READ_BIT = 0x00000002,
            const VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT = 0x00000004,
            const VK_ACCESS_UNIFORM_READ_BIT = 0x00000008,
            const VK_ACCESS_INPUT_ATTACHMENT_READ_BIT = 0x00000010,
            const VK_ACCESS_SHADER_READ_BIT = 0x00000020,
            const VK_ACCESS_SHADER_WRITE_BIT = 0x00000040,
            const VK_ACCESS_COLOR_ATTACHMENT_READ_BIT = 0x00000080,
            const VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT = 0x00000100,
            const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT = 0x00000200,
            const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = 0x00000400,
            const VK_ACCESS_TRANSFER_READ_BIT = 0x00000800,
            const VK_ACCESS_TRANSFER_WRITE_BIT = 0x00001000,
            const VK_ACCESS_HOST_READ_BIT = 0x00002000,
            const VK_ACCESS_HOST_WRITE_BIT = 0x00004000,
            const VK_ACCESS_MEMORY_READ_BIT = 0x00008000,
            const VK_ACCESS_MEMORY_WRITE_BIT = 0x00010000
        }
    }

    bitflags! {
        pub flags VkDependencyFlags: VkFlags {
            const VK_DEPENDENCY_BY_REGION_BIT = 0x00000001
        }
    }

    bitflags! {
        pub flags VkCommandPoolCreateFlags: VkFlags {
            const VK_COMMAND_POOL_CREATE_TRANSIENT_BIT = 0x00000001,
            const VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT = 0x00000002
        }
    }

    bitflags! {
        pub flags VkCommandPoolResetFlags: VkFlags {
            const VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT = 0x00000001
        }
    }

    bitflags! {
        pub flags VkCommandBufferUsageFlags: VkFlags {
            const VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT = 0x00000001,
            const VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT = 0x00000002,
            const VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT = 0x00000004
        }
    }

    bitflags! {
        pub flags VkQueryControlFlags: VkFlags {
            const VK_QUERY_CONTROL_PRECISE_BIT = 0x00000001
        }
    }

    bitflags! {
        pub flags VkCommandBufferResetFlags: VkFlags {
            const VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT = 0x00000001
        }
    }

    bitflags! {
        pub flags VkStencilFaceFlags: VkFlags {
            const VK_STENCIL_FACE_FRONT_BIT = 0x00000001,
            const VK_STENCIL_FACE_BACK_BIT = 0x00000002,
            const VK_STENCIL_FRONT_AND_BACK = 0x00000003
        }
    }

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

    pub type vkVoidFunctionFn = *const u8;

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
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
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkInstanceCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkInstanceCreateFlags,
        pub pApplicationInfo: *const VkApplicationInfo,
        pub enabledLayerCount: u32,
        pub ppEnabledLayerNames: *const *const c_char,
        pub enabledExtensionCount: u32,
        pub ppEnabledExtensionNames: *const *const c_char
    }

    #[repr(C)]
    #[derive(Copy)]
    pub struct VkAllocationCallbacks {
        pub pUserData: *const c_void,
        pub pfnAllocation: Option<vkAllocationFunctionFn>,
        pub pfnReallocation: Option<vkReallocationFunctionFn>,
        pub pfnFree: Option<vkFreeFunctionFn>,
        pub pfnInternalAllocation: Option<vkInternalAllocationNotificationFn>,
        pub pfnInternalFree: Option<vkInternalFreeNotificationFn>
    }

    // Due to Rust issue #24000
    impl Clone for VkAllocationCallbacks {
        fn clone(&self) -> Self {
            unsafe {
                ::std::mem::transmute_copy(self)
            }
        }
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkPhysicalDeviceFeatures {
        pub robustBufferAccess: VkBool32,
        pub fullDrawIndexUint32: VkBool32,
        pub imageCubeArray: VkBool32,
        pub independentBlend: VkBool32,
        pub geometryShader: VkBool32,
        pub tessellationShader: VkBool32,
        pub sampleRateShading: VkBool32,
        pub dualSrcBlend: VkBool32,
        pub logicOp: VkBool32,
        pub multiDrawIndirect: VkBool32,
        pub drawIndirectFirstInstance: VkBool32,
        pub depthClamp: VkBool32,
        pub depthBiasClamp: VkBool32,
        pub fillModeNonSolid: VkBool32,
        pub depthBounds: VkBool32,
        pub wideLines: VkBool32,
        pub largePoints: VkBool32,
        pub alphaToOne: VkBool32,
        pub multiViewport: VkBool32,
        pub samplerAnisotropy: VkBool32,
        pub textureCompressionETC2: VkBool32,
        pub textureCompressionASTC_LDR: VkBool32,
        pub textureCompressionBC: VkBool32,
        pub occlusionQueryPrecise: VkBool32,
        pub pipelineStatisticsQuery: VkBool32,
        pub vertexPipelineStoresAndAtomics: VkBool32,
        pub fragmentStoresAndAtomics: VkBool32,
        pub shaderTessellationAndGeometryPointSize: VkBool32,
        pub shaderImageGatherExtended: VkBool32,
        pub shaderStorageImageExtendedFormats: VkBool32,
        pub shaderStorageImageMultisample: VkBool32,
        pub shaderStorageImageReadWithoutFormat: VkBool32,
        pub shaderStorageImageWriteWithoutFormat: VkBool32,
        pub shaderUniformBufferArrayDynamicIndexing: VkBool32,
        pub shaderSampledImageArrayDynamicIndexing: VkBool32,
        pub shaderStorageBufferArrayDynamicIndexing: VkBool32,
        pub shaderStorageImageArrayDynamicIndexing: VkBool32,
        pub shaderClipDistance: VkBool32,
        pub shaderCullDistance: VkBool32,
        pub shaderFloat64: VkBool32,
        pub shaderInt64: VkBool32,
        pub shaderInt16: VkBool32,
        pub shaderResourceResidency: VkBool32,
        pub shaderResourceMinLod: VkBool32,
        pub sparseBinding: VkBool32,
        pub sparseResidencyBuffer: VkBool32,
        pub sparseResidencyImage2D: VkBool32,
        pub sparseResidencyImage3D: VkBool32,
        pub sparseResidency2Samples: VkBool32,
        pub sparseResidency4Samples: VkBool32,
        pub sparseResidency8Samples: VkBool32,
        pub sparseResidency16Samples: VkBool32,
        pub sparseResidencyAliased: VkBool32,
        pub variableMultisampleRate: VkBool32,
        pub inheritedQueries: VkBool32
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkFormatProperties {
        pub linearTilingFeatures: VkFormatFeatureFlags,
        pub optimalTilingFeatures: VkFormatFeatureFlags,
        pub bufferFeatures: VkFormatFeatureFlags
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkExtent3D {
        pub width: uint32_t,
        pub height: uint32_t,
        pub depth: uint32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkImageFormatProperties {
        pub maxExtent: VkExtent3D,
        pub maxMipLevels: uint32_t,
        pub maxArrayLayers: uint32_t,
        pub sampleCounts: VkSampleCountFlags,
        pub maxResourceSize: VkDeviceSize
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkPhysicalDeviceLimits {
        pub maxImageDimension2D: uint32_t,
        pub maxImageDimension1D: uint32_t,
        pub maxImageDimension3D: uint32_t,
        pub maxImageDimensionCube: uint32_t,
        pub maxImageArrayLayers: uint32_t,
        pub maxTexelBufferElements: uint32_t,
        pub maxUniformBufferRange: uint32_t,
        pub maxStorageBufferRange: uint32_t,
        pub maxPushConstantsSize: uint32_t,
        pub maxMemoryAllocationCount: uint32_t,
        pub maxSamplerAllocationCount: uint32_t,
        pub bufferImageGranularity: VkDeviceSize,
        pub sparseAddressSpaceSize: VkDeviceSize,
        pub maxBoundDescriptorSets: uint32_t,
        pub maxPerStageDescriptorSamplers: uint32_t,
        pub maxPerStageDescriptorUniformBuffers: uint32_t,
        pub maxPerStageDescriptorStorageBuffers: uint32_t,
        pub maxPerStageDescriptorSampledImages: uint32_t,
        pub maxPerStageDescriptorStorageImages: uint32_t,
        pub maxPerStageDescriptorInputAttachments: uint32_t,
        pub maxPerStageResources: uint32_t,
        pub maxDescriptorSetSamplers: uint32_t,
        pub maxDescriptorSetUniformBuffers: uint32_t,
        pub maxDescriptorSetUniformBuffersDynamic: uint32_t,
        pub maxDescriptorSetStorageBuffers: uint32_t,
        pub maxDescriptorSetStorageBuffersDynamic: uint32_t,
        pub maxDescriptorSetSampledImages: uint32_t,
        pub maxDescriptorSetStorageImages: uint32_t,
        pub maxDescriptorSetInputAttachments: uint32_t,
        pub maxVertexInputAttributes: uint32_t,
        pub maxVertexInputBindings: uint32_t,
        pub maxVertexInputAttributeOffset: uint32_t,
        pub maxVertexInputBindingStride: uint32_t,
        pub maxVertexOutputComponents: uint32_t,
        pub maxTessellationGenerationLevel: uint32_t,
        pub maxTessellationPatchSize: uint32_t,
        pub maxTessellationControlPerVertexInputComponents: uint32_t,
        pub maxTessellationControlPerVertexOutputComponents: uint32_t,
        pub maxTessellationControlPerPatchOutputComponents: uint32_t,
        pub maxTessellationControlTotalOutputComponents: uint32_t,
        pub maxTessellationEvaluationInputComponents: uint32_t,
        pub maxTessellationEvaluationOutputComponents: uint32_t,
        pub maxGeometryShaderInvocations: uint32_t,
        pub maxGeometryInputComponents: uint32_t,
        pub maxGeometryOutputComponents: uint32_t,
        pub maxGeometryOutputVertices: uint32_t,
        pub maxGeometryTotalOutputComponents: uint32_t,
        pub maxFragmentInputComponents: uint32_t,
        pub maxFragmentOutputAttachments: uint32_t,
        pub maxFragmentDualSrcAttachments: uint32_t,
        pub maxFragmentCombinedOutputResources: uint32_t,
        pub maxComputeSharedMemorySize: uint32_t,
        pub maxComputeWorkGroupCount: [uint32_t;3],
        pub maxComputeWorkGroupInvocations: uint32_t,
        pub maxComputeWorkGroupSize: [uint32_t;3],
        pub subPixelPrecisionBits: uint32_t,
        pub subTexelPrecisionBits: uint32_t,
        pub mipmapPrecisionBits: uint32_t,
        pub maxDrawIndexedIndexValue: uint32_t,
        pub maxDrawIndirectCount: uint32_t,
        pub maxSamplerLodBias: c_float,
        pub maxSamplerAnisotropy: c_float,
        pub maxViewports: uint32_t,
        pub maxViewportDimensions: [uint32_t;2],
        pub viewportBoundsRange: [c_float;2],
        pub viewportSubPixelBits: uint32_t,
        pub minMemoryMapAlignment: size_t,
        pub minTexelBufferOffsetAlignment: VkDeviceSize,
        pub minUniformBufferOffsetAlignment: VkDeviceSize,
        pub minStorageBufferOffsetAlignment: VkDeviceSize,
        pub minTexelOffset: int32_t,
        pub maxTexelOffset: uint32_t,
        pub minTexelGatherOffset: int32_t,
        pub maxTexelGatherOffset: uint32_t,
        pub minInterpolationOffset: c_float,
        pub maxInterpolationOffset: c_float,
        pub subPixelInterpolationOffsetBits: uint32_t,
        pub maxFramebufferWidth: uint32_t,
        pub maxFramebufferHeight: uint32_t,
        pub maxFramebufferLayers: uint32_t,
        pub framebufferColorSampleCounts: VkSampleCountFlags,
        pub framebufferDepthSampleCounts: VkSampleCountFlags,
        pub framebufferStencilSampleCounts: VkSampleCountFlags,
        pub framebufferNoAttachmentsSampleCounts: VkSampleCountFlags,
        pub maxColorAttachments: uint32_t,
        pub sampledImageColorSampleCounts: VkSampleCountFlags,
        pub sampledImageIntegerSampleCounts: VkSampleCountFlags,
        pub sampledImageDepthSampleCounts: VkSampleCountFlags,
        pub sampledImageStencilSampleCounts: VkSampleCountFlags,
        pub storageImageSampleCounts: VkSampleCountFlags,
        pub maxSampleMaskWords: uint32_t,
        pub timestampComputeAndGraphics: VkBool32,
        pub timestampPeriod: c_float,
        pub maxClipDistances: uint32_t,
        pub maxCullDistances: uint32_t,
        pub maxCombinedClipAndCullDistances: uint32_t,
        pub discreteQueuePriorities: uint32_t,
        pub pointSizeRange: [c_float;2],
        pub lineWidthRange: [c_float;2],
        pub pointSizeGranularity: c_float,
        pub lineWidthGranularity: c_float,
        pub strictLines: VkBool32,
        pub standardSampleLocations: VkBool32,
        pub optimalBufferCopyOffsetAlignment: VkDeviceSize,
        pub optimalBufferCopyRowPitchAlignment: VkDeviceSize,
        pub nonCoherentAtomSize: VkDeviceSize
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkPhysicalDeviceSparseProperties {
        pub residencyStandard2DBlockShape: VkBool32,
        pub residencyStandard2DMultisampleBlockShape: VkBool32,
        pub residencyStandard3DBlockShape: VkBool32,
        pub residencyAlignedMipSize: VkBool32,
        pub residencyNonResidentStrict: VkBool32
    }

    #[repr(C)]
    #[derive(Copy)]
    pub struct VkPhysicalDeviceProperties {
        pub apiVersion: uint32_t,
        pub driverVersion: uint32_t,
        pub vendorID: uint32_t,
        pub deviceID: uint32_t,
        pub deviceType: VkPhysicalDeviceType,
        pub deviceName: [c_char;VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
        pub pipelineCacheUUID: [uint8_t;VK_UUID_SIZE],
        pub limits: VkPhysicalDeviceLimits,
        pub sparseProperties: VkPhysicalDeviceSparseProperties,
    }

    // Due to Rust issue #7622
    impl Clone for VkPhysicalDeviceProperties {
        fn clone(&self) -> Self {
            unsafe {
                ::std::mem::transmute_copy(self)
            }
        }
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkQueueFamilyProperties {
        pub queueFlags: VkQueueFlags,
        pub queueCount: uint32_t,
        pub timestampValidBits: uint32_t,
        pub minImageTransferGranularity: VkExtent3D
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkMemoryType {
        pub propertyFlags: VkMemoryPropertyFlags,
        pub heapIndex: uint32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkMemoryHeap {
        pub size: VkDeviceSize,
        pub flags: VkMemoryHeapFlags
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkPhysicalDeviceMemoryProperties {
        pub memoryTypeCount: uint32_t,
        pub memoryTypes: [VkMemoryType;VK_MAX_MEMORY_TYPES],
        pub memoryHeapCount: uint32_t,
        pub memoryHeaps: [VkMemoryHeap;VK_MAX_MEMORY_HEAPS]
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkDeviceQueueCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkDeviceQueueCreateFlags,
        pub queueFamilyIndex: uint32_t,
        pub queueCount: uint32_t,
        pub pQueuePriorities: *const c_float
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkDeviceCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkDeviceCreateFlags,
        pub queueCreateInfoCount: uint32_t,
        pub pQueueCreateInfos: *const VkDeviceQueueCreateInfo,
        pub enabledLayerCount: uint32_t,
        pub ppEnabledLayerNames: *const *const c_char,
        pub enabledExtensionCount: uint32_t,
        pub ppEnabledExtensionNames: *const *const c_char,
        pub pEnabledFeatures: *const VkPhysicalDeviceFeatures
    }

    #[repr(C)]
    #[derive(Copy)]
    pub struct VkExtensionProperties {
        pub extensionName: [c_char; VK_MAX_EXTENSION_NAME_SIZE],
        pub specVersion: uint32_t
    }

    // Due to Rust issue #7622
    impl Clone for VkExtensionProperties {
        fn clone(&self) -> Self {
            unsafe {
                ::std::mem::transmute_copy(self)
            }
        }
    }

    #[repr(C)]
    #[derive(Copy)]
    pub struct VkLayerProperties {
        pub layerName: [c_char;VK_MAX_EXTENSION_NAME_SIZE],
        pub specVersion: uint32_t,
        pub implementationVersion: uint32_t,
        pub description: [c_char;VK_MAX_DESCRIPTION_SIZE]
    }

    // Due to Rust issue #7622
    impl Clone for VkLayerProperties {
        fn clone(&self) -> Self {
            unsafe {
                ::std::mem::transmute_copy(self)
            }
        }
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkSubmitInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub waitSemaphoreCount: uint32_t,
        pub pWaitSemaphores: *const VkSemaphore,
        pub pWaitDstStageMask: *const VkPipelineStageFlags,
        pub commandBufferCount: uint32_t,
        pub pCommandBuffers: *const VkCommandBuffer,
        pub signalSemaphoreCount: uint32_t,
        pub pSignalSemaphores: *const VkSemaphore
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkMemoryAllocateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub allocationSize: VkDeviceSize,
        pub memoryTypeIndex: uint32_t,
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkMappedMemoryRange {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub memory: VkDeviceMemory,
        pub offset: VkDeviceSize,
        pub size: VkDeviceSize
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkMemoryRequirements {
        pub size: VkDeviceSize,
        pub alignment: VkDeviceSize,
        pub memoryTypeBits: uint32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkSparseImageFormatProperties {
        pub aspectMask: VkImageAspectFlags,
        pub imageGranularity: VkExtent3D,
        pub flags: VkSparseImageFormatFlags
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkSparseImageMemoryRequirements {
        pub formatProperties: VkSparseImageFormatProperties,
        pub imageMipTailFirstLod: uint32_t,
        pub imageMipTailSize: VkDeviceSize,
        pub imageMipTailOffset: VkDeviceSize,
        pub imageMipTailStride: VkDeviceSize
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkSparseMemoryBind {
        pub resourceOffset: VkDeviceSize,
        pub size: VkDeviceSize,
        pub memory: VkDeviceMemory,
        pub memoryOffset: VkDeviceSize,
        pub flags: VkSparseMemoryBindFlags
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkSparseBufferMemoryBindInfo {
        pub buffer: VkBuffer,
        pub bindCount: uint32_t,
        pub pBinds: *const VkSparseMemoryBind
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkSparseImageOpaqueMemoryBindInfo {
        pub image: VkImage,
        pub bindCount: uint32_t,
        pub pBinds: *const VkSparseMemoryBind
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkImageSubresource {
        pub aspectMask: VkImageAspectFlags,
        pub mipLevel: uint32_t,
        pub arrayLayer: uint32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkOffset3D {
        pub x: int32_t,
        pub y: int32_t,
        pub z: int32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkSparseImageMemoryBind {
        pub subresource: VkImageSubresource,
        pub offset: VkOffset3D,
        pub extent: VkExtent3D,
        pub memory: VkDeviceMemory,
        pub memoryOffset: VkDeviceSize,
        pub flags: VkSparseMemoryBindFlags
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkSparseImageMemoryBindInfo {
        pub image: VkImage,
        pub bindCount: uint32_t,
        pub pBinds: *const VkSparseImageMemoryBind
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkBindSparseInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub waitSemaphoreCount: uint32_t,
        pub pWaitSemaphores: *const VkSemaphore,
        pub bufferBindCount: uint32_t,
        pub pBufferBinds: *const VkSparseBufferMemoryBindInfo,
        pub imageOpaqueBindCount: uint32_t,
        pub pImageOpaqueBinds: *const VkSparseImageOpaqueMemoryBindInfo,
        pub imageBindCount: uint32_t,
        pub pImageBinds: *const VkSparseImageMemoryBindInfo,
        pub signalSemaphoreCount: uint32_t,
        pub pSignalSemaphores: *const VkSemaphore
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkFenceCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkFenceCreateFlags
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkSemaphoreCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkSemaphoreCreateFlags
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkEventCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkEventCreateFlags
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkQueryPoolCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkQueryPoolCreateFlags,
        pub queryType: VkQueryType,
        pub queryCount: uint32_t,
        pub pipelineStatistics: VkQueryPipelineStatisticFlags,
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkBufferCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkBufferCreateFlags,
        pub size: VkDeviceSize,
        pub usage: VkBufferUsageFlags,
        pub sharingMode: VkSharingMode,
        pub queueFamilyIndexCount: uint32_t,
        pub pQueueFamilyIndices: *const uint32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkBufferViewCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkBufferViewCreateFlags,
        pub buffer: VkBuffer,
        pub format: VkFormat,
        pub offset: VkDeviceSize,
        pub range: VkDeviceSize
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkImageCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkImageCreateFlags,
        pub imageType: VkImageType,
        pub format: VkFormat,
        pub extent: VkExtent3D,
        pub mipLevels: uint32_t,
        pub arrayLayers: uint32_t,
        pub samples: VkSampleCountFlags,
        pub tiling: VkImageTiling,
        pub usage: VkImageUsageFlags,
        pub sharingMode: VkSharingMode,
        pub queueFamilyIndexCount: uint32_t,
        pub pQueueFamilyIndices: *const uint32_t,
        pub initialLayout: VkImageLayout
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkSubresourceLayout {
        pub offset: VkDeviceSize,
        pub size: VkDeviceSize,
        pub rowPitch: VkDeviceSize,
        pub arrayPitch: VkDeviceSize,
        pub depthPitch: VkDeviceSize
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkComponentMapping {
        pub r: VkComponentSwizzle,
        pub g: VkComponentSwizzle,
        pub b: VkComponentSwizzle,
        pub a: VkComponentSwizzle
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkImageSubresourceRange {
        pub aspectMask: VkImageAspectFlags,
        pub baseMipLevel: uint32_t,
        pub levelCount: uint32_t,
        pub baseArrayLayer: uint32_t,
        pub layerCount: uint32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkImageViewCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkImageViewCreateFlags,
        pub image: VkImage,
        pub viewType: VkImageViewType,
        pub format: VkFormat,
        pub components: VkComponentMapping,
        pub subresourceRange: VkImageSubresourceRange
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkShaderModuleCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkShaderModuleCreateFlags,
        pub codeSize: size_t,
        pub pCode: *const uint32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkPipelineCacheCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkPipelineCacheCreateFlags,
        pub initialDataSize: size_t,
        pub pInitialData: *const c_void
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkSpecializationMapEntry {
        pub constantID: uint32_t,
        pub offset: uint32_t,
        pub size: size_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkSpecializationInfo {
        pub mapEntryCount: uint32_t,
        pub pMapEntries: *const VkSpecializationMapEntry,
        pub dataSize: size_t,
        pub pData: *const c_void
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkPipelineShaderStageCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkPipelineShaderStageCreateFlags,
        pub stage: VkShaderStageFlags,
        pub module: VkShaderModule,
        pub pName: *const c_char,
        pub pSpecializationInfo: *const VkSpecializationInfo
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkVertexInputBindingDescription {
        pub binding: uint32_t,
        pub stride: uint32_t,
        pub inputRate: VkVertexInputRate
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkVertexInputAttributeDescription {
        pub location: uint32_t,
        pub binding: uint32_t,
        pub format: VkFormat,
        pub offset: uint32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkPipelineVertexInputStateCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkPipelineVertexInputStateCreateFlags,
        pub vertexBindingDescriptionCount: uint32_t,
        pub pVertexBindingDescriptions: *const VkVertexInputBindingDescription,
        pub vertexAttributeDescriptionCount: uint32_t,
        pub pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkPipelineInputAssemblyStateCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkPipelineInputAssemblyStateCreateFlags,
        pub topology: VkPrimitiveTopology,
        pub primitiveRestartEnable: VkBool32
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkPipelineTessellationStateCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkPipelineTessellationStateCreateFlags,
        pub patchControlPoints: uint32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkViewport {
        pub x: c_float,
        pub y: c_float,
        pub width: c_float,
        pub height: c_float,
        pub minDepth: c_float,
        pub maxDepth: c_float
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkOffset2D {
        pub x: int32_t,
        pub y: int32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkExtent2D {
        pub width: uint32_t,
        pub height: uint32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkRect2D {
        pub offset: VkOffset2D,
        pub extent: VkExtent2D
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkPipelineViewportStateCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkPipelineViewportStateCreateFlags,
        pub viewportCount: uint32_t,
        pub pViewports: *const VkViewport,
        pub scissorCount: uint32_t,
        pub pScissors: *const VkRect2D
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkPipelineRasterizationStateCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkPipelineRasterizationStateCreateFlags,
        pub depthClampEnable: VkBool32,
        pub rasterizerDiscardEnable: VkBool32,
        pub polygonMode: VkPolygonMode,
        pub cullMode: VkCullModeFlags,
        pub frontFace: VkFrontFace,
        pub depthBiasEnable: VkBool32,
        pub depthBiasConstantFactor: c_float,
        pub depthBiasClamp: c_float,
        pub depthBiasSlopeFactor: c_float,
        pub lineWidth: c_float
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkPipelineMultisampleStateCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkPipelineMultisampleStateCreateFlags,
        pub rasterizationSamples: VkSampleCountFlags,
        pub sampleShadingEnable: VkBool32,
        pub minSampleShading: c_float,
        pub pSampleMask: *const VkSampleMask,
        pub alphaToCoverageEnable: VkBool32,
        pub alphaToOneEnable: VkBool32
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkStencilOpState {
        pub failOp: VkStencilOp,
        pub passOp: VkStencilOp,
        pub depthFailOp: VkStencilOp,
        pub compareOp: VkCompareOp,
        pub compareMask: uint32_t,
        pub writeMask: uint32_t,
        pub reference: uint32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkPipelineDepthStencilStateCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkPipelineDepthStencilStateCreateFlags,
        pub depthTestEnable: VkBool32,
        pub depthWriteEnable: VkBool32,
        pub depthCompareOp: VkCompareOp,
        pub depthBoundsTestEnable: VkBool32,
        pub stencilTestEnable: VkBool32,
        pub front: VkStencilOpState,
        pub back: VkStencilOpState,
        pub minDepthBounds: c_float,
        pub maxDepthBounds: c_float
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkPipelineColorBlendAttachmentState {
        pub blendEnable: VkBool32,
        pub srcColorBlendFactor: VkBlendFactor,
        pub dstColorBlendFactor: VkBlendFactor,
        pub colorBlendOp: VkBlendOp,
        pub srcAlphaBlendFactor: VkBlendFactor,
        pub dstAlphaBlendFactor: VkBlendFactor,
        pub alphaBlendOp: VkBlendOp,
        pub colorWriteMask: VkColorComponentFlags
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkPipelineColorBlendStateCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkPipelineColorBlendStateCreateFlags,
        pub logicOpEnable: VkBool32,
        pub logicOp: VkLogicOp,
        pub attachmentCount: uint32_t,
        pub pAttachments: *const VkPipelineColorBlendAttachmentState,
        pub blendConstants: [c_float;4]
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkPipelineDynamicStateCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkPipelineDynamicStateCreateFlags,
        pub dynamicStateCount: uint32_t,
        pub pDynamicStates: *const VkDynamicState
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkGraphicsPipelineCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkPipelineCreateFlags,
        pub stageCount: uint32_t,
        pub pStages: *const VkPipelineShaderStageCreateInfo,
        pub pVertexInputState: *const VkPipelineVertexInputStateCreateInfo,
        pub pInputAssemblyState: *const VkPipelineInputAssemblyStateCreateInfo,
        pub pTessellationState: *const VkPipelineTessellationStateCreateInfo,
        pub pViewportState: *const VkPipelineViewportStateCreateInfo,
        pub pRasterizationState: *const VkPipelineRasterizationStateCreateInfo,
        pub pMultisampleState: *const VkPipelineMultisampleStateCreateInfo,
        pub pDepthStencilState: *const VkPipelineDepthStencilStateCreateInfo,
        pub pColorBlendState: *const VkPipelineColorBlendStateCreateInfo,
        pub pDynamicState: *const VkPipelineDynamicStateCreateInfo,
        pub layout: VkPipelineLayout,
        pub renderPass: VkRenderPass,
        pub subpass: uint32_t,
        pub basePipelineHandle: VkPipeline,
        pub basePipelineIndex: int32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkComputePipelineCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkPipelineCreateFlags,
        pub stage: VkPipelineShaderStageCreateInfo,
        pub layout: VkPipelineLayout,
        pub basePipelineHandle: VkPipeline,
        pub basePipelineIndex: int32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkPushConstantRange {
        pub stageFlags: VkShaderStageFlags,
        pub offset: uint32_t,
        pub size: uint32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkPipelineLayoutCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkPipelineLayoutCreateFlags,
        pub setLayoutCount: uint32_t,
        pub pSetLayouts: *const VkDescriptorSetLayout,
        pub pushConstantRangeCount: uint32_t,
        pub pPushConstantRanges: *const VkPushConstantRange
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkSamplerCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkSamplerCreateFlags,
        pub magFilter: VkFilter,
        pub minFilter: VkFilter,
        pub mipmapMode: VkSamplerMipmapMode,
        pub addressModeU: VkSamplerAddressMode,
        pub addressModeV: VkSamplerAddressMode,
        pub addressModeW: VkSamplerAddressMode,
        pub mipLodBias: c_float,
        pub anisotropyEnable: VkBool32,
        pub maxAnisotropy: c_float,
        pub compareEnable: VkBool32,
        pub compareOp: VkCompareOp,
        pub minLod: c_float,
        pub maxLod: c_float,
        pub borderColor: VkBorderColor,
        pub unnormalizedCoordinates: VkBool32
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkDescriptorSetLayoutBinding {
        pub binding: uint32_t,
        pub descriptorType: VkDescriptorType,
        pub descriptorCount: uint32_t,
        pub stageFlags: VkShaderStageFlags,
        pub pImmutableSamplers: *const VkSampler
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkDescriptorSetLayoutCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkDescriptorSetLayoutCreateFlags,
        pub bindingCount: uint32_t,
        pub pBindings: *const VkDescriptorSetLayoutBinding
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkDescriptorPoolSize {
        /// Renamed from type to dType due to keyword collision
        pub dType: VkDescriptorType,
        pub descriptorCount: uint32_t,
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkDescriptorPoolCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkDescriptorPoolCreateFlags,
        pub maxSets: uint32_t,
        pub poolSizeCount: uint32_t,
        pub pPoolSizes: *const VkDescriptorPoolSize
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkDescriptorSetAllocateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub descriptorPool: VkDescriptorPool,
        pub descriptorSetCount: uint32_t,
        pub pSetLayouts: *const VkDescriptorSetLayout
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkDescriptorImageInfo {
        pub sampler: VkSampler,
        pub imageView: VkImageView,
        pub imageLayout: VkImageLayout
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkDescriptorBufferInfo {
        pub buffer: VkBuffer,
        pub offset: VkDeviceSize,
        pub range: VkDeviceSize
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkWriteDescriptorSet {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub dstSet: VkDescriptorSet,
        pub dstBinding: uint32_t,
        pub dstArrayElement: uint32_t,
        pub descriptorCount: uint32_t,
        pub descriptorType: VkDescriptorType,
        pub pImageInfo: *const VkDescriptorImageInfo,
        pub pBufferInfo: *const VkDescriptorBufferInfo,
        pub pTexelBufferView: *const VkBufferView
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkCopyDescriptorSet {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub srcSet: VkDescriptorSet,
        pub srcBinding: uint32_t,
        pub srcArrayElement: uint32_t,
        pub dstSet: VkDescriptorSet,
        pub dstBinding: uint32_t,
        pub dstArrayElement: uint32_t,
        pub descriptorCount: uint32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkFramebufferCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkFramebufferCreateFlags,
        pub renderPass: VkRenderPass,
        pub attachmentCount: uint32_t,
        pub pAttachments: *const VkImageView,
        pub width: uint32_t,
        pub height: uint32_t,
        pub layers: uint32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkAttachmentDescription {
        pub flags: VkAttachmentDescriptionFlags,
        pub format: VkFormat,
        pub samples: VkSampleCountFlags,
        pub loadOp: VkAttachmentLoadOp,
        pub storeOp: VkAttachmentStoreOp,
        pub stencilLoadOp: VkAttachmentLoadOp,
        pub stencilStoreOp: VkAttachmentStoreOp,
        pub initialLayout: VkImageLayout,
        pub finalLayout: VkImageLayout
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkAttachmentReference {
        pub attachment: uint32_t,
        pub layout: VkImageLayout
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkSubpassDescription {
        pub flags: VkSubpassDescriptionFlags,
        pub pipelineBindPoint: VkPipelineBindPoint,
        pub inputAttachmentCount: uint32_t,
        pub pInputAttachments: *const VkAttachmentReference,
        pub colorAttachmentCount: uint32_t,
        pub pColorAttachments: *const VkAttachmentReference,
        pub pResolveAttachments: *const VkAttachmentReference,
        pub pDepthStencilAttachment: *const VkAttachmentReference,
        pub preserveAttachmentCount: uint32_t,
        pub pPreserveAttachments: *const uint32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkSubpassDependency {
        pub srcSubpass: uint32_t,
        pub dstSubpass: uint32_t,
        pub srcStageMask: VkPipelineStageFlags,
        pub dstStageMask: VkPipelineStageFlags,
        pub srcAccessMask: VkAccessFlags,
        pub dstAccessMask: VkAccessFlags,
        pub dependencyFlags: VkDependencyFlags
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkRenderPassCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkRenderPassCreateFlags,
        pub attachmentCount: uint32_t,
        pub pAttachments: *const VkAttachmentDescription,
        pub subpassCount: uint32_t,
        pub pSubpasses: *const VkSubpassDescription,
        pub dependencyCount: uint32_t,
        pub pDependencies: *const VkSubpassDependency
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkCommandPoolCreateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkCommandPoolCreateFlags,
        pub queueFamilyIndex: uint32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkCommandBufferAllocateInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub commandPool: VkCommandPool,
        pub level: VkCommandBufferLevel,
        pub commandBufferCount: uint32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkCommandBufferInheritanceInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub renderPass: VkRenderPass,
        pub subpass: uint32_t,
        pub framebuffer: VkFramebuffer,
        pub occlusionQueryEnable: VkBool32,
        pub queryFlags: VkQueryControlFlags,
        pub pipelineStatistics: VkQueryPipelineStatisticFlags
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkCommandBufferBeginInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkCommandBufferUsageFlags,
        pub pInheritanceInfo: *const VkCommandBufferInheritanceInfo
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkBufferCopy {
        pub srcOffset: VkDeviceSize,
        pub dstOffset: VkDeviceSize,
        pub size: VkDeviceSize
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkImageSubresourceLayers {
        pub aspectMask: VkImageAspectFlags,
        pub mipLevel: uint32_t,
        pub baseArrayLayer: uint32_t,
        pub layerCount: uint32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkImageCopy {
        pub srcSubresource: VkImageSubresourceLayers,
        pub srcOffset: VkOffset3D,
        pub dstSubresource: VkImageSubresourceLayers,
        pub dstOffset: VkOffset3D,
        pub extent: VkExtent3D
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkImageBlit {
        pub srcSubresource: VkImageSubresourceLayers,
        pub srcOffsets: [VkOffset3D;2],
        pub dstSubresource: VkImageSubresourceLayers,
        pub dstOffsets: [VkOffset3D;2]
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkBufferImageCopy {
        pub bufferOffset: VkDeviceSize,
        pub bufferRowLength: uint32_t,
        pub bufferImageHeight: uint32_t,
        pub imageSubresource: VkImageSubresourceLayers,
        pub imageOffset: VkOffset3D,
        pub imageExtent: VkExtent3D
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkClearColorValue {
        union_data: [u8;16]
    }

    #[repr(C)]
    pub enum VkClearColorValueUnion {
        Float32([c_float;4]),
        Int32([int32_t;4]),
        UInt32([uint32_t;4])
    }

    impl From<VkClearColorValueUnion> for VkClearColorValue {
        fn from(union:VkClearColorValueUnion) -> Self {
            unsafe {
                match union {
                    VkClearColorValueUnion::Float32(color4f) => {
                        VkClearColorValue{union_data:transmute(color4f)}
                    },
                    VkClearColorValueUnion::Int32(color4i) => {
                        VkClearColorValue{union_data:transmute(color4i)}
                    },
                    VkClearColorValueUnion::UInt32(color4u) => {
                        VkClearColorValue{union_data:transmute(color4u)}
                    },
                }
            }
        }
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkClearDepthStencilValue {
        pub depth: c_float,
        pub stencil: uint32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkClearValue {
        union_data: [u8;16]
    }

    pub enum VkClearValueUnion {
        Color(VkClearColorValue),
        DepthStencil(VkClearDepthStencilValue)
    }

    impl From<VkClearValueUnion> for VkClearValue {
        fn from(union: VkClearValueUnion) -> Self {
            unsafe {
                match union {
                    VkClearValueUnion::Color(clear_color_value) => {
                        VkClearValue{union_data:clear_color_value.union_data}
                    },
                    VkClearValueUnion::DepthStencil(clear_depth_stencil_value) => {
                        let mut clear_value:VkClearValue = ::std::mem::zeroed();
                        {
                            let clear_color_bytes: [u8;8] = transmute(clear_depth_stencil_value);
                            let union_data_slice: &mut[u8] = &mut clear_value.union_data[0..8];
                            union_data_slice.clone_from_slice(&clear_color_bytes);
                        }
                        clear_value
                    }
                }
            }
        }
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkClearAttachment {
        pub aspectMask: VkImageAspectFlags,
        pub colorAttachment: uint32_t,
        pub clearValue: VkClearValue
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkClearRect {
        pub rect: VkRect2D,
        pub baseArrayLayer: uint32_t,
        pub layerCount: uint32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkImageResolve {
        pub srcSubresource: VkImageSubresourceLayers,
        pub srcOffset: VkOffset3D,
        pub dstSubresource: VkImageSubresourceLayers,
        pub dstOffset: VkOffset3D,
        pub extent: VkExtent3D
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkMemoryBarrier {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub srcAccessMask: VkAccessFlags,
        pub dstAccessMask: VkAccessFlags
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkBufferMemoryBarrier {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub srcAccessMask: VkAccessFlags,
        pub dstAccessMask: VkAccessFlags,
        pub srcQueueFamilyIndex: uint32_t,
        pub dstQueueFamilyIndex: uint32_t,
        pub buffer: VkBuffer,
        pub offset: VkDeviceSize,
        pub size: VkDeviceSize
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkImageMemoryBarrier {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub srcAccessMask: VkAccessFlags,
        pub dstAccessMask: VkAccessFlags,
        pub oldLayout: VkImageLayout,
        pub newLayout: VkImageLayout,
        pub srcQueueFamilyIndex: uint32_t,
        pub dstQueueFamilyIndex: uint32_t,
        pub image: VkImage,
        pub subresourceRange: VkImageSubresourceRange
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkRenderPassBeginInfo {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub renderPass: VkRenderPass,
        pub framebuffer: VkFramebuffer,
        pub renderArea: VkRect2D,
        pub clearValueCount: uint32_t,
        pub pClearValues: *const VkClearValue
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkDispatchIndirectCommand {
        pub x: uint32_t,
        pub y: uint32_t,
        pub z: uint32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkDrawIndexedIndirectCommand {
        pub indexCount: uint32_t,
        pub instanceCount: uint32_t,
        pub firstIndex: uint32_t,
        pub vertexOffset: int32_t,
        pub firstInstance: uint32_t
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkDrawIndirectCommand {
        pub vertexCount: uint32_t,
        pub instanceCount: uint32_t,
        pub firstVertex: uint32_t,
        pub firstInstance: uint32_t
    }

    pub type vkCreateInstanceFn = unsafe extern "stdcall" fn(pCreateInfo: *const VkInstanceCreateInfo, 
                                                             pAllocator: *const VkAllocationCallbacks, 
                                                             pInstance: *mut VkInstance) -> VkResult;

    pub type vkDestroyInstanceFn = unsafe extern "stdcall" fn(instance: VkInstance, 
                                                              pAllocator: *const VkAllocationCallbacks);

    pub type vkEnumeratePhysicalDevicesFn = unsafe extern "stdcall" fn(instance: VkInstance, 
                                                                       pPhysicalDeviceCount: *mut uint32_t, 
                                                                       pPhysicalDevices: *mut VkPhysicalDevice) -> VkResult;

    pub type vkGetPhysicalDeviceFeaturesFn = unsafe extern "stdcall" fn(physicalDevice: VkPhysicalDevice, 
                                                                        pFeatures: *mut VkPhysicalDeviceFeatures);

    pub type vkGetPhysicalDeviceFormatPropertiesFn = unsafe extern "stdcall" fn(physicalDevice: VkPhysicalDevice, 
                                                                                format: VkFormat, 
                                                                                pFormatProperties: *mut VkFormatProperties);

    pub type vkGetPhysicalDeviceImageFormatPropertiesFn = unsafe extern "stdcall" fn(physicalDevice: VkPhysicalDevice,
                                                                                     format: VkFormat,
                                                                                     iType: VkImageType,
                                                                                     tiling: VkImageTiling,
                                                                                     usage: VkImageUsageFlags,
                                                                                     flags: VkImageCreateFlags,
                                                                                     pImageFormatProperties: *mut VkImageFormatProperties) -> VkResult;

    pub type vkGetPhysicalDevicePropertiesFn = unsafe extern "stdcall" fn(physicalDevice: VkPhysicalDevice,
                                                                          pProperties: *mut VkPhysicalDeviceProperties);

    pub type vkGetPhysicalDeviceQueueFamilyPropertiesFn = unsafe extern "stdcall" fn(physicalDevice: VkPhysicalDevice,
                                                                                     pQueueFamilyPropertyCount: *mut uint32_t,
                                                                                     pQueueFamilyProperties: *mut VkQueueFamilyProperties);

    pub type vkGetPhysicalDeviceMemoryPropertiesFn = unsafe extern "stdcall" fn(physicalDevice: VkPhysicalDevice,
                                                                                pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties);

    pub type vkGetInstanceProcAddrFn = unsafe extern "stdcall" fn(instance: VkInstance,
                                                                  pName: *const c_char) -> vkVoidFunctionFn;

    pub type vkGetDeviceProcAddrFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                pName: *const c_char) -> vkVoidFunctionFn;

    pub type vkCreateDeviceFn = unsafe extern "stdcall" fn(physicalDevice: VkPhysicalDevice,
                                                           pCreateInfo: *const VkDeviceCreateInfo,
                                                           pAllocator: *const VkAllocationCallbacks,
                                                           pDevice: *mut VkDevice) -> VkResult;

    pub type vkDestroyDeviceFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                            pAllocator: *const VkAllocationCallbacks);

    pub type vkEnumerateInstanceExtensionPropertiesFn = unsafe extern "stdcall" fn(pLayerName: *const c_char,
                                                                                   pPropertyCount: *mut uint32_t,
                                                                                   pProperties: *mut VkExtensionProperties) -> VkResult;

    pub type vkEnumerateDeviceExtensionPropertiesFn = unsafe extern "stdcall" fn(physicalDevice: VkPhysicalDevice,
                                                                                 pLayerName: *const c_char,
                                                                                 pPropertyCount: *mut uint32_t,
                                                                                 pProperties: *mut VkExtensionProperties) -> VkResult;

    pub type vkEnumerateInstanceLayerPropertiesFn = unsafe extern "stdcall" fn(pPropertyCount: *mut uint32_t,
                                                                               pProperties: *mut VkLayerProperties) -> VkResult;

    pub type vkEnumerateDeviceLayerPropertiesFn = unsafe extern "stdcall" fn(physicalDevice: VkPhysicalDevice,
                                                                             pPropertyCount: *mut uint32_t,
                                                                             pProperties: *mut VkLayerProperties) -> VkResult;

    pub type vkGetDeviceQueueFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                             queueFamilyIndex: uint32_t,
                                                             queueIndex: uint32_t,
                                                             pQueue: *mut VkQueue);

    pub type vkQueueSubmitFn = unsafe extern "stdcall" fn(queue: VkQueue,
                                                          submitCount: uint32_t,
                                                          pSubmits: *const VkSubmitInfo,
                                                          fence: VkFence) -> VkResult;

    pub type vkQueueWaitIdleFn = unsafe extern "stdcall" fn(queue: VkQueue) -> VkResult;

    pub type vkDeviceWaitIdleFn = unsafe extern "stdcall" fn(device: VkDevice) -> VkResult;

    pub type vkAllocateMemoryFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                             pAllocateInfo: *const VkMemoryAllocateInfo,
                                                             pAllocator: *const VkAllocationCallbacks,
                                                             pMemory: *mut VkDeviceMemory) -> VkResult;

    pub type vkFreeMemoryFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                         memory: VkDeviceMemory,
                                                         pAllocator: *const VkAllocationCallbacks);

    pub type vkMapMemoryFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                        memory: VkDeviceMemory,
                                                        offset: VkDeviceSize,
                                                        size: VkDeviceSize,
                                                        flags: VkMemoryMapFlags,
                                                        ppData: *mut *mut c_void) -> VkResult;

    pub type vkUnmapMemoryFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                          memory: VkDeviceMemory);

    pub type vkFlushMappedMemoryRangesFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                      memoryRangeCount: uint32_t,
                                                                      pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;

    pub type vkInvalidateMappedMemoryRangesFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                           memoryRangeCount: uint32_t,
                                                                           pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;

    pub type vkGetDeviceMemoryCommitmentFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                        memory: VkDeviceMemory,
                                                                        pCommittedMemoryInBytes: *mut VkDeviceSize);

    pub type vkBindBufferMemoryFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                               buffer: VkBuffer,
                                                               memory: VkDeviceMemory,
                                                               memoryOffset: VkDeviceSize) -> VkResult;

    pub type vkBindImageMemoryFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                              image: VkImage,
                                                              memory: VkDeviceMemory,
                                                              memoryOffset: VkDeviceSize) -> VkResult;

    pub type vkGetBufferMemoryRequirementsFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                          buffer: VkBuffer,
                                                                          pMemoryRequirements: *mut VkMemoryRequirements);

    pub type vkGetImageMemoryRequirementsFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                         image: VkImage,
                                                                         pMemoryRequirements: *mut VkMemoryRequirements);

    pub type vkGetImageSparseMemoryRequirementsFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                               image: VkImage,
                                                                               pSparseMemoryRequirementCount: *mut uint32_t,
                                                                               pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements);

    pub type vkGetPhysicalDeviceSparseImageFormatPropertiesFn = unsafe extern "stdcall" fn(physicalDevice: VkPhysicalDevice,
                                                                                           format: VkFormat,
                                                                                           iType: VkImageType,
                                                                                           samples: VkSampleCountFlags,
                                                                                           usage: VkImageUsageFlags,
                                                                                           tiling: VkImageTiling,
                                                                                           pPropertyCount: *mut uint32_t,
                                                                                           pProperties: *mut VkSparseImageFormatProperties);
    pub type vkQueueBindSparseFn = unsafe extern "stdcall" fn(queue: VkQueue,
                                                              bindInfoCount: uint32_t,
                                                              pBindInfo: *const VkBindSparseInfo,
                                                              fence: VkFence) -> VkResult;

    pub type vkCreateFenceFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                          pCreateInfo: *const VkFenceCreateInfo,
                                                          pAllocator: *const VkAllocationCallbacks,
                                                          pFence: *mut VkFence) -> VkResult;

    pub type vkDestroyFenceFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                           fence: VkFence,
                                                           pAllocator: *const VkAllocationCallbacks);

    pub type vkResetFencesFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                          fenceCount: uint32_t,
                                                          pFences: *const VkFence) -> VkResult;

    pub type vkGetFenceStatusFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                             fence: VkFence) -> VkResult;

    pub type vkWaitForFencesFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                            fenceCount: uint32_t,
                                                            pFences: *const VkFence,
                                                            waitAll: VkBool32,
                                                            timeout: uint64_t) -> VkResult;

    pub type vkCreateSemaphoreFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                              pCreateInfo: *const VkSemaphoreCreateInfo,
                                                              pAllocator: *const VkAllocationCallbacks,
                                                              pSemaphore: *mut VkSemaphore) -> VkResult;

    pub type vkDestroySemaphoreFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                               semaphore: VkSemaphore,
                                                               pAllocator: *const VkAllocationCallbacks);

    pub type vkCreateEventFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                          pCreateInfo: *const VkEventCreateInfo,
                                                          pAllocator: *const VkAllocationCallbacks,
                                                          pEvent: *mut VkEvent) -> VkResult;

    pub type vkDestroyEventFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                           event: VkEvent,
                                                           pAllocator: *const VkAllocationCallbacks);

    pub type vkGetEventStatusFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                             event: VkEvent) -> VkResult;

    pub type vkSetEventFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                       event: VkEvent) -> VkResult;

    pub type vkResetEventFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                         event: VkEvent) -> VkResult;

    pub type vkCreateQueryPoolFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                              pCreateInfo: *const VkQueryPoolCreateInfo,
                                                              pAllocator: *const VkAllocationCallbacks,
                                                              pQueryPool: *mut VkQueryPool) -> VkResult;

    pub type vkDestroyQueryPoolFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                               queryPool: VkQueryPool,
                                                               pAllocator: *const VkAllocationCallbacks);

    pub type vkGetQueryPoolResultsFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                  queryPool: VkQueryPool,
                                                                  firstQuery: uint32_t,
                                                                  queryCount: uint32_t,
                                                                  dataSize: size_t,
                                                                  pData: *mut c_void,
                                                                  stride: VkDeviceSize,
                                                                  flags: VkDeviceSize) -> VkResult;

    pub type vkCreateBufferFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                           pCreateInfo: *const VkBufferCreateInfo,
                                                           pAllocator: *const VkAllocationCallbacks,
                                                           pBuffer: *mut VkBuffer) -> VkResult;

    pub type vkDestroyBufferFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                            buffer: VkBuffer,
                                                            pAllocator: *const VkAllocationCallbacks);

    pub type vkCreateBufferViewFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                               pCreateInfo: *const VkBufferViewCreateInfo,
                                                               pAllocator: *const VkAllocationCallbacks,
                                                               pView: *mut VkBufferView) -> VkResult;

    pub type vkDestroyBufferViewFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                bufferView: VkBufferView,
                                                                pAllocator: *const VkAllocationCallbacks);

    pub type vkCreateImageFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                          pCreateInfo: *const VkImageCreateInfo,
                                                          pAllocator: *const VkAllocationCallbacks,
                                                          pImage: *mut VkImage) -> VkResult;

    pub type vkDestroyImageFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                           image: VkImage,
                                                           pAllocator: *const VkAllocationCallbacks);

    pub type vkGetImageSubresourceLayoutFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                        image: VkImage,
                                                                        pSubresource: *const VkImageSubresource,
                                                                        pLayout: *mut VkSubresourceLayout);

    pub type vkCreateImageViewFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                              pCreateInfo: *const VkImageViewCreateInfo,
                                                              pAllocator: *const VkAllocationCallbacks,
                                                              pView: *mut VkImageView) -> VkResult;

    pub type vkDestroyImageViewFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                               imageView: VkImageView,
                                                               pAllocator: *const VkAllocationCallbacks);

    pub type vkCreateShaderModuleFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                 pCreateInfo: *const VkShaderModuleCreateInfo,
                                                                 pAllocator: *const VkAllocationCallbacks,
                                                                 pShaderModule: *mut VkShaderModule) -> VkResult;

    pub type vkDestroyShaderModuleFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                  shaderModule: VkShaderModule,
                                                                  pAllocator: *const VkAllocationCallbacks);

    pub type vkCreatePipelineCacheFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                  pCreateInfo: *const VkPipelineCacheCreateInfo,
                                                                  pAllocator: *const VkAllocationCallbacks,
                                                                  pPipelineCache: *mut VkPipelineCache) -> VkResult;

    pub type vkDestroyPipelineCacheFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                   pipelineCache: VkPipelineCache,
                                                                   pAllocator: *const VkAllocationCallbacks);

    pub type vkGetPipelineCacheDataFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                   pipelineCache: VkPipelineCache,
                                                                   pDataSize: *mut size_t,
                                                                   pData: *mut c_void) -> VkResult;

    pub type vkMergePipelineCachesFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                  dstCache: VkPipelineCache,
                                                                  srcCacheCount: uint32_t,
                                                                  pSrcCaches: *const VkPipelineCache) -> VkResult;

    pub type vkCreateGraphicsPipelinesFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                      pipelineCache: VkPipelineCache,
                                                                      createInfoCount: uint32_t,
                                                                      pCreateInfos: *const VkGraphicsPipelineCreateInfo,
                                                                      pAllocator: *const VkAllocationCallbacks,
                                                                      pPipelines: *mut VkPipeline) -> VkResult;

    pub type vkCreateComputePipelinesFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                     pipelineCache: VkPipelineCache,
                                                                     createInfoCount: uint32_t,
                                                                     pCreateInfos: *const VkComputePipelineCreateInfo,
                                                                     pAllocator: *const VkAllocationCallbacks,
                                                                     pPipelines: *mut VkPipeline) -> VkResult;

    pub type vkDestroyPipelineFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                              pipeline: VkPipeline,
                                                              pAllocator: *const VkAllocationCallbacks);

    pub type vkCreatePipelineLayoutFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                   pCreateInfo: *const VkPipelineLayoutCreateInfo,
                                                                   pAllocator: *const VkAllocationCallbacks,
                                                                   pPipelineLayout: *mut VkPipelineLayout) -> VkResult;

    pub type vkDestroyPipelineLayoutFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                    pipelineLayout: VkPipelineLayout,
                                                                    pAllocator: *const VkAllocationCallbacks);

    pub type vkCreateSamplerFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                            pCreateInfo: *const VkSamplerCreateInfo,
                                                            pAllocator: *const VkAllocationCallbacks,
                                                            pSampler: *mut VkSampler) -> VkResult;

    pub type vkDestroySamplerFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                             sampler: VkSampler,
                                                             pAllocator: *const VkAllocationCallbacks);

    pub type vkCreateDescriptorSetLayoutFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                        pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
                                                                        pAllocator: *const VkAllocationCallbacks,
                                                                        pSetLayout: *mut VkDescriptorSetLayout) -> VkResult;

    pub type vkDestroyDescriptorSetLayoutFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                         descriptorSetLayout: VkDescriptorSetLayout,
                                                                         pAllocator: *const VkAllocationCallbacks);

    pub type vkCreateDescriptorPoolFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                   pCreateInfo: *const VkDescriptorPoolCreateInfo,
                                                                   pAllocator: *const VkAllocationCallbacks,
                                                                   pDescriptorPool: *mut VkDescriptorPool) -> VkResult;

    pub type vkDestroyDescriptorPoolFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                    descriptorPool: VkDescriptorPool,
                                                                    pAllocator: *const VkAllocationCallbacks);

    pub type vkResetDescriptorPoolFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                  descriptorPool: VkDescriptorPool,
                                                                  flags: VkDescriptorPoolResetFlags) -> VkResult;

    pub type vkAllocateDescriptorSetsFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                     pAllocateInfo: *const VkDescriptorSetAllocateInfo,
                                                                     pDescriptorSets: *mut VkDescriptorSet) -> VkResult;

    pub type vkFreeDescriptorSetsFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                 descriptorPool: VkDescriptorPool,
                                                                 descriptorSetCount: uint32_t,
                                                                 pDescriptorSets: *const VkDescriptorSet) -> VkResult;

    pub type vkUpdateDescriptorSetsFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                   descriptorWriteCount: uint32_t,
                                                                   pDescriptorWrites: *const VkWriteDescriptorSet,
                                                                   descriptorCopyCount: uint32_t,
                                                                   pDescriptorCopies: *const VkCopyDescriptorSet);

    pub type vkCreateFramebufferFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                pCreateInfo: *const VkFramebufferCreateInfo,
                                                                pAllocator: *const VkAllocationCallbacks,
                                                                pFramebuffer: *mut VkFramebuffer) -> VkResult;

    pub type vkDestroyFramebufferFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                 framebuffer: VkFramebuffer,
                                                                 pAllocator: *const VkAllocationCallbacks);

    pub type vkCreateRenderPassFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                               pCreateInfo: *const VkRenderPassCreateInfo,
                                                               pAllocator: *const VkAllocationCallbacks,
                                                               pRenderPass: *mut VkRenderPass) -> VkResult;

    pub type vkDestroyRenderPassFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                renderPass: VkRenderPass,
                                                                pAllocator: *const VkAllocationCallbacks);

    pub type vkGetRenderAreaGranularityFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                       renderPass: VkRenderPass,
                                                                       pGranularity: *mut VkExtent2D);

    pub type vkCreateCommandPoolFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                pCreateInfo: *const VkCommandPoolCreateInfo,
                                                                pAllocator: *const VkAllocationCallbacks,
                                                                pCommandPool: *mut VkCommandPool) -> VkResult;

    pub type vkDestroyCommandPoolFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                 commandPool: VkCommandPool,
                                                                 pAllocator: *const VkAllocationCallbacks);

    pub type vkResetCommandPoolFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                               commandPool: VkCommandPool,
                                                               flags: VkCommandPoolResetFlags) -> VkResult;

    pub type vkAllocateCommandBuffersFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                     pAllocateInfo: *const VkCommandBufferAllocateInfo,
                                                                     pCommandBuffers: *mut VkCommandBuffer) -> VkResult;

    pub type vkFreeCommandBuffersFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                 commandPool: VkCommandPool,
                                                                 commandBufferCount: uint32_t,
                                                                 pCommandBuffers: *const VkCommandBuffer);

    pub type vkBeginCommandBufferFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                                 pBeginInfo: *const VkCommandBufferBeginInfo) -> VkResult;

    pub type vkEndCommandBufferFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer) -> VkResult;

    pub type vkResetCommandBufferFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                                 flags: VkCommandBufferResetFlags) -> VkResult;

    pub type vkCmdBindPipelineFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                              pipelineBindPoint: VkPipelineBindPoint,
                                                              pipeline: VkPipeline);

    pub type vkCmdSetViewportFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                             firstViewport: uint32_t,
                                                             viewportCount: uint32_t,
                                                             pViewports: *const VkViewport);

    pub type vkCmdSetScissorFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                            firstScissor: uint32_t,
                                                            scissorCount: uint32_t,
                                                            pScissors: *const VkRect2D);

    pub type vkCmdSetLineWidthFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                              lineWidth: c_float);

    pub type vkCmdSetDepthBiasFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                              depthBiasConstantFactor: c_float,
                                                              depthBiasClamp: c_float,
                                                              depthBiasSlopeFactor: c_float);

    // TODO: make sure [c_float;4] is the right type here
    pub type vkCmdSetBlendConstantsFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                                   blendConstants: [c_float;4]);

    pub type vkCmdSetDepthBoundsFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                                minDepthBounds: c_float,
                                                                maxDepthBounds: c_float);

    pub type vkCmdSetStencilCompareMaskFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                                       faceMask: VkStencilFaceFlags,
                                                                       compareMask: uint32_t);

    pub type vkCmdSetStencilWriteMaskFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                                     faceMask: VkStencilFaceFlags,
                                                                     writeMask: uint32_t);

    pub type vkCmdSetStencilReferenceFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                                     faceMask: VkStencilFaceFlags,
                                                                     reference: uint32_t);

    pub type vkCmdBindDescriptorSetsFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                                    pipelineBindPoint: VkPipelineBindPoint,
                                                                    layout: VkPipelineLayout,
                                                                    firstSet: uint32_t,
                                                                    descriptorSetCount: uint32_t,
                                                                    pDescriptorSets: *const VkDescriptorSet,
                                                                    dynamicOffsetCount: uint32_t,
                                                                    pDynamicOffsets: *const uint32_t);

    pub type vkCmdBindIndexBufferFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                                 buffer: VkBuffer,
                                                                 offset: VkDeviceSize,
                                                                 indexType: VkIndexType);

    pub type vkCmdBindVertexBuffersFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                                   firstBinding: uint32_t,
                                                                   bindingCount: uint32_t,
                                                                   pBuffers: *const VkBuffer,
                                                                   pOffsets: *const VkDeviceSize);

    pub type vkCmdDrawFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                      vertexCount: uint32_t,
                                                      instanceCount: uint32_t,
                                                      firstVertex: uint32_t,
                                                      firstInstance: uint32_t);

    pub type vkCmdDrawIndexedFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                             indexCount: uint32_t,
                                                             instanceCount: uint32_t,
                                                             firstIndex: uint32_t,
                                                             vertexOffset: int32_t,
                                                             firstInstance: uint32_t);

    pub type vkCmdDrawIndirectFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                              buffer: VkBuffer,
                                                              offset: VkDeviceSize,
                                                              drawCount: uint32_t,
                                                              stride: uint32_t);

    pub type vkCmdDrawIndexedIndirectFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                                     buffer: VkBuffer,
                                                                     offset: VkDeviceSize,
                                                                     drawCount: uint32_t,
                                                                     stride: uint32_t);

    pub type vkCmdDispatchFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                          x: uint32_t,
                                                          y: uint32_t,
                                                          z: uint32_t);

    pub type vkCmdDispatchIndirectFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                                  buffer: VkBuffer,
                                                                  offset: VkDeviceSize);

    pub type vkCmdCopyBufferFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                            srcBuffer: VkBuffer,
                                                            dstBuffer: VkBuffer,
                                                            regionCount: uint32_t,
                                                            pRegions: *const VkBufferCopy);

    pub type vkCmdCopyImageFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                           srcImage: VkImage,
                                                           srcImageLayout: VkImageLayout,
                                                           dstImage: VkImage,
                                                           dstImageLayout: VkImageLayout,
                                                           regionCount: uint32_t,
                                                           pRegions: *const VkImageCopy);

    pub type vkCmdBlitImageFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                           srcImage: VkImage,
                                                           srcImageLayout: VkImageLayout,
                                                           dstImage: VkImage,
                                                           dstImageLayout: VkImageLayout,
                                                           regionCount: uint32_t,
                                                           pRegions: *const VkImageBlit,
                                                           filter: VkFilter);

    pub type vkCmdCopyBufferToImageFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                                   srcBuffer: VkBuffer,
                                                                   dstImage: VkImage,
                                                                   dstImageLayout: VkImageLayout,
                                                                   regionCount: uint32_t,
                                                                   pRegions: *const VkBufferImageCopy);

    pub type vkCmdCopyImageToBufferFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                                   srcImage: VkImage,
                                                                   srcImageLayout: VkImageLayout,
                                                                   dstBuffer: VkBuffer,
                                                                   regionCount: uint32_t,
                                                                   pRegions: *const VkBufferImageCopy);

    pub type vkCmdUpdateBufferFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                              dstBuffer: VkBuffer,
                                                              dstOffset: VkDeviceSize,
                                                              dataSize: VkDeviceSize,
                                                              pData: *const uint32_t);

    pub type vkCmdFillBufferFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                            dstBuffer: VkBuffer,
                                                            dstOffset: VkDeviceSize,
                                                            size: VkDeviceSize,
                                                            data: uint32_t);

    pub type vkCmdClearColorImageFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                                 image: VkImage,
                                                                 imageLayout: VkImageLayout,
                                                                 pColor: *const VkClearColorValue,
                                                                 rangeCount: uint32_t,
                                                                 pRanges: *const VkImageSubresourceRange);

    pub type vkCmdClearDepthStencilImageFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                                        image: VkImage,
                                                                        imageLayout: VkImageLayout,
                                                                        pDepthStencil: *const VkClearDepthStencilValue,
                                                                        rangeCount: uint32_t,
                                                                        pRanges: *const VkImageSubresourceRange);

    pub type vkCmdClearAttachmentsFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                                  attachmentCount: uint32_t,
                                                                  pAttachments: *const VkClearAttachment,
                                                                  rectCount: uint32_t,
                                                                  pRects: *const VkClearRect);

    pub type vkCmdResolveImageFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                              srcImage: VkImage,
                                                              srcImageLayout: VkImageLayout,
                                                              dstImage: VkImage,
                                                              dstImageLayout: VkImageLayout,
                                                              regionCount: uint32_t,
                                                              pRegions: *const VkImageResolve);

    pub type vkCmdSetEventFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                          event: VkEvent,
                                                          stageMask: VkPipelineStageFlags);

    pub type vkCmdResetEventFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                            event: VkEvent,
                                                            stageMask: VkPipelineStageFlags);

    pub type vkCmdWaitEventsFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                            eventCount: uint32_t,
                                                            pEvents: *const VkEvent,
                                                            srcStageMask: VkPipelineStageFlags,
                                                            dstStageMask: VkPipelineStageFlags,
                                                            memoryBarrierCount: uint32_t,
                                                            pMemoryBarriers: *const VkMemoryBarrier,
                                                            bufferMemoryBarrierCount: uint32_t,
                                                            pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
                                                            imageMemoryBarrierCount: uint32_t,
                                                            pImageMemoryBarriers: *const VkImageMemoryBarrier);

    pub type vkCmdPipelineBarrierFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                                 srcStageMask: VkPipelineStageFlags,
                                                                 dstStageMask: VkPipelineStageFlags,
                                                                 dependencyFlags: VkDependencyFlags,
                                                                 memoryBarrierCount: uint32_t,
                                                                 pMemoryBarriers: *const VkMemoryBarrier,
                                                                 bufferMemoryBarrierCount: uint32_t,
                                                                 pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
                                                                 imageMemoryBarrierCount: uint32_t,
                                                                 pImageMemoryBarriers: *const VkImageMemoryBarrier);

    pub type vkCmdBeginQueryFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                            queryPool: VkQueryPool,
                                                            query: uint32_t,
                                                            flags: VkQueryControlFlags);

    pub type vkCmdEndQueryFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                          queryPool: VkQueryPool,
                                                          query: uint32_t);

    pub type vkCmdResetQueryPoolFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                                queryPool: VkQueryPool,
                                                                firstQuery: uint32_t,
                                                                queryCount: uint32_t);

    pub type vkCmdWriteTimestampFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                                pipelineStage: VkPipelineStageFlags,
                                                                queryPool: VkQueryPool,
                                                                query: uint32_t);

    pub type vkCmdCopyQueryPoolResultsFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                                      queryPool: VkQueryPool,
                                                                      firstQuery: uint32_t,
                                                                      queryCount: uint32_t,
                                                                      dstBuffer: VkBuffer,
                                                                      dstOffset: VkDeviceSize,
                                                                      stride: VkDeviceSize,
                                                                      flags: VkQueryResultFlags);

    pub type vkCmdPushConstantsFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                               layout: VkPipelineLayout,
                                                               stageFlags: VkShaderStageFlags,
                                                               offset: uint32_t,
                                                               size: uint32_t,
                                                               pValues: *const c_void);

    pub type vkCmdBeginRenderPassFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                                 pRenderPassBegin: *const VkRenderPassBeginInfo,
                                                                 contents: VkSubpassContents);

    pub type vkCmdNextSubpassFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                             contents: VkSubpassContents);

    pub type vkCmdEndRenderPassFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer);

    pub type vkCmdExecuteCommandsFn = unsafe extern "stdcall" fn(commandBuffer: VkCommandBuffer,
                                                                 commandBufferCount: uint32_t,
                                                                 pCommandBuffers: *const VkCommandBuffer);

    pub struct VkCoreCommands {
        library: Option<DynamicLibrary>,
        vkCreateInstance: Option<vkCreateInstanceFn>,
        vkDestroyInstance: Option<vkDestroyInstanceFn>,
        vkEnumeratePhysicalDevices: Option<vkEnumeratePhysicalDevicesFn>,
        vkGetPhysicalDeviceFeatures: Option<vkGetPhysicalDeviceFeaturesFn>,
        vkGetPhysicalDeviceFormatProperties: Option<vkGetPhysicalDeviceFormatPropertiesFn>,
        vkGetPhysicalDeviceImageFormatProperties: Option<vkGetPhysicalDeviceImageFormatPropertiesFn>,
        vkGetPhysicalDeviceProperties: Option<vkGetPhysicalDevicePropertiesFn>,
        vkGetPhysicalDeviceQueueFamilyProperties: Option<vkGetPhysicalDeviceQueueFamilyPropertiesFn>,
        vkGetPhysicalDeviceMemoryProperties: Option<vkGetPhysicalDeviceMemoryPropertiesFn>,
        vkGetInstanceProcAddr: Option<vkGetInstanceProcAddrFn>,
        vkGetDeviceProcAddr: Option<vkGetDeviceProcAddrFn>,
        vkCreateDevice: Option<vkCreateDeviceFn>,
        vkDestroyDevice: Option<vkDestroyDeviceFn>,
        vkEnumerateInstanceExtensionProperties: Option<vkEnumerateInstanceExtensionPropertiesFn>,
        vkEnumerateDeviceExtensionProperties: Option<vkEnumerateDeviceExtensionPropertiesFn>,
        vkEnumerateInstanceLayerProperties: Option<vkEnumerateInstanceLayerPropertiesFn>,
        vkEnumerateDeviceLayerProperties: Option<vkEnumerateDeviceLayerPropertiesFn>,
        vkGetDeviceQueue: Option<vkGetDeviceQueueFn>,
        vkQueueSubmit: Option<vkQueueSubmitFn>,
        vkQueueWaitIdle: Option<vkQueueWaitIdleFn>,
        vkDeviceWaitIdle: Option<vkDeviceWaitIdleFn>,
        vkAllocateMemory: Option<vkAllocateMemoryFn>,
        vkFreeMemory: Option<vkFreeMemoryFn>,
        vkMapMemory: Option<vkMapMemoryFn>,
        vkUnmapMemory: Option<vkUnmapMemoryFn>,
        vkFlushMappedMemoryRanges: Option<vkFlushMappedMemoryRangesFn>,
        vkInvalidateMappedMemoryRanges: Option<vkInvalidateMappedMemoryRangesFn>,
        vkGetDeviceMemoryCommitment: Option<vkGetDeviceMemoryCommitmentFn>,
        vkBindBufferMemory: Option<vkBindBufferMemoryFn>,
        vkBindImageMemory: Option<vkBindImageMemoryFn>,
        vkGetBufferMemoryRequirements: Option<vkGetBufferMemoryRequirementsFn>,
        vkGetImageMemoryRequirements: Option<vkGetImageMemoryRequirementsFn>,
        vkGetImageSparseMemoryRequirements: Option<vkGetImageSparseMemoryRequirementsFn>,
        vkGetPhysicalDeviceSparseImageFormatProperties: Option<vkGetPhysicalDeviceSparseImageFormatPropertiesFn>,
        vkQueueBindSparse: Option<vkQueueBindSparseFn>,
        vkCreateFence: Option<vkCreateFenceFn>,
        vkDestroyFence: Option<vkDestroyFenceFn>,
        vkResetFences: Option<vkResetFencesFn>,
        vkGetFenceStatus: Option<vkGetFenceStatusFn>,
        vkWaitForFences: Option<vkWaitForFencesFn>,
        vkCreateSemaphore: Option<vkCreateSemaphoreFn>,
        vkDestroySemaphore: Option<vkDestroySemaphoreFn>,
        vkCreateEvent: Option<vkCreateEventFn>,
        vkDestroyEvent: Option<vkDestroyEventFn>,
        vkGetEventStatus: Option<vkGetEventStatusFn>,
        vkSetEvent: Option<vkSetEventFn>,
        vkResetEvent: Option<vkResetEventFn>,
        vkCreateQueryPool: Option<vkCreateQueryPoolFn>,
        vkDestroyQueryPool: Option<vkDestroyQueryPoolFn>,
        vkGetQueryPoolResults: Option<vkGetQueryPoolResultsFn>,
        vkCreateBuffer: Option<vkCreateBufferFn>,
        vkDestroyBuffer: Option<vkDestroyBufferFn>,
        vkCreateBufferView: Option<vkCreateBufferViewFn>,
        vkDestroyBufferView: Option<vkDestroyBufferViewFn>,
        vkCreateImage: Option<vkCreateImageFn>,
        vkDestroyImage: Option<vkDestroyImageFn>,
        vkGetImageSubresourceLayout: Option<vkGetImageSubresourceLayoutFn>,
        vkCreateImageView: Option<vkCreateImageViewFn>,
        vkDestroyImageView: Option<vkDestroyImageViewFn>,
        vkCreateShaderModule: Option<vkCreateShaderModuleFn>,
        vkDestroyShaderModule: Option<vkDestroyShaderModuleFn>,
        vkCreatePipelineCache: Option<vkCreatePipelineCacheFn>,
        vkDestroyPipelineCache: Option<vkDestroyPipelineCacheFn>,
        vkGetPipelineCacheData: Option<vkGetPipelineCacheDataFn>,
        vkMergePipelineCaches: Option<vkMergePipelineCachesFn>,
        vkCreateGraphicsPipelines: Option<vkCreateGraphicsPipelinesFn>,
        vkCreateComputePipelines: Option<vkCreateComputePipelinesFn>,
        vkDestroyPipeline: Option<vkDestroyPipelineFn>,
        vkCreatePipelineLayout: Option<vkCreatePipelineLayoutFn>,
        vkDestroyPipelineLayout: Option<vkDestroyPipelineLayoutFn>,
        vkCreateSampler: Option<vkCreateSamplerFn>,
        vkDestroySampler: Option<vkDestroySamplerFn>,
        vkCreateDescriptorSetLayout: Option<vkCreateDescriptorSetLayoutFn>,
        vkDestroyDescriptorSetLayout: Option<vkDestroyDescriptorSetLayoutFn>,
        vkCreateDescriptorPool: Option<vkCreateDescriptorPoolFn>,
        vkDestroyDescriptorPool: Option<vkDestroyDescriptorPoolFn>,
        vkResetDescriptorPool: Option<vkResetDescriptorPoolFn>,
        vkAllocateDescriptorSets: Option<vkAllocateDescriptorSetsFn>,
        vkFreeDescriptorSets: Option<vkFreeDescriptorSetsFn>,
        vkUpdateDescriptorSets: Option<vkUpdateDescriptorSetsFn>,
        vkCreateFramebuffer: Option<vkCreateFramebufferFn>,
        vkDestroyFramebuffer: Option<vkDestroyFramebufferFn>,
        vkCreateRenderPass: Option<vkCreateRenderPassFn>,
        vkDestroyRenderPass: Option<vkDestroyRenderPassFn>,
        vkGetRenderAreaGranularity: Option<vkGetRenderAreaGranularityFn>,
        vkCreateCommandPool: Option<vkCreateCommandPoolFn>,
        vkDestroyCommandPool: Option<vkDestroyCommandPoolFn>,
        vkResetCommandPool: Option<vkResetCommandPoolFn>,
        vkAllocateCommandBuffers: Option<vkAllocateCommandBuffersFn>,
        vkFreeCommandBuffers: Option<vkFreeCommandBuffersFn>,
        vkBeginCommandBuffer: Option<vkBeginCommandBufferFn>,
        vkEndCommandBuffer: Option<vkEndCommandBufferFn>,
        vkResetCommandBuffer: Option<vkResetCommandBufferFn>,
        vkCmdBindPipeline: Option<vkCmdBindPipelineFn>,
        vkCmdSetViewport: Option<vkCmdSetViewportFn>,
        vkCmdSetScissor: Option<vkCmdSetScissorFn>,
        vkCmdSetLineWidth: Option<vkCmdSetLineWidthFn>,
        vkCmdSetDepthBias: Option<vkCmdSetDepthBiasFn>,
        vkCmdSetBlendConstants: Option<vkCmdSetBlendConstantsFn>,
        vkCmdSetDepthBounds: Option<vkCmdSetDepthBoundsFn>,
        vkCmdSetStencilCompareMask: Option<vkCmdSetStencilCompareMaskFn>,
        vkCmdSetStencilWriteMask: Option<vkCmdSetStencilWriteMaskFn>,
        vkCmdSetStencilReference: Option<vkCmdSetStencilReferenceFn>,
        vkCmdBindDescriptorSets: Option<vkCmdBindDescriptorSetsFn>,
        vkCmdBindIndexBuffer: Option<vkCmdBindIndexBufferFn>,
        vkCmdBindVertexBuffers: Option<vkCmdBindVertexBuffersFn>,
        vkCmdDraw: Option<vkCmdDrawFn>,
        vkCmdDrawIndexed: Option<vkCmdDrawIndexedFn>,
        vkCmdDrawIndirect: Option<vkCmdDrawIndirectFn>,
        vkCmdDrawIndexedIndirect: Option<vkCmdDrawIndexedIndirectFn>,
        vkCmdDispatch: Option<vkCmdDispatchFn>,
        vkCmdDispatchIndirect: Option<vkCmdDispatchIndirectFn>,
        vkCmdCopyBuffer: Option<vkCmdCopyBufferFn>,
        vkCmdCopyImage: Option<vkCmdCopyImageFn>,
        vkCmdBlitImage: Option<vkCmdBlitImageFn>,
        vkCmdCopyBufferToImage: Option<vkCmdCopyBufferToImageFn>,
        vkCmdCopyImageToBuffer: Option<vkCmdCopyImageToBufferFn>,
        vkCmdUpdateBuffer: Option<vkCmdUpdateBufferFn>,
        vkCmdFillBuffer: Option<vkCmdFillBufferFn>,
        vkCmdClearColorImage: Option<vkCmdClearColorImageFn>,
        vkCmdClearDepthStencilImage: Option<vkCmdClearDepthStencilImageFn>,
        vkCmdClearAttachments: Option<vkCmdClearAttachmentsFn>,
        vkCmdResolveImage: Option<vkCmdResolveImageFn>,
        vkCmdSetEvent: Option<vkCmdSetEventFn>,
        vkCmdResetEvent: Option<vkCmdResetEventFn>,
        vkCmdWaitEvents: Option<vkCmdWaitEventsFn>,
        vkCmdPipelineBarrier: Option<vkCmdPipelineBarrierFn>,
        vkCmdBeginQuery: Option<vkCmdBeginQueryFn>,
        vkCmdEndQuery: Option<vkCmdEndQueryFn>,
        vkCmdResetQueryPool: Option<vkCmdResetQueryPoolFn>,
        vkCmdWriteTimestamp: Option<vkCmdWriteTimestampFn>,
        vkCmdCopyQueryPoolResults: Option<vkCmdCopyQueryPoolResultsFn>,
        vkCmdPushConstants: Option<vkCmdPushConstantsFn>,
        vkCmdBeginRenderPass: Option<vkCmdBeginRenderPassFn>,
        vkCmdNextSubpass: Option<vkCmdNextSubpassFn>,
        vkCmdEndRenderPass: Option<vkCmdEndRenderPassFn>,
        vkCmdExecuteCommands: Option<vkCmdExecuteCommandsFn>,
    }

    impl VkCoreCommands {
        pub fn new() -> Result<VkCoreCommands, String> {
            unsafe {
                let mut vulkan_core: VkCoreCommands;
                vulkan_core = ::std::mem::zeroed::<VkCoreCommands>();
                let library_path = Path::new(VULKAN_LIBRARY);
                vulkan_core.library = match DynamicLibrary::open(Some(library_path)) {
                    Err(error) => return Err(format!("Failed to load {}: {}", VULKAN_LIBRARY, error)),
                    Ok(library) => Some(library),
                };
                // Only vkGetInstanceProcAddr is guaranteed to be exported by the library
                vulkan_core.vkGetInstanceProcAddr = Some(transmute(try!(vulkan_core.library.as_ref().unwrap().symbol::<u8>("vkGetInstanceProcAddr"))));
                // Load global commands via vkGetInstanceProcAddr
                vulkan_core.vkCreateInstance = Some(transmute(load_command!(vulkan_core, VkInstance::null(), "vkCreateInstance")));
                vulkan_core.vkEnumerateInstanceExtensionProperties = Some(transmute(load_command!(vulkan_core, VkInstance::null(), "vkEnumerateInstanceExtensionProperties")));
                vulkan_core.vkEnumerateInstanceLayerProperties = Some(transmute(load_command!(vulkan_core, VkInstance::null(), "vkEnumerateInstanceLayerProperties")));
                Ok(vulkan_core)
            }
        }

        pub fn load(&mut self, instance: VkInstance) -> Result<(), String> {
            unsafe {
                //self.vkCreateInstance = Some(transmute(load_command!(self, instance, "vkCreateInstance")));
                self.vkDestroyInstance = Some(transmute(load_command!(self, instance, "vkDestroyInstance")));
                self.vkEnumeratePhysicalDevices = Some(transmute(load_command!(self, instance, "vkEnumeratePhysicalDevices")));
                self.vkGetPhysicalDeviceFeatures = Some(transmute(load_command!(self, instance, "vkGetPhysicalDeviceFeatures")));
                self.vkGetPhysicalDeviceFormatProperties = Some(transmute(load_command!(self, instance, "vkGetPhysicalDeviceFormatProperties")));
                self.vkGetPhysicalDeviceImageFormatProperties = Some(transmute(load_command!(self, instance, "vkGetPhysicalDeviceImageFormatProperties")));
                self.vkGetPhysicalDeviceProperties = Some(transmute(load_command!(self, instance, "vkGetPhysicalDeviceProperties")));
                self.vkGetPhysicalDeviceQueueFamilyProperties = Some(transmute(load_command!(self, instance, "vkGetPhysicalDeviceQueueFamilyProperties")));
                self.vkGetPhysicalDeviceMemoryProperties = Some(transmute(load_command!(self, instance, "vkGetPhysicalDeviceMemoryProperties")));
                self.vkGetInstanceProcAddr = Some(transmute(load_command!(self, instance, "vkGetInstanceProcAddr")));
                self.vkGetDeviceProcAddr = Some(transmute(load_command!(self, instance, "vkGetDeviceProcAddr")));
                self.vkCreateDevice = Some(transmute(load_command!(self, instance, "vkCreateDevice")));
                self.vkDestroyDevice = Some(transmute(load_command!(self, instance, "vkDestroyDevice")));
                //self.vkEnumerateInstanceExtensionProperties = Some(transmute(load_command!(self, instance, "vkEnumerateInstanceExtensionProperties")));
                self.vkEnumerateDeviceExtensionProperties = Some(transmute(load_command!(self, instance, "vkEnumerateDeviceExtensionProperties")));
                //self.vkEnumerateInstanceLayerProperties = Some(transmute(load_command!(self, instance, "vkEnumerateInstanceLayerProperties")));
                self.vkEnumerateDeviceLayerProperties = Some(transmute(load_command!(self, instance, "vkEnumerateDeviceLayerProperties")));
                self.vkGetDeviceQueue = Some(transmute(load_command!(self, instance, "vkGetDeviceQueue")));
                self.vkQueueSubmit = Some(transmute(load_command!(self, instance, "vkQueueSubmit")));
                self.vkQueueWaitIdle = Some(transmute(load_command!(self, instance, "vkQueueWaitIdle")));
                self.vkDeviceWaitIdle = Some(transmute(load_command!(self, instance, "vkDeviceWaitIdle")));
                self.vkAllocateMemory = Some(transmute(load_command!(self, instance, "vkAllocateMemory")));
                self.vkFreeMemory = Some(transmute(load_command!(self, instance, "vkFreeMemory")));
                self.vkMapMemory = Some(transmute(load_command!(self, instance, "vkMapMemory")));
                self.vkUnmapMemory = Some(transmute(load_command!(self, instance, "vkUnmapMemory")));
                self.vkFlushMappedMemoryRanges = Some(transmute(load_command!(self, instance, "vkFlushMappedMemoryRanges")));
                self.vkInvalidateMappedMemoryRanges = Some(transmute(load_command!(self, instance, "vkInvalidateMappedMemoryRanges")));
                self.vkGetDeviceMemoryCommitment = Some(transmute(load_command!(self, instance, "vkGetDeviceMemoryCommitment")));
                self.vkBindBufferMemory = Some(transmute(load_command!(self, instance, "vkBindBufferMemory")));
                self.vkBindImageMemory = Some(transmute(load_command!(self, instance, "vkBindImageMemory")));
                self.vkGetBufferMemoryRequirements = Some(transmute(load_command!(self, instance, "vkGetBufferMemoryRequirements")));
                self.vkGetImageMemoryRequirements = Some(transmute(load_command!(self, instance, "vkGetImageMemoryRequirements")));
                self.vkGetImageSparseMemoryRequirements = Some(transmute(load_command!(self, instance, "vkGetImageSparseMemoryRequirements")));
                self.vkGetPhysicalDeviceSparseImageFormatProperties = Some(transmute(load_command!(self, instance, "vkGetPhysicalDeviceSparseImageFormatProperties")));
                self.vkQueueBindSparse = Some(transmute(load_command!(self, instance, "vkQueueBindSparse")));
                self.vkCreateFence = Some(transmute(load_command!(self, instance, "vkCreateFence")));
                self.vkDestroyFence = Some(transmute(load_command!(self, instance, "vkDestroyFence")));
                self.vkResetFences = Some(transmute(load_command!(self, instance, "vkResetFences")));
                self.vkGetFenceStatus = Some(transmute(load_command!(self, instance, "vkGetFenceStatus")));
                self.vkWaitForFences = Some(transmute(load_command!(self, instance, "vkWaitForFences")));
                self.vkCreateSemaphore = Some(transmute(load_command!(self, instance, "vkCreateSemaphore")));
                self.vkDestroySemaphore = Some(transmute(load_command!(self, instance, "vkDestroySemaphore")));
                self.vkCreateEvent = Some(transmute(load_command!(self, instance, "vkCreateEvent")));
                self.vkDestroyEvent = Some(transmute(load_command!(self, instance, "vkDestroyEvent")));
                self.vkGetEventStatus = Some(transmute(load_command!(self, instance, "vkGetEventStatus")));
                self.vkSetEvent = Some(transmute(load_command!(self, instance, "vkSetEvent")));
                self.vkResetEvent = Some(transmute(load_command!(self, instance, "vkResetEvent")));
                self.vkCreateQueryPool = Some(transmute(load_command!(self, instance, "vkCreateQueryPool")));
                self.vkDestroyQueryPool = Some(transmute(load_command!(self, instance, "vkDestroyQueryPool")));
                self.vkGetQueryPoolResults = Some(transmute(load_command!(self, instance, "vkGetQueryPoolResults")));
                self.vkCreateBuffer = Some(transmute(load_command!(self, instance, "vkCreateBuffer")));
                self.vkDestroyBuffer = Some(transmute(load_command!(self, instance, "vkDestroyBuffer")));
                self.vkCreateBufferView = Some(transmute(load_command!(self, instance, "vkCreateBufferView")));
                self.vkDestroyBufferView = Some(transmute(load_command!(self, instance, "vkDestroyBufferView")));
                self.vkCreateImage = Some(transmute(load_command!(self, instance, "vkCreateImage")));
                self.vkDestroyImage = Some(transmute(load_command!(self, instance, "vkDestroyImage")));
                self.vkGetImageSubresourceLayout = Some(transmute(load_command!(self, instance, "vkGetImageSubresourceLayout")));
                self.vkCreateImageView = Some(transmute(load_command!(self, instance, "vkCreateImageView")));
                self.vkDestroyImageView = Some(transmute(load_command!(self, instance, "vkDestroyImageView")));
                self.vkCreateShaderModule = Some(transmute(load_command!(self, instance, "vkCreateShaderModule")));
                self.vkDestroyShaderModule = Some(transmute(load_command!(self, instance, "vkDestroyShaderModule")));
                self.vkCreatePipelineCache = Some(transmute(load_command!(self, instance, "vkCreatePipelineCache")));
                self.vkDestroyPipelineCache = Some(transmute(load_command!(self, instance, "vkDestroyPipelineCache")));
                self.vkGetPipelineCacheData = Some(transmute(load_command!(self, instance, "vkGetPipelineCacheData")));
                self.vkMergePipelineCaches = Some(transmute(load_command!(self, instance, "vkMergePipelineCaches")));
                self.vkCreateGraphicsPipelines = Some(transmute(load_command!(self, instance, "vkCreateGraphicsPipelines")));
                self.vkCreateComputePipelines = Some(transmute(load_command!(self, instance, "vkCreateComputePipelines")));
                self.vkDestroyPipeline = Some(transmute(load_command!(self, instance, "vkDestroyPipeline")));
                self.vkCreatePipelineLayout = Some(transmute(load_command!(self, instance, "vkCreatePipelineLayout")));
                self.vkDestroyPipelineLayout = Some(transmute(load_command!(self, instance, "vkDestroyPipelineLayout")));
                self.vkCreateSampler = Some(transmute(load_command!(self, instance, "vkCreateSampler")));
                self.vkDestroySampler = Some(transmute(load_command!(self, instance, "vkDestroySampler")));
                self.vkCreateDescriptorSetLayout = Some(transmute(load_command!(self, instance, "vkCreateDescriptorSetLayout")));
                self.vkDestroyDescriptorSetLayout = Some(transmute(load_command!(self, instance, "vkDestroyDescriptorSetLayout")));
                self.vkCreateDescriptorPool = Some(transmute(load_command!(self, instance, "vkCreateDescriptorPool")));
                self.vkDestroyDescriptorPool = Some(transmute(load_command!(self, instance, "vkDestroyDescriptorPool")));
                self.vkResetDescriptorPool = Some(transmute(load_command!(self, instance, "vkResetDescriptorPool")));
                self.vkAllocateDescriptorSets = Some(transmute(load_command!(self, instance, "vkAllocateDescriptorSets")));
                self.vkFreeDescriptorSets = Some(transmute(load_command!(self, instance, "vkFreeDescriptorSets")));
                self.vkUpdateDescriptorSets = Some(transmute(load_command!(self, instance, "vkUpdateDescriptorSets")));
                self.vkCreateFramebuffer = Some(transmute(load_command!(self, instance, "vkCreateFramebuffer")));
                self.vkDestroyFramebuffer = Some(transmute(load_command!(self, instance, "vkDestroyFramebuffer")));
                self.vkCreateRenderPass = Some(transmute(load_command!(self, instance, "vkCreateRenderPass")));
                self.vkDestroyRenderPass = Some(transmute(load_command!(self, instance, "vkDestroyRenderPass")));
                self.vkGetRenderAreaGranularity = Some(transmute(load_command!(self, instance, "vkGetRenderAreaGranularity")));
                self.vkCreateCommandPool = Some(transmute(load_command!(self, instance, "vkCreateCommandPool")));
                self.vkDestroyCommandPool = Some(transmute(load_command!(self, instance, "vkDestroyCommandPool")));
                self.vkResetCommandPool = Some(transmute(load_command!(self, instance, "vkResetCommandPool")));
                self.vkAllocateCommandBuffers = Some(transmute(load_command!(self, instance, "vkAllocateCommandBuffers")));
                self.vkFreeCommandBuffers = Some(transmute(load_command!(self, instance, "vkFreeCommandBuffers")));
                self.vkBeginCommandBuffer = Some(transmute(load_command!(self, instance, "vkBeginCommandBuffer")));
                self.vkEndCommandBuffer = Some(transmute(load_command!(self, instance, "vkEndCommandBuffer")));
                self.vkResetCommandBuffer = Some(transmute(load_command!(self, instance, "vkResetCommandBuffer")));
                self.vkCmdBindPipeline = Some(transmute(load_command!(self, instance, "vkCmdBindPipeline")));
                self.vkCmdSetViewport = Some(transmute(load_command!(self, instance, "vkCmdSetViewport")));
                self.vkCmdSetScissor = Some(transmute(load_command!(self, instance, "vkCmdSetScissor")));
                self.vkCmdSetLineWidth = Some(transmute(load_command!(self, instance, "vkCmdSetLineWidth")));
                self.vkCmdSetDepthBias = Some(transmute(load_command!(self, instance, "vkCmdSetDepthBias")));
                self.vkCmdSetBlendConstants = Some(transmute(load_command!(self, instance, "vkCmdSetBlendConstants")));
                self.vkCmdSetDepthBounds = Some(transmute(load_command!(self, instance, "vkCmdSetDepthBounds")));
                self.vkCmdSetStencilCompareMask = Some(transmute(load_command!(self, instance, "vkCmdSetStencilCompareMask")));
                self.vkCmdSetStencilWriteMask = Some(transmute(load_command!(self, instance, "vkCmdSetStencilWriteMask")));
                self.vkCmdSetStencilReference = Some(transmute(load_command!(self, instance, "vkCmdSetStencilReference")));
                self.vkCmdBindDescriptorSets = Some(transmute(load_command!(self, instance, "vkCmdBindDescriptorSets")));
                self.vkCmdBindIndexBuffer = Some(transmute(load_command!(self, instance, "vkCmdBindIndexBuffer")));
                self.vkCmdBindVertexBuffers = Some(transmute(load_command!(self, instance, "vkCmdBindVertexBuffers")));
                self.vkCmdDraw = Some(transmute(load_command!(self, instance, "vkCmdDraw")));
                self.vkCmdDrawIndexed = Some(transmute(load_command!(self, instance, "vkCmdDrawIndexed")));
                self.vkCmdDrawIndirect = Some(transmute(load_command!(self, instance, "vkCmdDrawIndirect")));
                self.vkCmdDrawIndexedIndirect = Some(transmute(load_command!(self, instance, "vkCmdDrawIndexedIndirect")));
                self.vkCmdDispatch = Some(transmute(load_command!(self, instance, "vkCmdDispatch")));
                self.vkCmdDispatchIndirect = Some(transmute(load_command!(self, instance, "vkCmdDispatchIndirect")));
                self.vkCmdCopyBuffer = Some(transmute(load_command!(self, instance, "vkCmdCopyBuffer")));
                self.vkCmdCopyImage = Some(transmute(load_command!(self, instance, "vkCmdCopyImage")));
                self.vkCmdBlitImage = Some(transmute(load_command!(self, instance, "vkCmdBlitImage")));
                self.vkCmdCopyBufferToImage = Some(transmute(load_command!(self, instance, "vkCmdCopyBufferToImage")));
                self.vkCmdCopyImageToBuffer = Some(transmute(load_command!(self, instance, "vkCmdCopyImageToBuffer")));
                self.vkCmdUpdateBuffer = Some(transmute(load_command!(self, instance, "vkCmdUpdateBuffer")));
                self.vkCmdFillBuffer = Some(transmute(load_command!(self, instance, "vkCmdFillBuffer")));
                self.vkCmdClearColorImage = Some(transmute(load_command!(self, instance, "vkCmdClearColorImage")));
                self.vkCmdClearDepthStencilImage = Some(transmute(load_command!(self, instance, "vkCmdClearDepthStencilImage")));
                self.vkCmdClearAttachments = Some(transmute(load_command!(self, instance, "vkCmdClearAttachments")));
                self.vkCmdResolveImage = Some(transmute(load_command!(self, instance, "vkCmdResolveImage")));
                self.vkCmdSetEvent = Some(transmute(load_command!(self, instance, "vkCmdSetEvent")));
                self.vkCmdResetEvent = Some(transmute(load_command!(self, instance, "vkCmdResetEvent")));
                self.vkCmdWaitEvents = Some(transmute(load_command!(self, instance, "vkCmdWaitEvents")));
                self.vkCmdPipelineBarrier = Some(transmute(load_command!(self, instance, "vkCmdPipelineBarrier")));
                self.vkCmdBeginQuery = Some(transmute(load_command!(self, instance, "vkCmdBeginQuery")));
                self.vkCmdEndQuery = Some(transmute(load_command!(self, instance, "vkCmdEndQuery")));
                self.vkCmdResetQueryPool = Some(transmute(load_command!(self, instance, "vkCmdResetQueryPool")));
                self.vkCmdWriteTimestamp = Some(transmute(load_command!(self, instance, "vkCmdWriteTimestamp")));
                self.vkCmdCopyQueryPoolResults = Some(transmute(load_command!(self, instance, "vkCmdCopyQueryPoolResults")));
                self.vkCmdPushConstants = Some(transmute(load_command!(self, instance, "vkCmdPushConstants")));
                self.vkCmdBeginRenderPass = Some(transmute(load_command!(self, instance, "vkCmdBeginRenderPass")));
                self.vkCmdNextSubpass = Some(transmute(load_command!(self, instance, "vkCmdNextSubpass")));
                self.vkCmdEndRenderPass = Some(transmute(load_command!(self, instance, "vkCmdEndRenderPass")));
                self.vkCmdExecuteCommands = Some(transmute(load_command!(self, instance, "vkCmdExecuteCommands")));
            }
            Ok(())
        }

        pub unsafe fn vkCreateInstance(&self, pCreateInfo: *const VkInstanceCreateInfo, pAllocator: *const VkAllocationCallbacks, pInstance: *mut VkInstance) -> VkResult {
            invoke_command!(self, vkCreateInstance, pCreateInfo, pAllocator, pInstance)
        }

        pub unsafe fn vkDestroyInstance(&self, instance: VkInstance, pAllocator: *const VkAllocationCallbacks) {
            invoke_command!(self, vkDestroyInstance, instance, pAllocator)
        }

        pub unsafe fn vkEnumeratePhysicalDevices(&self, instance: VkInstance, pPhysicalDeviceCount: *mut uint32_t, pPhysicalDevices: *mut VkPhysicalDevice) -> VkResult {
            invoke_command!(self, vkEnumeratePhysicalDevices, instance, pPhysicalDeviceCount, pPhysicalDevices)
        }

        pub unsafe fn vkGetPhysicalDeviceFeatures(&self, physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures) {
            invoke_command!(self, vkGetPhysicalDeviceFeatures, physicalDevice, pFeatures)
        }

        pub unsafe fn vkGetPhysicalDeviceFormatProperties(&self, physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties) {
            invoke_command!(self, vkGetPhysicalDeviceFormatProperties, physicalDevice, format, pFormatProperties)
        }

        pub unsafe fn vkGetPhysicalDeviceImageFormatProperties(&self, physicalDevice: VkPhysicalDevice, format: VkFormat, iType: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags, pImageFormatProperties: *mut VkImageFormatProperties) -> VkResult {
            invoke_command!(self, vkGetPhysicalDeviceImageFormatProperties, physicalDevice, format, iType, tiling, usage, flags, pImageFormatProperties)
        }

        pub unsafe fn vkGetPhysicalDeviceProperties(&self, physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties) {
            invoke_command!(self, vkGetPhysicalDeviceProperties, physicalDevice, pProperties)
        }

        pub unsafe fn vkGetPhysicalDeviceQueueFamilyProperties(&self, physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut uint32_t, pQueueFamilyProperties: *mut VkQueueFamilyProperties) {
            invoke_command!(self, vkGetPhysicalDeviceQueueFamilyProperties, physicalDevice, pQueueFamilyPropertyCount, pQueueFamilyProperties)
        }

        pub unsafe fn vkGetPhysicalDeviceMemoryProperties(&self, physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties) {
            invoke_command!(self, vkGetPhysicalDeviceMemoryProperties, physicalDevice, pMemoryProperties)
        }

        pub unsafe fn vkGetInstanceProcAddr(&self, instance: VkInstance, pName: *const c_char) -> vkVoidFunctionFn {
            invoke_command!(self, vkGetInstanceProcAddr, instance, pName)
        }

        pub unsafe fn vkGetDeviceProcAddr(&self, device: VkDevice, pName: *const c_char) -> vkVoidFunctionFn {
            invoke_command!(self, vkGetDeviceProcAddr, device, pName)
        }

        pub unsafe fn vkCreateDevice(&self, physicalDevice: VkPhysicalDevice, pCreateInfo: *const VkDeviceCreateInfo, pAllocator: *const VkAllocationCallbacks, pDevice: *mut VkDevice) -> VkResult {
            invoke_command!(self, vkCreateDevice, physicalDevice, pCreateInfo, pAllocator, pDevice)
        }

        pub unsafe fn vkDestroyDevice(&self, device: VkDevice, pAllocator: *const VkAllocationCallbacks) {
            invoke_command!(self, vkDestroyDevice, device, pAllocator)
        }

        pub unsafe fn vkEnumerateInstanceExtensionProperties(&self, pLayerName: *const c_char, pPropertyCount: *mut uint32_t, pProperties: *mut VkExtensionProperties) -> VkResult {
            invoke_command!(self, vkEnumerateInstanceExtensionProperties, pLayerName, pPropertyCount, pProperties)
        }

        pub unsafe fn vkEnumerateDeviceExtensionProperties(&self, physicalDevice: VkPhysicalDevice, pLayerName: *const c_char, pPropertyCount: *mut uint32_t, pProperties: *mut VkExtensionProperties) -> VkResult {
            invoke_command!(self, vkEnumerateDeviceExtensionProperties, physicalDevice, pLayerName, pPropertyCount, pProperties)
        }

        pub unsafe fn vkEnumerateInstanceLayerProperties(&self, pPropertyCount: *mut uint32_t, pProperties: *mut VkLayerProperties) -> VkResult {
            invoke_command!(self, vkEnumerateInstanceLayerProperties, pPropertyCount, pProperties)
        }

        pub unsafe fn vkEnumerateDeviceLayerProperties(&self, physicalDevice: VkPhysicalDevice, pPropertyCount: *mut uint32_t, pProperties: *mut VkLayerProperties) -> VkResult {
            invoke_command!(self, vkEnumerateDeviceLayerProperties, physicalDevice, pPropertyCount, pProperties)
        }

        pub unsafe fn vkGetDeviceQueue(&self, device: VkDevice, queueFamilyIndex: uint32_t, queueIndex: uint32_t, pQueue: *mut VkQueue) {
            invoke_command!(self, vkGetDeviceQueue, device, queueFamilyIndex, queueIndex, pQueue)
        }

        pub unsafe fn vkQueueSubmit(&self, queue: VkQueue, submitCount: uint32_t, pSubmits: *const VkSubmitInfo, fence: VkFence) -> VkResult {
            invoke_command!(self, vkQueueSubmit, queue, submitCount, pSubmits, fence)
        }

        pub unsafe fn vkQueueWaitIdle(&self, queue: VkQueue) -> VkResult {
            invoke_command!(self, vkQueueWaitIdle, queue)
        }

        pub unsafe fn vkDeviceWaitIdle(&self, device: VkDevice) -> VkResult {
            invoke_command!(self, vkDeviceWaitIdle, device)
        }

        pub unsafe fn vkAllocateMemory(&self, device: VkDevice, pAllocateInfo: *const VkMemoryAllocateInfo, pAllocator: *const VkAllocationCallbacks, pMemory: *mut VkDeviceMemory) -> VkResult {
            invoke_command!(self, vkAllocateMemory, device, pAllocateInfo, pAllocator, pMemory)
        }

        pub unsafe fn vkFreeMemory(&self, device: VkDevice, memory: VkDeviceMemory, pAllocator: *const VkAllocationCallbacks) {
            invoke_command!(self, vkFreeMemory, device, memory, pAllocator)
        }

        pub unsafe fn vkMapMemory(&self, device: VkDevice, memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize, flags: VkMemoryMapFlags, ppData: *mut *mut c_void) -> VkResult {
            invoke_command!(self, vkMapMemory, device, memory, offset, size, flags, ppData)
        }

        pub unsafe fn vkUnmapMemory(&self, device: VkDevice, memory: VkDeviceMemory) {
            invoke_command!(self, vkUnmapMemory, device, memory)
        }

        pub unsafe fn vkFlushMappedMemoryRanges(&self, device: VkDevice, memoryRangeCount: uint32_t, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult {
            invoke_command!(self, vkFlushMappedMemoryRanges, device, memoryRangeCount, pMemoryRanges)
        }

        pub unsafe fn vkInvalidateMappedMemoryRanges(&self, device: VkDevice, memoryRangeCount: uint32_t, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult {
            invoke_command!(self, vkInvalidateMappedMemoryRanges, device, memoryRangeCount, pMemoryRanges)
        }

        pub unsafe fn vkGetDeviceMemoryCommitment(&self, device: VkDevice, memory: VkDeviceMemory, pCommittedMemoryInBytes: *mut VkDeviceSize) {
            invoke_command!(self, vkGetDeviceMemoryCommitment, device, memory, pCommittedMemoryInBytes)
        }

        pub unsafe fn vkBindBufferMemory(&self, device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult {
            invoke_command!(self, vkBindBufferMemory, device, buffer, memory, memoryOffset)
        }

        pub unsafe fn vkBindImageMemory(&self, device: VkDevice, image: VkImage, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult {
            invoke_command!(self, vkBindImageMemory, device, image, memory, memoryOffset)
        }

        pub unsafe fn vkGetBufferMemoryRequirements(&self, device: VkDevice, buffer: VkBuffer, pMemoryRequirements: *mut VkMemoryRequirements) {
            invoke_command!(self, vkGetBufferMemoryRequirements, device, buffer, pMemoryRequirements)
        }

        pub unsafe fn vkGetImageMemoryRequirements(&self, device: VkDevice, image: VkImage, pMemoryRequirements: *mut VkMemoryRequirements) {
            invoke_command!(self, vkGetImageMemoryRequirements, device, image, pMemoryRequirements)
        }

        pub unsafe fn vkGetImageSparseMemoryRequirements(&self, device: VkDevice, image: VkImage, pSparseMemoryRequirementCount: *mut uint32_t, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements) {
            invoke_command!(self, vkGetImageSparseMemoryRequirements, device, image, pSparseMemoryRequirementCount, pSparseMemoryRequirements)
        }

        pub unsafe fn vkGetPhysicalDeviceSparseImageFormatProperties(&self, physicalDevice: VkPhysicalDevice, format: VkFormat, iType: VkImageType, samples: VkSampleCountFlags, usage: VkImageUsageFlags, tiling: VkImageTiling, pPropertyCount: *mut uint32_t, pProperties: *mut VkSparseImageFormatProperties) {
            invoke_command!(self, vkGetPhysicalDeviceSparseImageFormatProperties, physicalDevice, format, iType, samples, usage, tiling, pPropertyCount, pProperties)
        }
        pub unsafe fn vkQueueBindSparse(&self, queue: VkQueue, bindInfoCount: uint32_t, pBindInfo: *const VkBindSparseInfo, fence: VkFence) -> VkResult {
            invoke_command!(self, vkQueueBindSparse, queue, bindInfoCount, pBindInfo, fence)
        }

        pub unsafe fn vkCreateFence(&self, device: VkDevice, pCreateInfo: *const VkFenceCreateInfo, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult {
            invoke_command!(self, vkCreateFence, device, pCreateInfo, pAllocator, pFence)
        }

        pub unsafe fn vkDestroyFence(&self, device: VkDevice, fence: VkFence, pAllocator: *const VkAllocationCallbacks) {
            invoke_command!(self, vkDestroyFence, device, fence, pAllocator)
        }

        pub unsafe fn vkResetFences(&self, device: VkDevice, fenceCount: uint32_t, pFences: *const VkFence) -> VkResult {
            invoke_command!(self, vkResetFences, device, fenceCount, pFences)
        }

        pub unsafe fn vkGetFenceStatus(&self, device: VkDevice, fence: VkFence) -> VkResult {
            invoke_command!(self, vkGetFenceStatus, device, fence)
        }

        pub unsafe fn vkWaitForFences(&self, device: VkDevice, fenceCount: uint32_t, pFences: *const VkFence, waitAll: VkBool32, timeout: uint64_t) -> VkResult {
            invoke_command!(self, vkWaitForFences, device, fenceCount, pFences, waitAll, timeout)
        }

        pub unsafe fn vkCreateSemaphore(&self, device: VkDevice, pCreateInfo: *const VkSemaphoreCreateInfo, pAllocator: *const VkAllocationCallbacks, pSemaphore: *mut VkSemaphore) -> VkResult {
            invoke_command!(self, vkCreateSemaphore, device, pCreateInfo, pAllocator, pSemaphore)
        }

        pub unsafe fn vkDestroySemaphore(&self, device: VkDevice, semaphore: VkSemaphore, pAllocator: *const VkAllocationCallbacks) {
            invoke_command!(self, vkDestroySemaphore, device, semaphore, pAllocator)
        }

        pub unsafe fn vkCreateEvent(&self, device: VkDevice, pCreateInfo: *const VkEventCreateInfo, pAllocator: *const VkAllocationCallbacks, pEvent: *mut VkEvent) -> VkResult {
            invoke_command!(self, vkCreateEvent, device, pCreateInfo, pAllocator, pEvent)
        }

        pub unsafe fn vkDestroyEvent(&self, device: VkDevice, event: VkEvent, pAllocator: *const VkAllocationCallbacks) {
            invoke_command!(self, vkDestroyEvent, device, event, pAllocator)
        }

        pub unsafe fn vkGetEventStatus(&self, device: VkDevice, event: VkEvent) -> VkResult {
            invoke_command!(self, vkGetEventStatus, device, event)
        }

        pub unsafe fn vkSetEvent(&self, device: VkDevice, event: VkEvent) -> VkResult {
            invoke_command!(self, vkSetEvent, device, event)
        }

        pub unsafe fn vkResetEvent(&self, device: VkDevice, event: VkEvent) -> VkResult {
            invoke_command!(self, vkResetEvent, device, event)
        }

        pub unsafe fn vkCreateQueryPool(&self, device: VkDevice, pCreateInfo: *const VkQueryPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pQueryPool: *mut VkQueryPool) -> VkResult {
            invoke_command!(self, vkCreateQueryPool, device, pCreateInfo, pAllocator, pQueryPool)
        }

        pub unsafe fn vkDestroyQueryPool(&self, device: VkDevice, queryPool: VkQueryPool, pAllocator: *const VkAllocationCallbacks) {
            invoke_command!(self, vkDestroyQueryPool, device, queryPool, pAllocator)
        }

        pub unsafe fn vkGetQueryPoolResults(&self, device: VkDevice, queryPool: VkQueryPool, firstQuery: uint32_t, queryCount: uint32_t, dataSize: size_t, pData: *mut c_void, stride: VkDeviceSize, flags: VkDeviceSize) -> VkResult {
            invoke_command!(self, vkGetQueryPoolResults, device, queryPool, firstQuery, queryCount, dataSize, pData, stride, flags)
        }

        pub unsafe fn vkCreateBuffer(&self, device: VkDevice, pCreateInfo: *const VkBufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pBuffer: *mut VkBuffer) -> VkResult {
            invoke_command!(self, vkCreateBuffer, device, pCreateInfo, pAllocator, pBuffer)
        }

        pub unsafe fn vkDestroyBuffer(&self, device: VkDevice, buffer: VkBuffer, pAllocator: *const VkAllocationCallbacks) {
            invoke_command!(self, vkDestroyBuffer, device, buffer, pAllocator)
        }

        pub unsafe fn vkCreateBufferView(&self, device: VkDevice, pCreateInfo: *const VkBufferViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkBufferView) -> VkResult {
            invoke_command!(self, vkCreateBufferView, device, pCreateInfo, pAllocator, pView)
        }

        pub unsafe fn vkDestroyBufferView(&self, device: VkDevice, bufferView: VkBufferView, pAllocator: *const VkAllocationCallbacks) {
            invoke_command!(self, vkDestroyBufferView, device, bufferView, pAllocator)
        }

        pub unsafe fn vkCreateImage(&self, device: VkDevice, pCreateInfo: *const VkImageCreateInfo, pAllocator: *const VkAllocationCallbacks, pImage: *mut VkImage) -> VkResult {
            invoke_command!(self, vkCreateImage, device, pCreateInfo, pAllocator, pImage)
        }

        pub unsafe fn vkDestroyImage(&self, device: VkDevice, image: VkImage, pAllocator: *const VkAllocationCallbacks) {
            invoke_command!(self, vkDestroyImage, device, image, pAllocator)
        }

        pub unsafe fn vkGetImageSubresourceLayout(&self, device: VkDevice, image: VkImage, pSubresource: *const VkImageSubresource, pLayout: *mut VkSubresourceLayout) {
            invoke_command!(self, vkGetImageSubresourceLayout, device, image, pSubresource, pLayout)
        }

        pub unsafe fn vkCreateImageView(&self, device: VkDevice, pCreateInfo: *const VkImageViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkImageView) -> VkResult {
            invoke_command!(self, vkCreateImageView, device, pCreateInfo, pAllocator, pView)
        }

        pub unsafe fn vkDestroyImageView(&self, device: VkDevice, imageView: VkImageView, pAllocator: *const VkAllocationCallbacks) {
            invoke_command!(self, vkDestroyImageView, device, imageView, pAllocator)
        }

        pub unsafe fn vkCreateShaderModule(&self, device: VkDevice, pCreateInfo: *const VkShaderModuleCreateInfo, pAllocator: *const VkAllocationCallbacks, pShaderModule: *mut VkShaderModule) -> VkResult {
            invoke_command!(self, vkCreateShaderModule, device, pCreateInfo, pAllocator, pShaderModule)
        }

        pub unsafe fn vkDestroyShaderModule(&self, device: VkDevice, shaderModule: VkShaderModule, pAllocator: *const VkAllocationCallbacks) {
            invoke_command!(self, vkDestroyShaderModule, device, shaderModule, pAllocator)
        }

        pub unsafe fn vkCreatePipelineCache(&self, device: VkDevice, pCreateInfo: *const VkPipelineCacheCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineCache: *mut VkPipelineCache) -> VkResult {
            invoke_command!(self, vkCreatePipelineCache, device, pCreateInfo, pAllocator, pPipelineCache)
        }

        pub unsafe fn vkDestroyPipelineCache(&self, device: VkDevice, pipelineCache: VkPipelineCache, pAllocator: *const VkAllocationCallbacks) {
            invoke_command!(self, vkDestroyPipelineCache, device, pipelineCache, pAllocator)
        }

        pub unsafe fn vkGetPipelineCacheData(&self, device: VkDevice, pipelineCache: VkPipelineCache, pDataSize: *mut size_t, pData: *mut c_void) -> VkResult {
            invoke_command!(self, vkGetPipelineCacheData, device, pipelineCache, pDataSize, pData)
        }

        pub unsafe fn vkMergePipelineCaches(&self, device: VkDevice, dstCache: VkPipelineCache, srcCacheCount: uint32_t, pSrcCaches: *const VkPipelineCache) -> VkResult {
            invoke_command!(self, vkMergePipelineCaches, device, dstCache, srcCacheCount, pSrcCaches)
        }

        pub unsafe fn vkCreateGraphicsPipelines(&self, device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: uint32_t, pCreateInfos: *const VkGraphicsPipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult {
            invoke_command!(self, vkCreateGraphicsPipelines, device, pipelineCache, createInfoCount, pCreateInfos, pAllocator, pPipelines)
        }

        pub unsafe fn vkCreateComputePipelines(&self, device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: uint32_t, pCreateInfos: *const VkComputePipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult {
            invoke_command!(self, vkCreateComputePipelines, device, pipelineCache, createInfoCount, pCreateInfos, pAllocator, pPipelines)
        }

        pub unsafe fn vkDestroyPipeline(&self, device: VkDevice, pipeline: VkPipeline, pAllocator: *const VkAllocationCallbacks) {
            invoke_command!(self, vkDestroyPipeline, device, pipeline, pAllocator)
        }

        pub unsafe fn vkCreatePipelineLayout(&self, device: VkDevice, pCreateInfo: *const VkPipelineLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineLayout: *mut VkPipelineLayout) -> VkResult {
            invoke_command!(self, vkCreatePipelineLayout, device, pCreateInfo, pAllocator, pPipelineLayout)
        }

        pub unsafe fn vkDestroyPipelineLayout(&self, device: VkDevice, pipelineLayout: VkPipelineLayout, pAllocator: *const VkAllocationCallbacks) {
            invoke_command!(self, vkDestroyPipelineLayout, device, pipelineLayout, pAllocator)
        }

        pub unsafe fn vkCreateSampler(&self, device: VkDevice, pCreateInfo: *const VkSamplerCreateInfo, pAllocator: *const VkAllocationCallbacks, pSampler: *mut VkSampler) -> VkResult {
            invoke_command!(self, vkCreateSampler, device, pCreateInfo, pAllocator, pSampler)
        }

        pub unsafe fn vkDestroySampler(&self, device: VkDevice, sampler: VkSampler, pAllocator: *const VkAllocationCallbacks) {
            invoke_command!(self, vkDestroySampler, device, sampler, pAllocator)
        }

        pub unsafe fn vkCreateDescriptorSetLayout(&self, device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pSetLayout: *mut VkDescriptorSetLayout) -> VkResult {
            invoke_command!(self, vkCreateDescriptorSetLayout, device, pCreateInfo, pAllocator, pSetLayout)
        }

        pub unsafe fn vkDestroyDescriptorSetLayout(&self, device: VkDevice, descriptorSetLayout: VkDescriptorSetLayout, pAllocator: *const VkAllocationCallbacks) {
            invoke_command!(self, vkDestroyDescriptorSetLayout, device, descriptorSetLayout, pAllocator)
        }

        pub unsafe fn vkCreateDescriptorPool(&self, device: VkDevice, pCreateInfo: *const VkDescriptorPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pDescriptorPool: *mut VkDescriptorPool) -> VkResult {
            invoke_command!(self, vkCreateDescriptorPool, device, pCreateInfo, pAllocator, pDescriptorPool)
        }

        pub unsafe fn vkDestroyDescriptorPool(&self, device: VkDevice, descriptorPool: VkDescriptorPool, pAllocator: *const VkAllocationCallbacks) {
            invoke_command!(self, vkDestroyDescriptorPool, device, descriptorPool, pAllocator)
        }

        pub unsafe fn vkResetDescriptorPool(&self, device: VkDevice, descriptorPool: VkDescriptorPool, flags: VkDescriptorPoolResetFlags) -> VkResult {
            invoke_command!(self, vkResetDescriptorPool, device, descriptorPool, flags)
        }

        pub unsafe fn vkAllocateDescriptorSets(&self, device: VkDevice, pAllocateInfo: *const VkDescriptorSetAllocateInfo, pDescriptorSets: *mut VkDescriptorSet) -> VkResult {
            invoke_command!(self, vkAllocateDescriptorSets, device, pAllocateInfo, pDescriptorSets)
        }

        pub unsafe fn vkFreeDescriptorSets(&self, device: VkDevice, descriptorPool: VkDescriptorPool, descriptorSetCount: uint32_t, pDescriptorSets: *const VkDescriptorSet) -> VkResult {
            invoke_command!(self, vkFreeDescriptorSets, device, descriptorPool, descriptorSetCount, pDescriptorSets)
        }

        pub unsafe fn vkUpdateDescriptorSets(&self, device: VkDevice, descriptorWriteCount: uint32_t, pDescriptorWrites: *const VkWriteDescriptorSet, descriptorCopyCount: uint32_t, pDescriptorCopies: *const VkCopyDescriptorSet) {
            invoke_command!(self, vkUpdateDescriptorSets, device, descriptorWriteCount, pDescriptorWrites, descriptorCopyCount, pDescriptorCopies)
        }

        pub unsafe fn vkCreateFramebuffer(&self, device: VkDevice, pCreateInfo: *const VkFramebufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pFramebuffer: *mut VkFramebuffer) -> VkResult {
            invoke_command!(self, vkCreateFramebuffer, device, pCreateInfo, pAllocator, pFramebuffer)
        }

        pub unsafe fn vkDestroyFramebuffer(&self, device: VkDevice, framebuffer: VkFramebuffer, pAllocator: *const VkAllocationCallbacks) {
            invoke_command!(self, vkDestroyFramebuffer, device, framebuffer, pAllocator)
        }

        pub unsafe fn vkCreateRenderPass(&self, device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult {
            invoke_command!(self, vkCreateRenderPass, device, pCreateInfo, pAllocator, pRenderPass)
        }

        pub unsafe fn vkDestroyRenderPass(&self, device: VkDevice, renderPass: VkRenderPass, pAllocator: *const VkAllocationCallbacks) {
            invoke_command!(self, vkDestroyRenderPass, device, renderPass, pAllocator)
        }

        pub unsafe fn vkGetRenderAreaGranularity(&self, device: VkDevice, renderPass: VkRenderPass, pGranularity: *mut VkExtent2D) {
            invoke_command!(self, vkGetRenderAreaGranularity, device, renderPass, pGranularity)
        }

        pub unsafe fn vkCreateCommandPool(&self, device: VkDevice, pCreateInfo: *const VkCommandPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pCommandPool: *mut VkCommandPool) -> VkResult {
            invoke_command!(self, vkCreateCommandPool, device, pCreateInfo, pAllocator, pCommandPool)
        }

        pub unsafe fn vkDestroyCommandPool(&self, device: VkDevice, commandPool: VkCommandPool, pAllocator: *const VkAllocationCallbacks) {
            invoke_command!(self, vkDestroyCommandPool, device, commandPool, pAllocator)
        }

        pub unsafe fn vkResetCommandPool(&self, device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult {
            invoke_command!(self, vkResetCommandPool, device, commandPool, flags)
        }

        pub unsafe fn vkAllocateCommandBuffers(&self, device: VkDevice, pAllocateInfo: *const VkCommandBufferAllocateInfo, pCommandBuffers: *mut VkCommandBuffer) -> VkResult {
            invoke_command!(self, vkAllocateCommandBuffers, device, pAllocateInfo, pCommandBuffers)
        }

        pub unsafe fn vkFreeCommandBuffers(&self, device: VkDevice, commandPool: VkCommandPool, commandBufferCount: uint32_t, pCommandBuffers: *const VkCommandBuffer) {
            invoke_command!(self, vkFreeCommandBuffers, device, commandPool, commandBufferCount, pCommandBuffers)
        }

        pub unsafe fn vkBeginCommandBuffer(&self, commandBuffer: VkCommandBuffer, pBeginInfo: *const VkCommandBufferBeginInfo) -> VkResult {
            invoke_command!(self, vkBeginCommandBuffer, commandBuffer, pBeginInfo)
        }

        pub unsafe fn vkEndCommandBuffer(&self, commandBuffer: VkCommandBuffer) -> VkResult {
            invoke_command!(self, vkEndCommandBuffer, commandBuffer)
        }

        pub unsafe fn vkResetCommandBuffer(&self, commandBuffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult {
            invoke_command!(self, vkResetCommandBuffer, commandBuffer, flags)
        }

        pub unsafe fn vkCmdBindPipeline(&self, commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline) {
            invoke_command!(self, vkCmdBindPipeline, commandBuffer, pipelineBindPoint, pipeline)
        }

        pub unsafe fn vkCmdSetViewport(&self, commandBuffer: VkCommandBuffer, firstViewport: uint32_t, viewportCount: uint32_t, pViewports: *const VkViewport) {
            invoke_command!(self, vkCmdSetViewport, commandBuffer, firstViewport, viewportCount, pViewports)
        }

        pub unsafe fn vkCmdSetScissor(&self, commandBuffer: VkCommandBuffer, firstScissor: uint32_t, scissorCount: uint32_t, pScissors: *const VkRect2D) {
            invoke_command!(self, vkCmdSetScissor, commandBuffer, firstScissor, scissorCount, pScissors)
        }

        pub unsafe fn vkCmdSetLineWidth(&self, commandBuffer: VkCommandBuffer, lineWidth: c_float) {
            invoke_command!(self, vkCmdSetLineWidth, commandBuffer, lineWidth)
        }

        pub unsafe fn vkCmdSetDepthBias(&self, commandBuffer: VkCommandBuffer, depthBiasConstantFactor: c_float, depthBiasClamp: c_float, depthBiasSlopeFactor: c_float) {
            invoke_command!(self, vkCmdSetDepthBias, commandBuffer, depthBiasConstantFactor, depthBiasClamp, depthBiasSlopeFactor)
        }

        // TODO: make sure this is working
        pub unsafe fn vkCmdSetBlendConstants(&self, commandBuffer: VkCommandBuffer, blendConstants: [c_float; 4]) {
            invoke_command!(self, vkCmdSetBlendConstants, commandBuffer, blendConstants)
        }

        pub unsafe fn vkCmdSetDepthBounds(&self, commandBuffer: VkCommandBuffer, minDepthBounds: c_float, maxDepthBounds: c_float) {
            invoke_command!(self, vkCmdSetDepthBounds, commandBuffer, minDepthBounds, maxDepthBounds)
        }

        pub unsafe fn vkCmdSetStencilCompareMask(&self, commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, compareMask: uint32_t) {
            invoke_command!(self, vkCmdSetStencilCompareMask, commandBuffer, faceMask, compareMask)
        }

        pub unsafe fn vkCmdSetStencilWriteMask(&self, commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, writeMask: uint32_t) {
            invoke_command!(self, vkCmdSetStencilWriteMask, commandBuffer, faceMask, writeMask)
        }

        pub unsafe fn vkCmdSetStencilReference(&self, commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, reference: uint32_t) {
            invoke_command!(self, vkCmdSetStencilReference, commandBuffer, faceMask, reference)
        }

        pub unsafe fn vkCmdBindDescriptorSets(&self, commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, firstSet: uint32_t, descriptorSetCount: uint32_t, pDescriptorSets: *const VkDescriptorSet, dynamicOffsetCount: uint32_t, pDynamicOffsets: *const uint32_t) {
            invoke_command!(self, vkCmdBindDescriptorSets, commandBuffer, pipelineBindPoint, layout, firstSet, descriptorSetCount, pDescriptorSets, dynamicOffsetCount, pDynamicOffsets)
        }

        pub unsafe fn vkCmdBindIndexBuffer(&self, commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, indexType: VkIndexType) {
            invoke_command!(self, vkCmdBindIndexBuffer, commandBuffer, buffer, offset, indexType)
        }

        pub unsafe fn vkCmdBindVertexBuffers(&self, commandBuffer: VkCommandBuffer, firstBinding: uint32_t, bindingCount: uint32_t, pBuffers: *const VkBuffer, pOffsets: *const VkDeviceSize) {
            invoke_command!(self, vkCmdBindVertexBuffers, commandBuffer, firstBinding, bindingCount, pBuffers, pOffsets)
        }

        pub unsafe fn vkCmdDraw(&self, commandBuffer: VkCommandBuffer, vertexCount: uint32_t, instanceCount: uint32_t, firstVertex: uint32_t, firstInstance: uint32_t) {
            invoke_command!(self, vkCmdDraw, commandBuffer, vertexCount, instanceCount, firstVertex, firstInstance)
        }

        pub unsafe fn vkCmdDrawIndexed(&self, commandBuffer: VkCommandBuffer, indexCount: uint32_t, instanceCount: uint32_t, firstIndex: uint32_t, vertexOffset: int32_t, firstInstance: uint32_t) {
            invoke_command!(self, vkCmdDrawIndexed, commandBuffer, indexCount, instanceCount, firstIndex, vertexOffset, firstInstance)
        }

        pub unsafe fn vkCmdDrawIndirect(&self, commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: uint32_t, stride: uint32_t) {
            invoke_command!(self, vkCmdDrawIndirect, commandBuffer, buffer, offset, drawCount, stride)
        }

        pub unsafe fn vkCmdDrawIndexedIndirect(&self, commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: uint32_t, stride: uint32_t) {
            invoke_command!(self, vkCmdDrawIndexedIndirect, commandBuffer, buffer, offset, drawCount, stride)
        }

        pub unsafe fn vkCmdDispatch(&self, commandBuffer: VkCommandBuffer, x: uint32_t, y: uint32_t, z: uint32_t) {
            invoke_command!(self, vkCmdDispatch, commandBuffer, x, y, z)
        }

        pub unsafe fn vkCmdDispatchIndirect(&self, commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize) {
            invoke_command!(self, vkCmdDispatchIndirect, commandBuffer, buffer, offset)
        }

        pub unsafe fn vkCmdCopyBuffer(&self, commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstBuffer: VkBuffer, regionCount: uint32_t, pRegions: *const VkBufferCopy) {
            invoke_command!(self, vkCmdCopyBuffer, commandBuffer, srcBuffer, dstBuffer, regionCount, pRegions)
        }

        pub unsafe fn vkCmdCopyImage(&self, commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: uint32_t, pRegions: *const VkImageCopy) {
            invoke_command!(self, vkCmdCopyImage, commandBuffer, srcImage, srcImageLayout, dstImage, dstImageLayout, regionCount, pRegions)
        }

        pub unsafe fn vkCmdBlitImage(&self, commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: uint32_t, pRegions: *const VkImageBlit, filter: VkFilter) {
            invoke_command!(self, vkCmdBlitImage, commandBuffer, srcImage, srcImageLayout, dstImage, dstImageLayout, regionCount, pRegions, filter)
        }

        pub unsafe fn vkCmdCopyBufferToImage(&self, commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: uint32_t, pRegions: *const VkBufferImageCopy) {
            invoke_command!(self, vkCmdCopyBufferToImage, commandBuffer, srcBuffer, dstImage, dstImageLayout, regionCount, pRegions)
        }

        pub unsafe fn vkCmdCopyImageToBuffer(&self, commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstBuffer: VkBuffer, regionCount: uint32_t, pRegions: *const VkBufferImageCopy) {
            invoke_command!(self, vkCmdCopyImageToBuffer, commandBuffer, srcImage, srcImageLayout, dstBuffer, regionCount, pRegions)
        }

        pub unsafe fn vkCmdUpdateBuffer(&self, commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, pData: *const uint32_t) {
            invoke_command!(self, vkCmdUpdateBuffer, commandBuffer, dstBuffer, dstOffset, dataSize, pData)
        }

        pub unsafe fn vkCmdFillBuffer(&self, commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, size: VkDeviceSize, data: uint32_t) {
            invoke_command!(self, vkCmdFillBuffer, commandBuffer, dstBuffer, dstOffset, size, data)
        }

        pub unsafe fn vkCmdClearColorImage(&self, commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pColor: *const VkClearColorValue, rangeCount: uint32_t, pRanges: *const VkImageSubresourceRange) {
            invoke_command!(self, vkCmdClearColorImage, commandBuffer, image, imageLayout, pColor, rangeCount, pRanges)
        }

        pub unsafe fn vkCmdClearDepthStencilImage(&self, commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pDepthStencil: *const VkClearDepthStencilValue, rangeCount: uint32_t, pRanges: *const VkImageSubresourceRange) {
            invoke_command!(self, vkCmdClearDepthStencilImage, commandBuffer, image, imageLayout, pDepthStencil, rangeCount, pRanges)
        }

        pub unsafe fn vkCmdClearAttachments(&self, commandBuffer: VkCommandBuffer, attachmentCount: uint32_t, pAttachments: *const VkClearAttachment, rectCount: uint32_t, pRects: *const VkClearRect) {
            invoke_command!(self, vkCmdClearAttachments, commandBuffer, attachmentCount, pAttachments, rectCount, pRects)
        }

        pub unsafe fn vkCmdResolveImage(&self, commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: uint32_t, pRegions: *const VkImageResolve) {
            invoke_command!(self, vkCmdResolveImage, commandBuffer, srcImage, srcImageLayout, dstImage, dstImageLayout, regionCount, pRegions)
        }

        pub unsafe fn vkCmdSetEvent(&self, commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags) {
            invoke_command!(self, vkCmdSetEvent, commandBuffer, event, stageMask)
        }

        pub unsafe fn vkCmdResetEvent(&self, commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags) {
            invoke_command!(self, vkCmdResetEvent, commandBuffer, event, stageMask)
        }

        pub unsafe fn vkCmdWaitEvents(&self, commandBuffer: VkCommandBuffer, eventCount: uint32_t, pEvents: *const VkEvent, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, memoryBarrierCount: uint32_t, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: uint32_t, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: uint32_t, pImageMemoryBarriers: *const VkImageMemoryBarrier) {
            invoke_command!(self, vkCmdWaitEvents, commandBuffer, eventCount, pEvents, srcStageMask, dstStageMask, memoryBarrierCount, pMemoryBarriers, bufferMemoryBarrierCount, pBufferMemoryBarriers, imageMemoryBarrierCount, pImageMemoryBarriers)
        }

        pub unsafe fn vkCmdPipelineBarrier(&self, commandBuffer: VkCommandBuffer, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, dependencyFlags: VkDependencyFlags, memoryBarrierCount: uint32_t, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: uint32_t, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: uint32_t, pImageMemoryBarriers: *const VkImageMemoryBarrier) {
            invoke_command!(self, vkCmdPipelineBarrier, commandBuffer, srcStageMask, dstStageMask, dependencyFlags, memoryBarrierCount, pMemoryBarriers, bufferMemoryBarrierCount, pBufferMemoryBarriers, imageMemoryBarrierCount, pImageMemoryBarriers)
        }

        pub unsafe fn vkCmdBeginQuery(&self, commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: uint32_t, flags: VkQueryControlFlags) {
            invoke_command!(self, vkCmdBeginQuery, commandBuffer, queryPool, query, flags)
        }

        pub unsafe fn vkCmdEndQuery(&self, commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: uint32_t) {
            invoke_command!(self, vkCmdEndQuery, commandBuffer, queryPool, query)
        }

        pub unsafe fn vkCmdResetQueryPool(&self, commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: uint32_t, queryCount: uint32_t) {
            invoke_command!(self, vkCmdResetQueryPool, commandBuffer, queryPool, firstQuery, queryCount)
        }

        pub unsafe fn vkCmdWriteTimestamp(&self, commandBuffer: VkCommandBuffer, pipelineStage: VkPipelineStageFlags, queryPool: VkQueryPool, query: uint32_t) {
            invoke_command!(self, vkCmdWriteTimestamp, commandBuffer, pipelineStage, queryPool, query)
        }

        pub unsafe fn vkCmdCopyQueryPoolResults(&self, commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: uint32_t, queryCount: uint32_t, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, stride: VkDeviceSize, flags: VkQueryResultFlags) {
            invoke_command!(self, vkCmdCopyQueryPoolResults, commandBuffer, queryPool, firstQuery, queryCount, dstBuffer, dstOffset, stride, flags)
        }

        pub unsafe fn vkCmdPushConstants(&self, commandBuffer: VkCommandBuffer, layout: VkPipelineLayout, stageFlags: VkShaderStageFlags, offset: uint32_t, size: uint32_t, pValues: *const c_void) {
            invoke_command!(self, vkCmdPushConstants, commandBuffer, layout, stageFlags, offset, size, pValues)
        }

        pub unsafe fn vkCmdBeginRenderPass(&self, commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, contents: VkSubpassContents) {
            invoke_command!(self, vkCmdBeginRenderPass, commandBuffer, pRenderPassBegin, contents)
        }

        pub unsafe fn vkCmdNextSubpass(&self, commandBuffer: VkCommandBuffer, contents: VkSubpassContents) {
            invoke_command!(self, vkCmdNextSubpass, commandBuffer, contents)
        }

        pub unsafe fn vkCmdEndRenderPass(&self, commandBuffer: VkCommandBuffer) {
            invoke_command!(self, vkCmdEndRenderPass, commandBuffer)
        }

        pub unsafe fn vkCmdExecuteCommands(&self, commandBuffer: VkCommandBuffer, commandBufferCount: uint32_t, pCommandBuffers: *const VkCommandBuffer) {
            invoke_command!(self, vkCmdExecuteCommands, commandBuffer, commandBufferCount, pCommandBuffers)
        }
    }
}

pub mod khr_surface {
    use ::libc::{c_char, uint64_t, uint32_t};
    use ::shared_library::dynamic_library::DynamicLibrary;
    use ::std::path::Path;
    use ::std::mem::transmute;
    use ::std::ffi::CString;
    use ::VULKAN_LIBRARY;
    use ::core::*;

    VK_DEFINE_NON_DISPATCHABLE_HANDLE!(VkSurfaceKHR);

    pub const VK_KHR_SURFACE_SPEC_VERSION: uint32_t = 25;
    pub const VK_KHR_SURFACE_EXTENSION_NAME: *const c_char = b"VK_KHR_surface\0" as *const u8 as *const c_char;
    pub const VK_COLORSPACE_SRGB_NONLINEAR_KHR: VkColorSpaceKHR = VkColorSpaceKHR::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR;

    #[repr(i32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkColorSpaceKHR {
        VK_COLOR_SPACE_SRGB_NONLINEAR_KHR = 0
    }

    #[repr(i32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkPresentModeKHR {
        VK_PRESENT_MODE_IMMEDIATE_KHR = 0,
        VK_PRESENT_MODE_MAILBOX_KHR = 1,
        VK_PRESENT_MODE_FIFO_KHR = 2,
        VK_PRESENT_MODE_FIFO_RELAXED_KHR = 3
    }

    bitflags! {
        pub flags VkSurfaceTransformFlagsKHR: VkFlags {
            const VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR = 0x00000001,
            const VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR = 0x00000002,
            const VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR = 0x00000004,
            const VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR = 0x00000008,
            const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR = 0x00000010,
            const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR = 0x00000020,
            const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR = 0x00000040,
            const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR = 0x00000080,
            const VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR = 0x00000100
        }
    }

    bitflags! { 
        pub flags VkCompositeAlphaFlagsKHR: VkFlags {
            const VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR = 0x00000001,
            const VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR = 0x00000002,
            const VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR = 0x00000004,
            const VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR = 0x00000008
        }
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkSurfaceCapabilitiesKHR {
        pub minImageCount: uint32_t,
        pub maxImageCount: uint32_t,
        pub currentExtent: VkExtent2D,
        pub minImageExtent: VkExtent2D,
        pub maxImageExtent: VkExtent2D,
        pub maxImageArrayLayers: uint32_t,
        pub supportedTransforms: VkSurfaceTransformFlagsKHR,
        pub currentTransform: VkSurfaceTransformFlagsKHR,
        pub supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
        pub supportedUsageFlags: VkImageUsageFlags
    }

    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkSurfaceFormatKHR {
        pub format: VkFormat,
        pub colorSpace: VkColorSpaceKHR
    }

    pub type vkDestroySurfaceKHRFn = unsafe extern "stdcall" fn(instance: VkInstance,
                                                                surface: VkSurfaceKHR,
                                                                pAllocator: *const VkAllocationCallbacks);

    pub type vkGetPhysicalDeviceSurfaceSupportKHRFn = unsafe extern "stdcall" fn(physicalDevice: VkPhysicalDevice,
                                                                                 queueFamilyIndex: uint32_t,
                                                                                 surface: VkSurfaceKHR,
                                                                                 pSupported: *mut VkBool32) -> VkResult;

    pub type vkGetPhysicalDeviceSurfaceCapabilitiesKHRFn = unsafe extern "stdcall" fn(physicalDevice: VkPhysicalDevice,
                                                                                      surface: VkSurfaceKHR,
                                                                                      pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR) -> VkResult;

    pub type vkGetPhysicalDeviceSurfaceFormatsKHRFn = unsafe extern "stdcall" fn(physicalDevice: VkPhysicalDevice,
                                                                                 surface: VkSurfaceKHR,
                                                                                 pSurfaceFormatCount: *mut uint32_t,
                                                                                 pSurfaceFormats: *mut VkSurfaceFormatKHR) -> VkResult;

    pub type vkGetPhysicalDeviceSurfacePresentModesKHRFn = unsafe extern "stdcall" fn(physicalDevice: VkPhysicalDevice,
                                                                                      surface: VkSurfaceKHR,
                                                                                      pPresentModeCount: *mut uint32_t,
                                                                                      pPresentModes: *mut VkPresentModeKHR) -> VkResult;

    pub struct VkKhrSurfaceCommands {
       library: Option<DynamicLibrary>,
       vkGetInstanceProcAddr: Option<vkGetInstanceProcAddrFn>,
       vkDestroySurfaceKHR: Option<vkDestroySurfaceKHRFn>,
       vkGetPhysicalDeviceSurfaceSupportKHR: Option<vkGetPhysicalDeviceSurfaceSupportKHRFn>,
       vkGetPhysicalDeviceSurfaceCapabilitiesKHR: Option<vkGetPhysicalDeviceSurfaceCapabilitiesKHRFn>,
       vkGetPhysicalDeviceSurfaceFormatsKHR: Option<vkGetPhysicalDeviceSurfaceFormatsKHRFn>,
       vkGetPhysicalDeviceSurfacePresentModesKHR: Option<vkGetPhysicalDeviceSurfacePresentModesKHRFn>
    }

    impl VkKhrSurfaceCommands {
        pub fn new() -> Result<VkKhrSurfaceCommands, String> {
            unsafe {
                let mut vulkan_khr_surface: VkKhrSurfaceCommands = ::std::mem::zeroed::<VkKhrSurfaceCommands>();
                let library_path = Path::new(VULKAN_LIBRARY);
                vulkan_khr_surface.library = match DynamicLibrary::open(Some(library_path)) {
                    Err(error) => return Err(format!("Failed to load {}: {}",VULKAN_LIBRARY,error)),
                    Ok(library) => Some(library),
                };
                vulkan_khr_surface.vkGetInstanceProcAddr = Some(transmute(try!(vulkan_khr_surface.library.as_ref().unwrap().symbol::<u8>("vkGetInstanceProcAddr"))));
                Ok(vulkan_khr_surface)
            }
        }

        pub fn load(&mut self, instance: VkInstance) -> Result<(), String> {
            unsafe {
                self.vkDestroySurfaceKHR = Some(transmute(load_command!(self, instance, "vkDestroySurfaceKHR")));
                self.vkGetPhysicalDeviceSurfaceSupportKHR = Some(transmute(load_command!(self, instance, "vkGetPhysicalDeviceSurfaceSupportKHR")));
                self.vkGetPhysicalDeviceSurfaceCapabilitiesKHR = Some(transmute(load_command!(self, instance, "vkGetPhysicalDeviceSurfaceCapabilitiesKHR")));
                self.vkGetPhysicalDeviceSurfaceFormatsKHR = Some(transmute(load_command!(self, instance, "vkGetPhysicalDeviceSurfaceFormatsKHR")));
                self.vkGetPhysicalDeviceSurfacePresentModesKHR = Some(transmute(load_command!(self, instance, "vkGetPhysicalDeviceSurfacePresentModesKHR")));
            }
            Ok(())
        }

        pub unsafe fn vkDestroySurfaceKHR(&self, 
                                          instance: VkInstance,
                                          surface: VkSurfaceKHR,
                                          pAllocator: *const VkAllocationCallbacks) {
            invoke_command!(self, vkDestroySurfaceKHR, instance, surface, pAllocator)
        }

        pub unsafe fn vkGetPhysicalDeviceSurfaceSupportKHR(&self, 
                                                           physicalDevice: VkPhysicalDevice,
                                                           queueFamilyIndex: uint32_t,
                                                           surface: VkSurfaceKHR,
                                                           pSupported: *mut VkBool32) -> VkResult {
            invoke_command!(self, vkGetPhysicalDeviceSurfaceSupportKHR, physicalDevice, queueFamilyIndex, surface, pSupported)
        }
        
        pub unsafe fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(&self, 
                                                                physicalDevice: VkPhysicalDevice,
                                                                surface: VkSurfaceKHR,
                                                                pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR) -> VkResult {
            invoke_command!(self, vkGetPhysicalDeviceSurfaceCapabilitiesKHR, physicalDevice, surface, pSurfaceCapabilities)
        }
        
        pub unsafe fn vkGetPhysicalDeviceSurfaceFormatsKHR(&self, 
                                                           physicalDevice: VkPhysicalDevice,
                                                           surface: VkSurfaceKHR,
                                                           pSurfaceFormatCount: *mut uint32_t,
                                                           pSurfaceFormats: *mut VkSurfaceFormatKHR) -> VkResult {
            invoke_command!(self, vkGetPhysicalDeviceSurfaceFormatsKHR, physicalDevice, surface, pSurfaceFormatCount, pSurfaceFormats)
        }
        
        pub unsafe fn vkGetPhysicalDeviceSurfacePresentModesKHR(&self, 
                                                                physicalDevice: VkPhysicalDevice,
                                                                surface: VkSurfaceKHR,
                                                                pPresentModeCount: *mut uint32_t,
                                                                pPresentModes: *mut VkPresentModeKHR) -> VkResult {
            invoke_command!(self, vkGetPhysicalDeviceSurfacePresentModesKHR, physicalDevice, surface, pPresentModeCount, pPresentModes)
        }
    }
}

pub mod khr_swapchain {
    use ::libc::{c_void, c_char, uint64_t, uint32_t};
    use ::shared_library::dynamic_library::DynamicLibrary;
    use ::std::path::Path;
    use ::std::mem::transmute;
    use ::std::ffi::CString;
    use ::VULKAN_LIBRARY;
    use ::core::*;
    use ::khr_surface::*;

    VK_DEFINE_NON_DISPATCHABLE_HANDLE!(VkSwapchainKHR);
    
    pub const VK_KHR_SWAPCHAIN_SPEC_VERSION: uint32_t = 68;
    pub const VK_KHR_SWAPCHAIN_EXTENSION_NAME: *const c_char = b"VK_KHR_swapchain\0" as *const u8 as *const c_char;
    
    reserved_bitflags! { 
        pub flags VkSwapchainCreateFlagsKHR: VkFlags;
    }
    
    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkSwapchainCreateInfoKHR {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkSwapchainCreateFlagsKHR,
        pub surface: VkSurfaceKHR,
        pub minImageCount: uint32_t,
        pub imageFormat: VkFormat,
        pub imageColorSpace: VkColorSpaceKHR,
        pub imageExtent: VkExtent2D,
        pub imageArrayLayers: uint32_t,
        pub imageUsage: VkImageUsageFlags,
        pub imageSharingMode: VkSharingMode,
        pub queueFamilyIndexCount: uint32_t,
        pub pQueueFamilyIndices: *const uint32_t,
        pub preTransform: VkSurfaceTransformFlagsKHR,
        pub compositeAlpha: VkCompositeAlphaFlagsKHR,
        pub presentMode: VkPresentModeKHR,
        pub clipped: VkBool32,
        pub oldSwapchain: VkSwapchainKHR
    }
    
    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkPresentInfoKHR {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub waitSemaphoreCount: uint32_t,
        pub pWaitSemaphores: *const VkSemaphore,
        pub swapchainCount: uint32_t,
        pub pSwapchains: *const VkSwapchainKHR,
        pub pImageIndices: *const uint32_t,
        pub pResults: *mut VkResult
    }
    
    pub type vkCreateSwapchainKHRFn = unsafe extern "stdcall" fn(device: VkDevice, 
                                                                 pCreateInfo: *const VkSwapchainCreateInfoKHR,
                                                                 pAllocator: *const VkAllocationCallbacks,
                                                                 pSwapchain: *mut VkSwapchainKHR) -> VkResult;
    
    pub type vkDestroySwapchainKHRFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                  swapchain: VkSwapchainKHR,
                                                                  pAllocator: *const VkAllocationCallbacks);
    
    pub type vkGetSwapchainImagesKHRFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                    swapchain: VkSwapchainKHR,
                                                                    pSwapchainImageCount: *mut uint32_t,
                                                                    pSwapchainImages: *mut VkImage) -> VkResult;
    
    pub type vkAcquireNextImageKHRFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                  swapchain: VkSwapchainKHR,
                                                                  timeout: uint64_t,
                                                                  semaphore: VkSemaphore,
                                                                  fence: VkFence,
                                                                  pImageIndex: *mut uint32_t) -> VkResult;
    
    pub type vkQueuePresentKHRFn = unsafe extern "stdcall" fn(queue: VkQueue,
                                                              pPresentInfo: *const VkPresentInfoKHR) -> VkResult;
    
    pub struct VkKhrSwapchainCommands {
       library: Option<DynamicLibrary>,
       vkGetInstanceProcAddr: Option<vkGetInstanceProcAddrFn>,
       vkCreateSwapchainKHR: Option<vkCreateSwapchainKHRFn>,
       vkDestroySwapchainKHR: Option<vkDestroySwapchainKHRFn>,
       vkGetSwapchainImagesKHR: Option<vkGetSwapchainImagesKHRFn>,
       vkAcquireNextImageKHR: Option<vkAcquireNextImageKHRFn>,
       vkQueuePresentKHR: Option<vkQueuePresentKHRFn>
    }
    
    impl VkKhrSwapchainCommands {
        pub fn new() -> Result<VkKhrSwapchainCommands, String> {
            unsafe {
                let mut vulkan_khr_swapchain: VkKhrSwapchainCommands = ::std::mem::zeroed::<VkKhrSwapchainCommands>();
                let library_path = Path::new(VULKAN_LIBRARY);
                vulkan_khr_swapchain.library = match DynamicLibrary::open(Some(library_path)) {
                    Err(error) => return Err(format!("Failed to load {}: {}",VULKAN_LIBRARY,error)),
                    Ok(library) => Some(library),
                };
                vulkan_khr_swapchain.vkGetInstanceProcAddr = Some(transmute(try!(vulkan_khr_swapchain.library.as_ref().unwrap().symbol::<u8>("vkGetInstanceProcAddr"))));
                Ok(vulkan_khr_swapchain)
            }
        }
    
        pub fn load(&mut self, instance: VkInstance) -> Result<(), String> {
            unsafe {
                self.vkCreateSwapchainKHR = Some(transmute(load_command!(self, instance, "vkCreateSwapchainKHR")));
                self.vkDestroySwapchainKHR = Some(transmute(load_command!(self, instance, "vkDestroySwapchainKHR")));
                self.vkGetSwapchainImagesKHR = Some(transmute(load_command!(self, instance, "vkGetSwapchainImagesKHR")));
                self.vkAcquireNextImageKHR = Some(transmute(load_command!(self, instance, "vkAcquireNextImageKHR")));
                self.vkQueuePresentKHR = Some(transmute(load_command!(self, instance, "vkQueuePresentKHR")));
            }
            Ok(())
        }
    
        pub unsafe fn vkCreateSwapchainKHR(&self,
                                             device: VkDevice, 
                                             pCreateInfo: *const VkSwapchainCreateInfoKHR,
                                             pAllocator: *const VkAllocationCallbacks,
                                             pSwapchain: *mut VkSwapchainKHR) -> VkResult {
            invoke_command!(self, vkCreateSwapchainKHR, device, pCreateInfo, pAllocator, pSwapchain)
        }
    
        pub unsafe fn vkDestroySwapchainKHR(&self,
                                             device: VkDevice,
                                             swapchain: VkSwapchainKHR,
                                             pAllocator: *const VkAllocationCallbacks) {
            invoke_command!(self, vkDestroySwapchainKHR, device, swapchain, pAllocator)
        }
        pub unsafe fn vkGetSwapchainImagesKHR(&self,
                                             device: VkDevice,
                                             swapchain: VkSwapchainKHR,
                                             pSwapchainImageCount: *mut uint32_t,
                                             pSwapchainImages: *mut VkImage) -> VkResult {
            invoke_command!(self, vkGetSwapchainImagesKHR, device, swapchain, pSwapchainImageCount, pSwapchainImages)
        }
    
        pub unsafe fn vkAcquireNextImageKHR(&self,
                                             device: VkDevice,
                                             swapchain: VkSwapchainKHR,
                                             timeout: uint64_t,
                                             semaphore: VkSemaphore,
                                             fence: VkFence,
                                             pImageIndex: *mut uint32_t) -> VkResult {
            invoke_command!(self, vkAcquireNextImageKHR, device, swapchain, timeout, semaphore, fence, pImageIndex)
        }
    
        pub unsafe fn vkQueuePresentKHR(&self,
                                        queue: VkQueue,
                                        pPresentInfo: *const VkPresentInfoKHR) -> VkResult {
            invoke_command!(self, vkQueuePresentKHR, queue, pPresentInfo)
        }
    }
}

pub mod khr_display {
    use ::libc::{c_void, c_char, c_float, uint64_t, uint32_t};
    use ::shared_library::dynamic_library::DynamicLibrary;
    use ::std::path::Path;
    use ::std::mem::transmute;
    use ::std::ffi::CString;
    use ::VULKAN_LIBRARY;
    use ::core::*;
    use ::khr_surface::*;

    VK_DEFINE_NON_DISPATCHABLE_HANDLE!(VkDisplayKHR);
    VK_DEFINE_NON_DISPATCHABLE_HANDLE!(VkDisplayModeKHR);
    
    pub const VK_KHR_DISPLAY_SPEC_VERSION: uint32_t = 21;
    pub const VK_KHR_DISPLAY_EXTENSION_NAME: *const c_char = b"VK_KHR_display\0" as *const u8 as *const c_char;
    
    bitflags! {
        pub flags VkDisplayPlaneAlphaFlagsKHR: VkFlags {
            const VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR = 0x00000001,
            const VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR = 0x00000002,
            const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR = 0x00000004,
            const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR = 0x00000008
        }
    }
    
    reserved_bitflags! { 
        pub flags VkDisplayModeCreateFlagsKHR: VkFlags;
    }
    reserved_bitflags! { 
        pub flags VkDisplaySurfaceCreateFlagsKHR: VkFlags;
    }
    
    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkDisplayPropertiesKHR {
        pub display: VkDisplayKHR,
        pub displayName: *const c_char,
        pub physicalDimensions: VkExtent2D,
        pub physicalResolution: VkExtent2D,
        pub supportedTransforms: VkSurfaceTransformFlagsKHR,
        pub planeReorderPossible: VkBool32,
        pub persistentContent: VkBool32
    }
    
    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkDisplayModeParametersKHR {
        pub visibleRegion: VkExtent2D,
        pub refreshRate: uint32_t
    }
    
    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkDisplayModePropertiesKHR {
        pub displayMode: VkDisplayModeKHR,
        pub parameters: VkDisplayModeParametersKHR
    }
    
    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkDisplayModeCreateInfoKHR {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkDisplayModeCreateFlagsKHR,
        pub parameters: VkDisplayModeParametersKHR
    }
    
    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkDisplayPlaneCapabilitiesKHR {
        pub supportedAlpha: VkDisplayPlaneAlphaFlagsKHR,
        pub minSrcPosition: VkOffset2D,
        pub maxSrcPosition: VkOffset2D,
        pub minSrcExtent: VkExtent2D,
        pub maxSrcExtent: VkExtent2D,
        pub minDstPosition: VkOffset2D,
        pub maxDstPosition: VkOffset2D,
        pub minDstExtent: VkExtent2D,
        pub maxDstExtent: VkExtent2D
    }
    
    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkDisplayPlanePropertiesKHR {
        pub currentDisplay: VkDisplayKHR,
        pub currentStackIndex: uint32_t
    }
    
    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkDisplaySurfaceCreateInfoKHR {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkDisplaySurfaceCreateFlagsKHR,
        pub displayMode: VkDisplayModeKHR,
        pub planeIndex: uint32_t,
        pub planeStackIndex: uint32_t,
        pub transform: VkSurfaceTransformFlagsKHR,
        pub globalAlpha: c_float,
        pub alphaMode: VkDisplayPlaneAlphaFlagsKHR,
        pub imageExtent: VkExtent2D
    }
    
    pub type vkGetPhysicalDeviceDisplayPropertiesKHRFn = unsafe extern "stdcall" fn(physicalDevice: VkPhysicalDevice, 
                                                                                    pPropertyCount: *mut uint32_t,
                                                                                    pProperties: *mut VkDisplayPropertiesKHR) -> VkResult;
    
    pub type vkGetPhysicalDeviceDisplayPlanePropertiesKHRFn = unsafe extern "stdcall" fn(physicalDevice: VkPhysicalDevice,
                                                                                         pPropertyCount: *mut uint32_t,
                                                                                         pProperties: *mut VkDisplayPlanePropertiesKHR) -> VkResult;
    
    pub type vkGetDisplayPlaneSupportedDisplaysKHRFn = unsafe extern "stdcall" fn(physicalDevice: VkPhysicalDevice,
                                                                                  planeIndex: uint32_t,
                                                                                  pDisplayCount: *mut uint32_t,
                                                                                  pDisplays: *mut VkDisplayKHR) -> VkResult;
    
    pub type vkGetDisplayModePropertiesKHRFn = unsafe extern "stdcall" fn(physicalDevice: VkPhysicalDevice,
                                                                          display: VkDisplayKHR, 
                                                                          pPropertyCount: *mut uint32_t,
                                                                          pProperties: *mut VkDisplayModePropertiesKHR) -> VkResult;
    
    pub type vkCreateDisplayModeKHRFn = unsafe extern "stdcall" fn(physicalDevice: VkPhysicalDevice,
                                                                   display: VkDisplayKHR,
                                                                   pCreateInfo: *const VkDisplayModeCreateInfoKHR,
                                                                   pAllocator: *const VkAllocationCallbacks,
                                                                   pMode: *mut VkDisplayModeKHR) -> VkResult;
    
    pub type vkGetDisplayPlaneCapabilitiesKHRFn = unsafe extern "stdcall" fn(physicalDevice: VkPhysicalDevice,
                                                                             mode: VkDisplayModeKHR,
                                                                             planeIndex: uint32_t,
                                                                             pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR) -> VkResult;
    
    pub type vkCreateDisplayPlaneSurfaceKHRFn = unsafe extern "stdcall" fn(instance: VkInstance,
                                                                           pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR,
                                                                           pAllocator: *const VkAllocationCallbacks,
                                                                           pSurface: *mut VkSurfaceKHR) -> VkResult;
    
    pub struct VkKhrDisplayCommands {
        library: Option<DynamicLibrary>,
        vkGetInstanceProcAddr: Option<vkGetInstanceProcAddrFn>,
        vkGetPhysicalDeviceDisplayPropertiesKHR: Option<vkGetPhysicalDeviceDisplayPropertiesKHRFn>,
        vkGetPhysicalDeviceDisplayPlanePropertiesKHR: Option<vkGetPhysicalDeviceDisplayPlanePropertiesKHRFn>,
        vkGetDisplayPlaneSupportedDisplaysKHR: Option<vkGetDisplayPlaneSupportedDisplaysKHRFn>,
        vkGetDisplayModePropertiesKHR: Option<vkGetDisplayModePropertiesKHRFn>,
        vkCreateDisplayModeKHR: Option<vkCreateDisplayModeKHRFn>,
        vkGetDisplayPlaneCapabilitiesKHR: Option<vkGetDisplayPlaneCapabilitiesKHRFn>,
        vkCreateDisplayPlaneSurfaceKHR: Option<vkCreateDisplayPlaneSurfaceKHRFn>
    }
    
    impl VkKhrDisplayCommands {
        pub fn new() -> Result<VkKhrDisplayCommands, String> {
            unsafe {
                let mut vulkan_khr_display: VkKhrDisplayCommands = ::std::mem::zeroed::<VkKhrDisplayCommands>();
                let library_path = Path::new(VULKAN_LIBRARY);
                vulkan_khr_display.library = match DynamicLibrary::open(Some(library_path)) {
                    Err(error) => return Err(format!("Failed to load {}: {}",VULKAN_LIBRARY,error)),
                    Ok(library) => Some(library),
                };
                vulkan_khr_display.vkGetInstanceProcAddr = Some(transmute(try!(vulkan_khr_display.library.as_ref().unwrap().symbol::<u8>("vkGetInstanceProcAddr"))));
                Ok(vulkan_khr_display)
            }
        }
    
        pub fn load(&mut self, instance: VkInstance) -> Result<(), String> {
            unsafe {
                self.vkGetPhysicalDeviceDisplayPropertiesKHR = Some(transmute(load_command!(self, instance, "vkGetPhysicalDeviceDisplayPropertiesKHR")));
                self.vkGetPhysicalDeviceDisplayPlanePropertiesKHR = Some(transmute(load_command!(self, instance, "vkGetPhysicalDeviceDisplayPlanePropertiesKHR")));
                self.vkGetDisplayPlaneSupportedDisplaysKHR = Some(transmute(load_command!(self, instance, "vkGetDisplayPlaneSupportedDisplaysKHR")));
                self.vkGetDisplayModePropertiesKHR = Some(transmute(load_command!(self, instance, "vkGetDisplayModePropertiesKHR")));
                self.vkCreateDisplayModeKHR = Some(transmute(load_command!(self, instance, "vkCreateDisplayModeKHR")));
                self.vkGetDisplayPlaneCapabilitiesKHR = Some(transmute(load_command!(self, instance, "vkGetDisplayPlaneCapabilitiesKHR")));
                self.vkCreateDisplayPlaneSurfaceKHR = Some(transmute(load_command!(self, instance, "vkCreateDisplayPlaneSurfaceKHR")));
            }
            Ok(())
        }
    
        pub unsafe fn vkGetPhysicalDeviceDisplayPropertiesKHR(&self, 
                                                              physicalDevice: VkPhysicalDevice, 
                                                              pPropertyCount: *mut uint32_t,
                                                              pProperties: *mut VkDisplayPropertiesKHR) -> VkResult {
            invoke_command!(self, vkGetPhysicalDeviceDisplayPropertiesKHR, physicalDevice, pPropertyCount, pProperties)
        }
    
        pub unsafe fn vkGetPhysicalDeviceDisplayPlanePropertiesKHR(&self, 
                                                                   physicalDevice: VkPhysicalDevice,
                                                                   pPropertyCount: *mut uint32_t,
                                                                   pProperties: *mut VkDisplayPlanePropertiesKHR) -> VkResult {
            invoke_command!(self, vkGetPhysicalDeviceDisplayPlanePropertiesKHR, physicalDevice, pPropertyCount, pProperties)
        }
        
        pub unsafe fn vkGetDisplayPlaneSupportedDisplaysKHR(&self, 
                                                            physicalDevice: VkPhysicalDevice,
                                                            planeIndex: uint32_t,
                                                            pDisplayCount: *mut uint32_t,
                                                            pDisplays: *mut VkDisplayKHR) -> VkResult {
            invoke_command!(self, vkGetDisplayPlaneSupportedDisplaysKHR, physicalDevice, planeIndex, pDisplayCount, pDisplays)
        }
        
        pub unsafe fn vkGetDisplayModePropertiesKHR(&self, 
                                                    physicalDevice: VkPhysicalDevice,
                                                    display: VkDisplayKHR, 
                                                    pPropertyCount: *mut uint32_t,
                                                    pProperties: *mut VkDisplayModePropertiesKHR) -> VkResult {
            invoke_command!(self, vkGetDisplayModePropertiesKHR, physicalDevice, display, pPropertyCount, pProperties)
        }
        
        pub unsafe fn vkCreateDisplayModeKHR(&self, 
                                             physicalDevice: VkPhysicalDevice,
                                             display: VkDisplayKHR,
                                             pCreateInfo: *const VkDisplayModeCreateInfoKHR,
                                             pAllocator: *const VkAllocationCallbacks,
                                             pMode: *mut VkDisplayModeKHR) -> VkResult {
            invoke_command!(self, vkCreateDisplayModeKHR, physicalDevice, display, pCreateInfo, pAllocator, pMode)
        }
        
        pub unsafe fn vkGetDisplayPlaneCapabilitiesKHR(&self, 
                                                       physicalDevice: VkPhysicalDevice,
                                                       mode: VkDisplayModeKHR,
                                                       planeIndex: uint32_t,
                                                       pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR) -> VkResult {
            invoke_command!(self, vkGetDisplayPlaneCapabilitiesKHR, physicalDevice, mode, planeIndex, pCapabilities)
        }
        
        pub unsafe fn vkCreateDisplayPlaneSurfaceKHR(&self, 
                                                     instance: VkInstance,
                                                     pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR,
                                                     pAllocator: *const VkAllocationCallbacks,
                                                     pSurface: *mut VkSurfaceKHR) -> VkResult {
            invoke_command!(self, vkCreateDisplayPlaneSurfaceKHR, instance, pCreateInfo, pAllocator, pSurface)
        }
    }
}

pub mod khr_display_swapchain {
    use ::libc::{c_void, c_char, uint32_t};
    use ::shared_library::dynamic_library::DynamicLibrary;
    use ::std::path::Path;
    use ::std::mem::transmute;
    use ::std::ffi::CString;
    use ::VULKAN_LIBRARY;
    use ::core::*;
    use ::khr_swapchain::*;

    pub const VK_KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION: uint32_t = 9;
    pub const VK_KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME: *const c_char = b"VK_KHR_display_swapchain\0" as *const u8 as *const c_char;
    
    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkDisplayPresentInfoKHR {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub srcRect: VkRect2D,
        pub dstRect: VkRect2D,
        pub persistent: VkBool32
    }
    
    pub type vkCreateSharedSwapchainsKHRFn = unsafe extern "stdcall" fn(device: VkDevice,
                                                                        swapchainCount: uint32_t,
                                                                        pCreateInfos: *const VkSwapchainCreateInfoKHR,
                                                                        pAllocator: *const VkAllocationCallbacks,
                                                                        pSwapchains: *mut VkSwapchainKHR) -> VkResult;
    
    pub struct VkKhrDisplaySwapchainCommands {
        library: Option<DynamicLibrary>,
        vkGetInstanceProcAddr: Option<vkGetInstanceProcAddrFn>,
        vkCreateSharedSwapchainsKHR: Option<vkCreateSharedSwapchainsKHRFn>,
    }
    
    impl VkKhrDisplaySwapchainCommands {
        pub fn new() -> Result<VkKhrDisplaySwapchainCommands, String> {
            unsafe {
                let mut vulkan_khr_display_swapchain: VkKhrDisplaySwapchainCommands = ::std::mem::zeroed::<VkKhrDisplaySwapchainCommands>();
                let library_path = Path::new(VULKAN_LIBRARY);
                vulkan_khr_display_swapchain.library = match DynamicLibrary::open(Some(library_path)) {
                    Err(error) => return Err(format!("Failed to load {}: {}",VULKAN_LIBRARY,error)),
                    Ok(library) => Some(library),
                };
                vulkan_khr_display_swapchain.vkGetInstanceProcAddr = Some(transmute(try!(vulkan_khr_display_swapchain.library.as_ref().unwrap().symbol::<u8>("vkGetInstanceProcAddr"))));
                Ok(vulkan_khr_display_swapchain)
            }
        }
    
        pub fn load(&mut self, instance: VkInstance) -> Result<(), String> {
            unsafe {
                self.vkCreateSharedSwapchainsKHR = Some(transmute(load_command!(self, instance, "vkCreateSharedSwapchainsKHR")));
            }
            Ok(())
        }
    
        pub unsafe fn vkCreateSharedSwapchainsKHR(&self, 
                                                  device: VkDevice,
                                                  swapchainCount: uint32_t,
                                                  pCreateInfos: *const VkSwapchainCreateInfoKHR,
                                                  pAllocator: *const VkAllocationCallbacks,
                                                  pSwapchains: *mut VkSwapchainKHR) -> VkResult {
            invoke_command!(self, vkCreateSharedSwapchainsKHR, device, swapchainCount, pCreateInfos, pAllocator, pSwapchains)
        }
    }
}

pub mod khr_win32_surface {
    use ::libc::{c_void, c_char, uint32_t};
    use ::shared_library::dynamic_library::DynamicLibrary;
    use ::std::path::Path;
    use ::std::ffi::CString;
    use ::std::mem::transmute;
    use ::VULKAN_LIBRARY;
    use ::core::*;
    use ::khr_surface::*;

    pub mod platform {
        use ::libc::c_void;
        pub type HINSTANCE = *mut c_void;
        pub type HWND = *mut c_void;
    }

    pub const VK_KHR_WIN32_SURFACE_SPEC_VERSION: uint32_t = 5;
    pub const VK_KHR_WIN32_SURFACE_EXTENSION_NAME: *const c_char = b"VK_KHR_win32_surface\0" as *const u8 as *const c_char;
    
    reserved_bitflags! { 
        pub flags VkWin32SurfaceCreateFlagsKHR: VkFlags;
    }
    
    #[repr(C)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub struct VkWin32SurfaceCreateInfoKHR {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkWin32SurfaceCreateFlagsKHR,
        pub hinstance: platform::HINSTANCE,
        pub hwnd: platform::HWND
    }
    
    pub type vkCreateWin32SurfaceKHRFn = unsafe extern "stdcall" fn(instance: VkInstance, 
                                                                    pCreateInfo: *const VkWin32SurfaceCreateInfoKHR,
                                                                    pAllocator: *const VkAllocationCallbacks,
                                                                    pSurface: *mut VkSurfaceKHR) -> VkResult;
    
    pub type vkGetPhysicalDeviceWin32PresentationSupportKHRFn = unsafe extern "stdcall" fn(physicalDevice: VkPhysicalDevice, 
                                                                                           queueFamilyIndex: uint32_t) -> VkBool32;
    
    pub struct VkKhrWin32SurfaceCommands {
        library: Option<DynamicLibrary>,
        vkGetInstanceProcAddr: Option<vkGetInstanceProcAddrFn>,
        vkCreateWin32SurfaceKHR: Option<vkCreateWin32SurfaceKHRFn>,
        vkGetPhysicalDeviceWin32PresentationSupportKHR: Option<vkGetPhysicalDeviceWin32PresentationSupportKHRFn>
    }
    
    impl VkKhrWin32SurfaceCommands {
        pub fn new() -> Result<VkKhrWin32SurfaceCommands, String> {
            unsafe {
                let mut vulkan_khr_win32_surface: VkKhrWin32SurfaceCommands = ::std::mem::zeroed::<VkKhrWin32SurfaceCommands>();
                let library_path = Path::new(VULKAN_LIBRARY);
                vulkan_khr_win32_surface.library = match DynamicLibrary::open(Some(library_path)) {
                    Err(error) => return Err(format!("Failed to load {}: {}",VULKAN_LIBRARY,error)),
                    Ok(library) => Some(library),
                };
                vulkan_khr_win32_surface.vkGetInstanceProcAddr = Some(transmute(try!(vulkan_khr_win32_surface.library.as_ref().unwrap().symbol::<u8>("vkGetInstanceProcAddr"))));
                Ok(vulkan_khr_win32_surface)
            }
        }
    
        pub fn load(&mut self, instance: VkInstance) -> Result<(), String> {
            unsafe {
                self.vkCreateWin32SurfaceKHR = Some(transmute(load_command!(self, instance, "vkCreateWin32SurfaceKHR")));
                self.vkGetPhysicalDeviceWin32PresentationSupportKHR = Some(transmute(load_command!(self, instance, "vkGetPhysicalDeviceWin32PresentationSupportKHR")));
            }
            Ok(())
        }
    
        pub unsafe fn vkCreateWin32SurfaceKHR(&self,
                                              instance: VkInstance,
                                              pCreateInfo: *const VkWin32SurfaceCreateInfoKHR,
                                              pAllocator: *const VkAllocationCallbacks,
                                              pSurface: *mut VkSurfaceKHR) -> VkResult {
            invoke_command!(self, vkCreateWin32SurfaceKHR, instance, pCreateInfo, pAllocator, pSurface)
        }
    
        pub unsafe fn vkGetPhysicalDeviceWin32PresentationSupportKHR(&self,
                                                                     physicalDevice: VkPhysicalDevice,
                                                                     queueFamilyIndex: uint32_t) -> VkBool32 {
            invoke_command!(self, vkGetPhysicalDeviceWin32PresentationSupportKHR, physicalDevice, queueFamilyIndex)
        }
    }
}

pub mod ext_debug_report {
    use ::libc::{c_void, c_char, uint32_t, int32_t, uint64_t, size_t};
    use ::shared_library::dynamic_library::DynamicLibrary;
    use ::std::path::Path;
    use ::std::ffi::CString;
    use ::std::mem::transmute;
    use ::VULKAN_LIBRARY;
    use ::core::*;

    VK_DEFINE_NON_DISPATCHABLE_HANDLE!(VkDebugReportCallbackEXT);
    
    pub const VK_EXT_DEBUG_REPORT_SPEC_VERSION: uint32_t = 2;
    pub const VK_EXT_DEBUG_REPORT_EXTENSION_NAME: *const c_char = b"VK_EXT_debug_report\0" as *const u8 as *const c_char;
    pub const VK_STRUCTURE_TYPE_DEBUG_REPORT_CREATE_INFO_EXT: VkStructureType = VkStructureType::VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT;
    
    #[repr(i32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkDebugReportObjectTypeEXT {
        VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT = 0,
        VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT = 1,
        VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT = 2,
        VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT = 3,
        VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT = 4,
        VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT = 5,
        VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT = 6,
        VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT = 7,
        VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT = 8,
        VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT = 9,
        VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT = 10,
        VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT = 11,
        VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT = 12,
        VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT = 13,
        VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT = 14,
        VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT = 15,
        VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT = 16,
        VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT = 17,
        VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT = 18,
        VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT = 19,
        VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT = 20,
        VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT = 21,
        VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT = 22,
        VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT = 23,
        VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT = 24,
        VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT = 25,
        VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT = 26,
        VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT = 27,
        VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT = 28
    }
    
    #[repr(i32)]
    #[derive(Eq)]
    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Copy)]
    #[derive(Clone)]
    pub enum VkDebugReportErrorEXT {
        VK_DEBUG_REPORT_ERROR_NONE_EXT = 0,
        VK_DEBUG_REPORT_ERROR_CALLBACK_REF_EXT = 1
    }
    
    bitflags! { 
        pub flags VkDebugReportFlagsEXT: VkFlags {
            const VK_DEBUG_REPORT_INFORMATION_BIT_EXT = 0x00000001,
            const VK_DEBUG_REPORT_WARNING_BIT_EXT = 0x00000002,
            const VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT = 0x00000004,
            const VK_DEBUG_REPORT_ERROR_BIT_EXT = 0x00000008,
            const VK_DEBUG_REPORT_DEBUG_BIT_EXT = 0x00000010
        }
    }
    
    pub type vkDebugReportCallbackEXTFn = unsafe extern "stdcall" fn(flags: VkDebugReportFlagsEXT,
                                                                     objectType: VkDebugReportObjectTypeEXT,
                                                                     object: uint64_t,
                                                                     location: size_t,
                                                                     messageCode: int32_t,
                                                                     pLayerPrefix: *const c_char,
                                                                     pMessage: *const c_char,
                                                                     pUserData: *mut c_void) -> VkBool32;
    #[repr(C)]
    #[derive(Copy)]
    pub struct VkDebugReportCallbackCreateInfoEXT {
        pub sType: VkStructureType,
        pub pNext: *const c_void,
        pub flags: VkDebugReportFlagsEXT,
        pub pfnCallback: Option<vkDebugReportCallbackEXTFn>,
        pub pUserData: *mut c_void
    }
    
    // Due to Rust issue #24000
    impl Clone for VkDebugReportCallbackCreateInfoEXT {
        fn clone(&self) -> Self {
            unsafe {
                ::std::mem::transmute_copy(self)
            }
        }
    }

    pub type vkCreateDebugReportCallbackEXTFn = unsafe extern "stdcall" fn(instance: VkInstance,
                                                                           pCreateInfo: *const VkDebugReportCallbackCreateInfoEXT,
                                                                           pAllocator: *const VkAllocationCallbacks, 
                                                                           pCallback: *mut VkDebugReportCallbackEXT) -> VkResult;
    
    pub type vkDestroyDebugReportCallbackEXTFn = unsafe extern "stdcall" fn(instance: VkInstance,
                                                                            callback: VkDebugReportCallbackEXT,
                                                                            pAllocator: *const VkAllocationCallbacks);
    
    pub type vkDebugReportMessageEXTFn = unsafe extern "stdcall" fn(instance: VkInstance,
                                                                    flags: VkDebugReportFlagsEXT,
                                                                    objectType: VkDebugReportObjectTypeEXT,
                                                                    object: uint64_t,
                                                                    location: size_t,
                                                                    messageCode: int32_t,
                                                                    pLayerPrefix: *const c_char,
                                                                    pMessage: *const c_char);
    
    pub struct VkExtDebugReportCommands {
       library: Option<DynamicLibrary>,
       vkGetInstanceProcAddr: Option<vkGetInstanceProcAddrFn>,
       vkCreateDebugReportCallbackEXT: Option<vkCreateDebugReportCallbackEXTFn>,
       vkDestroyDebugReportCallbackEXT: Option<vkDestroyDebugReportCallbackEXTFn>,
       vkDebugReportMessageEXT: Option<vkDebugReportMessageEXTFn>
    }
    
    impl VkExtDebugReportCommands {
        pub fn new() -> Result<VkExtDebugReportCommands, String> {
            unsafe {
                let mut vulkan_ext_debug_report: VkExtDebugReportCommands = ::std::mem::zeroed::<VkExtDebugReportCommands>();
                let library_path = Path::new(VULKAN_LIBRARY);
                vulkan_ext_debug_report.library = match DynamicLibrary::open(Some(library_path)) {
                    Err(error) => return Err(format!("Failed to load {}: {}",VULKAN_LIBRARY,error)),
                    Ok(library) => Some(library),
                };
                vulkan_ext_debug_report.vkGetInstanceProcAddr = Some(transmute(try!(vulkan_ext_debug_report.library.as_ref().unwrap().symbol::<u8>("vkGetInstanceProcAddr"))));
                Ok(vulkan_ext_debug_report)
            }
        }
    
        pub fn load(&mut self, instance: VkInstance) -> Result<(), String> {
            unsafe {
                self.vkCreateDebugReportCallbackEXT = Some(transmute(load_command!(self, instance, "vkCreateDebugReportCallbackEXT")));
                self.vkDestroyDebugReportCallbackEXT = Some(transmute(load_command!(self, instance, "vkDestroyDebugReportCallbackEXT")));
                self.vkDebugReportMessageEXT = Some(transmute(load_command!(self, instance, "vkDebugReportMessageEXT")));
            }
            Ok(())
        }
    
        pub unsafe fn vkCreateDebugReportCallbackEXT(&self,
                                                     instance: VkInstance,
                                                     pCreateInfo: *const VkDebugReportCallbackCreateInfoEXT,
                                                     pAllocator: *const VkAllocationCallbacks, 
                                                     pCallback: *mut VkDebugReportCallbackEXT) -> VkResult {
            invoke_command!(self, vkCreateDebugReportCallbackEXT, instance, pCreateInfo, pAllocator, pCallback)
        }
    
        pub unsafe fn vkDestroyDebugReportCallbackEXT(&self,
                                                      instance: VkInstance,
                                                      callback: VkDebugReportCallbackEXT,
                                                      pAllocator: *const VkAllocationCallbacks) {
            invoke_command!(self, vkDestroyDebugReportCallbackEXT, instance, callback, pAllocator)
        }
    
        pub unsafe fn vkDebugReportMessageEXT(&self,
                                              instance: VkInstance,
                                              flags: VkDebugReportFlagsEXT,
                                              objectType: VkDebugReportObjectTypeEXT,
                                              object: uint64_t,
                                              location: size_t,
                                              messageCode: int32_t,
                                              pLayerPrefix: *const c_char,
                                              pMessage: *const c_char) {
            invoke_command!(self, vkDebugReportMessageEXT, instance, flags, objectType, object, location, messageCode, pLayerPrefix, pMessage)
        }
    }
}