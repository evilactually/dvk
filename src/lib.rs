#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate shared_library;

use self::libc::{c_void, c_char, uint32_t, size_t, uint64_t, c_float, int32_t, uint8_t};
use self::shared_library::dynamic_library::DynamicLibrary;
use std::path::{Path};
use std::ffi::CString;

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

pub type VkFlags = uint32_t;
pub type VkBool32 = uint32_t;
pub type VkDeviceSize = uint64_t;
pub type VkSampleMask = uint32_t;
#[derive(Clone)] #[derive(Copy)] #[repr(C)] pub struct VkInstance(*const c_void);
#[derive(Clone)] #[derive(Copy)] #[repr(C)] pub struct VkPhysicalDevice(*const c_void);
#[derive(Clone)] #[derive(Copy)] #[repr(C)] pub struct VkDevice(*const c_void);
#[derive(Clone)] #[derive(Copy)] #[repr(C)] pub struct VkQueue(*const c_void);
#[derive(Clone)] #[derive(Copy)] #[repr(C)] pub struct VkSemaphore(uint64_t);
#[derive(Clone)] #[derive(Copy)] #[repr(C)] pub struct VkCommandBuffer(*const c_void);
#[derive(Clone)] #[derive(Copy)] #[repr(C)] pub struct VkFence(uint64_t);
#[derive(Clone)] #[derive(Copy)] #[repr(C)] pub struct VkDeviceMemory(uint64_t);
#[derive(Clone)] #[derive(Copy)] #[repr(C)] pub struct VkBuffer(uint64_t);
#[derive(Clone)] #[derive(Copy)] #[repr(C)] pub struct VkImage(uint64_t);
#[derive(Clone)] #[derive(Copy)] #[repr(C)] pub struct VkEvent(uint64_t);
#[derive(Clone)] #[derive(Copy)] #[repr(C)] pub struct VkQueryPool(uint64_t);
#[derive(Clone)] #[derive(Copy)] #[repr(C)] pub struct VkBufferView(uint64_t);
#[derive(Clone)] #[derive(Copy)] #[repr(C)] pub struct VkImageView(uint64_t);
#[derive(Clone)] #[derive(Copy)] #[repr(C)] pub struct VkShaderModule(uint64_t);
#[derive(Clone)] #[derive(Copy)] #[repr(C)] pub struct VkPipelineCache(uint64_t);
#[derive(Clone)] #[derive(Copy)] #[repr(C)] pub struct VkPipelineLayout(uint64_t);
#[derive(Clone)] #[derive(Copy)] #[repr(C)] pub struct VkRenderPass(uint64_t);
#[derive(Clone)] #[derive(Copy)] #[repr(C)] pub struct VkPipeline(uint64_t);
#[derive(Clone)] #[derive(Copy)] #[repr(C)] pub struct VkDescriptorSetLayout(uint64_t);
#[derive(Clone)] #[derive(Copy)] #[repr(C)] pub struct VkSampler(uint64_t);
#[derive(Clone)] #[derive(Copy)] #[repr(C)] pub struct VkDescriptorPool(uint64_t);
#[derive(Clone)] #[derive(Copy)] #[repr(C)] pub struct VkDescriptorSet(uint64_t);
#[derive(Clone)] #[derive(Copy)] #[repr(C)] pub struct VkFramebuffer(uint64_t);
#[derive(Clone)] #[derive(Copy)] #[repr(C)] pub struct VkCommandPool(uint64_t);

impl VkInstance {
    pub fn null() -> VkInstance {
        VkInstance(std::ptr::null())
    }
}

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
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum VkPipelineCacheHeaderVersion {
    VK_PIPELINE_CACHE_HEADER_VERSION_ONE = 1
}

#[repr(i32)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
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
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
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
pub enum VkInternalAllocationType {
    VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE = 0
}

#[repr(u32)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
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
pub enum VkImageType {
    VK_IMAGE_TYPE_1D = 0,
    VK_IMAGE_TYPE_2D = 1,
    VK_IMAGE_TYPE_3D = 2
}

#[repr(u32)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum VkImageTiling {
    VK_IMAGE_TILING_OPTIMAL = 0,
    VK_IMAGE_TILING_LINEAR = 1
}

#[repr(u32)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
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
pub enum VkQueryType {
    VK_QUERY_TYPE_OCCLUSION = 0,
    VK_QUERY_TYPE_PIPELINE_STATISTICS = 1,
    VK_QUERY_TYPE_TIMESTAMP = 2
}

#[repr(u32)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum VkSharingMode {
    VK_SHARING_MODE_EXCLUSIVE = 0,
    VK_SHARING_MODE_CONCURRENT = 1
}

#[repr(u32)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
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
pub enum VkVertexInputRate {
    VK_VERTEX_INPUT_RATE_VERTEX = 0,
    VK_VERTEX_INPUT_RATE_INSTANCE = 1
}

#[repr(u32)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
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
pub enum VkPolygonMode {
    VK_POLYGON_MODE_FILL = 0,
    VK_POLYGON_MODE_LINE = 1,
    VK_POLYGON_MODE_POINT = 2
}

#[repr(u32)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum VkFrontFace {
    VK_FRONT_FACE_COUNTER_CLOCKWISE = 0,
    VK_FRONT_FACE_CLOCKWISE = 1
}

#[repr(u32)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
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
pub enum VkFilter {
    VK_FILTER_NEAREST = 0,
    VK_FILTER_LINEAR = 1,
    VK_FILTER_CUBIC_IMG = 1000015000
}

#[repr(u32)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum VkSamplerMipmapMode {
    VK_SAMPLER_MIPMAP_MODE_NEAREST = 0,
    VK_SAMPLER_MIPMAP_MODE_LINEAR = 1
}

#[repr(u32)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
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
pub enum VkAttachmentLoadOp {
    VK_ATTACHMENT_LOAD_OP_LOAD = 0,
    VK_ATTACHMENT_LOAD_OP_CLEAR = 1,
    VK_ATTACHMENT_LOAD_OP_DONT_CARE = 2
}

#[repr(u32)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum VkAttachmentStoreOp {
    VK_ATTACHMENT_STORE_OP_STORE = 0,
    VK_ATTACHMENT_STORE_OP_DONT_CARE = 1
}

#[repr(u32)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum VkPipelineBindPoint {
    VK_PIPELINE_BIND_POINT_GRAPHICS = 0,
    VK_PIPELINE_BIND_POINT_COMPUTE = 1
}

#[repr(u32)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum VkCommandBufferLevel {
    VK_COMMAND_BUFFER_LEVEL_PRIMARY = 0,
    VK_COMMAND_BUFFER_LEVEL_SECONDARY = 1
}

#[repr(u32)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum VkIndexType {
    VK_INDEX_TYPE_UINT16 = 0,
    VK_INDEX_TYPE_UINT32 = 1
}

#[repr(u32)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum VkSubpassContents {
    VK_SUBPASS_CONTENTS_INLINE = 0,
    VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS = 1
}

pub type VkInstanceCreateFlags = VkFlags;

bitflags! { 
    pub flags VkFormatFeatureFlagBits: u32 {
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
pub type VkFormatFeatureFlags = VkFlags;

bitflags! { 
    pub flags VkImageUsageFlagBits: u32 {
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
pub type VkImageUsageFlags = VkFlags;

bitflags! { 
    pub flags VkImageCreateFlagBits: u32 {
        const VK_IMAGE_CREATE_SPARSE_BINDING_BIT = 0x00000001,
        const VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT = 0x00000002,
        const VK_IMAGE_CREATE_SPARSE_ALIASED_BIT = 0x00000004,
        const VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT = 0x00000008,
        const VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT = 0x00000010
    }
}
pub type VkImageCreateFlags = VkFlags;

bitflags! { 
    pub flags VkSampleCountFlagBits: u32 {
        const VK_SAMPLE_COUNT_1_BIT = 0x00000001,
        const VK_SAMPLE_COUNT_2_BIT = 0x00000002,
        const VK_SAMPLE_COUNT_4_BIT = 0x00000004,
        const VK_SAMPLE_COUNT_8_BIT = 0x00000008,
        const VK_SAMPLE_COUNT_16_BIT = 0x00000010,
        const VK_SAMPLE_COUNT_32_BIT = 0x00000020,
        const VK_SAMPLE_COUNT_64_BIT = 0x00000040
    }
}
pub type VkSampleCountFlags = VkFlags;

bitflags! { 
    pub flags VkQueueFlagBits: u32 {
        const VK_QUEUE_GRAPHICS_BIT = 0x00000001,
        const VK_QUEUE_COMPUTE_BIT = 0x00000002,
        const VK_QUEUE_TRANSFER_BIT = 0x00000004,
        const VK_QUEUE_SPARSE_BINDING_BIT = 0x00000008
    }
}
pub type VkQueueFlags = VkFlags;

bitflags! { 
    pub flags VkMemoryPropertyFlagBits: u32 {
        const VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT = 0x00000001,
        const VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT = 0x00000002,
        const VK_MEMORY_PROPERTY_HOST_COHERENT_BIT = 0x00000004,
        const VK_MEMORY_PROPERTY_HOST_CACHED_BIT = 0x00000008,
        const VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT = 0x00000010
    }
}
pub type VkMemoryPropertyFlags = VkFlags;

bitflags! { 
    pub flags VkMemoryHeapFlagBits: u32 {
        const VK_MEMORY_HEAP_DEVICE_LOCAL_BIT = 0x00000001
    }
}
pub type VkMemoryHeapFlags = VkFlags;
pub type VkDeviceCreateFlags = VkFlags;
pub type VkDeviceQueueCreateFlags = VkFlags;

bitflags! { 
    pub flags VkPipelineStageFlagBits: u32 {
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
pub type VkPipelineStageFlags = VkFlags;
pub type VkMemoryMapFlags = VkFlags;

bitflags! { 
    pub flags VkImageAspectFlagBits: u32 {
        const VK_IMAGE_ASPECT_COLOR_BIT = 0x00000001,
        const VK_IMAGE_ASPECT_DEPTH_BIT = 0x00000002,
        const VK_IMAGE_ASPECT_STENCIL_BIT = 0x00000004,
        const VK_IMAGE_ASPECT_METADATA_BIT = 0x00000008
    }
}
pub type VkImageAspectFlags = VkFlags;

bitflags! { 
    pub flags VkSparseImageFormatFlagBits: u32 {
        const VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT = 0x00000001,
        const VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT = 0x00000002,
        const VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT = 0x00000004
    }
}
pub type VkSparseImageFormatFlags = VkFlags;

bitflags! { 
    pub flags VkSparseMemoryBindFlagBits: u32 {
        const VK_SPARSE_MEMORY_BIND_METADATA_BIT = 0x00000001
    }
}
pub type VkSparseMemoryBindFlags = VkFlags;

bitflags! { 
    pub flags VkFenceCreateFlagBits: u32 {
        const VK_FENCE_CREATE_SIGNALED_BIT = 0x00000001
    }
}
pub type VkFenceCreateFlags = VkFlags;
pub type VkSemaphoreCreateFlags = VkFlags;
pub type VkEventCreateFlags = VkFlags;
pub type VkQueryPoolCreateFlags = VkFlags;

bitflags! { 
    pub flags VkQueryPipelineStatisticFlagBits: u32 {
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
pub type VkQueryPipelineStatisticFlags = VkFlags;

bitflags! { 
    pub flags VkQueryResultFlagBits: u32 {
        const VK_QUERY_RESULT_64_BIT = 0x00000001,
        const VK_QUERY_RESULT_WAIT_BIT = 0x00000002,
        const VK_QUERY_RESULT_WITH_AVAILABILITY_BIT = 0x00000004,
        const VK_QUERY_RESULT_PARTIAL_BIT = 0x00000008
    }
}
pub type VkQueryResultFlags = VkFlags;

bitflags! { 
    pub flags VkBufferCreateFlagBits: u32 {
        const VK_BUFFER_CREATE_SPARSE_BINDING_BIT = 0x00000001,
        const VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT = 0x00000002,
        const VK_BUFFER_CREATE_SPARSE_ALIASED_BIT = 0x00000004
    }
}
pub type VkBufferCreateFlags = VkFlags;

bitflags! { 
    pub flags VkBufferUsageFlagBits: u32 {
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
pub type VkBufferUsageFlags = VkFlags;
pub type VkBufferViewCreateFlags = VkFlags;
pub type VkImageViewCreateFlags = VkFlags;
pub type VkShaderModuleCreateFlags = VkFlags;
pub type VkPipelineCacheCreateFlags = VkFlags;

bitflags! { 
    pub flags VkPipelineCreateFlagBits: u32 {
        const VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT = 0x00000001,
        const VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT = 0x00000002,
        const VK_PIPELINE_CREATE_DERIVATIVE_BIT = 0x00000004
    }
}
pub type VkPipelineCreateFlags = VkFlags;
pub type VkPipelineShaderStageCreateFlags = VkFlags;

bitflags! { 
    pub flags VkShaderStageFlagBits: u32 {
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
pub type VkPipelineVertexInputStateCreateFlags = VkFlags;
pub type VkPipelineInputAssemblyStateCreateFlags = VkFlags;
pub type VkPipelineTessellationStateCreateFlags = VkFlags;
pub type VkPipelineViewportStateCreateFlags = VkFlags;
pub type VkPipelineRasterizationStateCreateFlags = VkFlags;

bitflags! { 
    pub flags VkCullModeFlagBits: u32 {
        const VK_CULL_MODE_NONE = 0,
        const VK_CULL_MODE_FRONT_BIT = 0x00000001,
        const VK_CULL_MODE_BACK_BIT = 0x00000002,
        const VK_CULL_MODE_FRONT_AND_BACK = 0x00000003
    }
}
pub type VkCullModeFlags = VkFlags;
pub type VkPipelineMultisampleStateCreateFlags = VkFlags;
pub type VkPipelineDepthStencilStateCreateFlags = VkFlags;
pub type VkPipelineColorBlendStateCreateFlags = VkFlags;

bitflags! { 
    pub flags VkColorComponentFlagBits: u32 {
        const VK_COLOR_COMPONENT_R_BIT = 0x00000001,
        const VK_COLOR_COMPONENT_G_BIT = 0x00000002,
        const VK_COLOR_COMPONENT_B_BIT = 0x00000004,
        const VK_COLOR_COMPONENT_A_BIT = 0x00000008
    }
}
pub type VkColorComponentFlags = VkFlags;
pub type VkPipelineDynamicStateCreateFlags = VkFlags;
pub type VkPipelineLayoutCreateFlags = VkFlags;
pub type VkShaderStageFlags = VkFlags;
pub type VkSamplerCreateFlags = VkFlags;
pub type VkDescriptorSetLayoutCreateFlags = VkFlags;

bitflags! { 
    pub flags VkDescriptorPoolCreateFlagBits: u32 {
        const VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT = 0x00000001
    }
}
pub type VkDescriptorPoolCreateFlags = VkFlags;
pub type VkDescriptorPoolResetFlags = VkFlags;
pub type VkFramebufferCreateFlags = VkFlags;
pub type VkRenderPassCreateFlags = VkFlags;

bitflags! { 
    pub flags VkAttachmentDescriptionFlagBits: u32 {
        const VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT = 0x00000001
    }
}
pub type VkAttachmentDescriptionFlags = VkFlags;
pub type VkSubpassDescriptionFlags = VkFlags;

bitflags! { 
    pub flags VkAccessFlagBits: u32 {
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
pub type VkAccessFlags = VkFlags;

bitflags! { 
    pub flags VkDependencyFlagBits: u32 {
        const VK_DEPENDENCY_BY_REGION_BIT = 0x00000001
    }
}
pub type VkDependencyFlags = VkFlags;

bitflags! { 
    pub flags VkCommandPoolCreateFlagBits: u32 {
        const VK_COMMAND_POOL_CREATE_TRANSIENT_BIT = 0x00000001,
        const VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT = 0x00000002
    }
}
pub type VkCommandPoolCreateFlags = VkFlags;

bitflags! { 
    pub flags VkCommandPoolResetFlagBits: u32 {
        const VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT = 0x00000001
    }
}
pub type VkCommandPoolResetFlags = VkFlags;

bitflags! { 
    pub flags VkCommandBufferUsageFlagBits: u32 {
        const VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT = 0x00000001,
        const VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT = 0x00000002,
        const VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT = 0x00000004
    }
}
pub type VkCommandBufferUsageFlags = VkFlags;

bitflags! { 
    pub flags VkQueryControlFlagBits: u32 {
        const VK_QUERY_CONTROL_PRECISE_BIT = 0x00000001
    }
}
pub type VkQueryControlFlags = VkFlags;

bitflags! { 
    pub flags VkCommandBufferResetFlagBits: u32 {
        const VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT = 0x00000001
    }
}
pub type VkCommandBufferResetFlags = VkFlags;

bitflags! { 
    pub flags VkStencilFaceFlagBits: u32 {
        const VK_STENCIL_FACE_FRONT_BIT = 0x00000001,
        const VK_STENCIL_FACE_BACK_BIT = 0x00000002,
        const VK_STENCIL_FRONT_AND_BACK = 0x00000003
    }
}
pub type VkStencilFaceFlags = VkFlags;

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
pub struct VkAllocationCallbacks {
    pub pUserData: *const c_void,
    pub pfnAllocation: vkAllocationFunctionFn,
    pub pfnReallocation: vkReallocationFunctionFn,
    pub pfnFree: vkFreeFunctionFn,
    pub pfnInternalAllocation: vkInternalAllocationNotificationFn,
    pub pfnInternalFree: vkInternalFreeNotificationFn
}

#[repr(C)]
pub struct VkPhysicalDeviceFeatures {
    robustBufferAccess: VkBool32,
    fullDrawIndexUint32: VkBool32,
    imageCubeArray: VkBool32,
    independentBlend: VkBool32,
    geometryShader: VkBool32,
    tessellationShader: VkBool32,
    sampleRateShading: VkBool32,
    dualSrcBlend: VkBool32,
    logicOp: VkBool32,
    multiDrawIndirect: VkBool32,
    drawIndirectFirstInstance: VkBool32,
    depthClamp: VkBool32,
    depthBiasClamp: VkBool32,
    fillModeNonSolid: VkBool32,
    depthBounds: VkBool32,
    wideLines: VkBool32,
    largePoints: VkBool32,
    alphaToOne: VkBool32,
    multiViewport: VkBool32,
    samplerAnisotropy: VkBool32,
    textureCompressionETC2: VkBool32,
    textureCompressionASTC_LDR: VkBool32,
    textureCompressionBC: VkBool32,
    occlusionQueryPrecise: VkBool32,
    pipelineStatisticsQuery: VkBool32,
    vertexPipelineStoresAndAtomics: VkBool32,
    fragmentStoresAndAtomics: VkBool32,
    shaderTessellationAndGeometryPointSize: VkBool32,
    shaderImageGatherExtended: VkBool32,
    shaderStorageImageExtendedFormats: VkBool32,
    shaderStorageImageMultisample: VkBool32,
    shaderStorageImageReadWithoutFormat: VkBool32,
    shaderStorageImageWriteWithoutFormat: VkBool32,
    shaderUniformBufferArrayDynamicIndexing: VkBool32,
    shaderSampledImageArrayDynamicIndexing: VkBool32,
    shaderStorageBufferArrayDynamicIndexing: VkBool32,
    shaderStorageImageArrayDynamicIndexing: VkBool32,
    shaderClipDistance: VkBool32,
    shaderCullDistance: VkBool32,
    shaderFloat64: VkBool32,
    shaderInt64: VkBool32,
    shaderInt16: VkBool32,
    shaderResourceResidency: VkBool32,
    shaderResourceMinLod: VkBool32,
    sparseBinding: VkBool32,
    sparseResidencyBuffer: VkBool32,
    sparseResidencyImage2D: VkBool32,
    sparseResidencyImage3D: VkBool32,
    sparseResidency2Samples: VkBool32,
    sparseResidency4Samples: VkBool32,
    sparseResidency8Samples: VkBool32,
    sparseResidency16Samples: VkBool32,
    sparseResidencyAliased: VkBool32,
    variableMultisampleRate: VkBool32,
    inheritedQueries: VkBool32
}

#[repr(C)]
pub struct VkFormatProperties {
    linearTilingFeatures: VkFormatFeatureFlags,
    optimalTilingFeatures: VkFormatFeatureFlags,
    bufferFeatures: VkFormatFeatureFlags
}

#[repr(C)]
pub struct VkExtent3D {
    width: uint32_t,
    height: uint32_t,
    depth: uint32_t
}

#[repr(C)]
pub struct VkImageFormatProperties {
    maxExtent: VkExtent3D,
    maxMipLevels: uint32_t,
    maxArrayLayers: uint32_t,
    sampleCounts: VkSampleCountFlags,
    maxResourceSize: VkDeviceSize
}

#[repr(C)]
pub struct VkPhysicalDeviceLimits {
    maxImageDimension2D: uint32_t,
    maxImageDimension1D: uint32_t,
    maxImageDimension3D: uint32_t,
    maxImageDimensionCube: uint32_t,
    maxImageArrayLayers: uint32_t,
    maxTexelBufferElements: uint32_t,
    maxUniformBufferRange: uint32_t,
    maxStorageBufferRange: uint32_t,
    maxPushConstantsSize: uint32_t,
    maxMemoryAllocationCount: uint32_t,
    maxSamplerAllocationCount: uint32_t,
    bufferImageGranularity: VkDeviceSize,
    sparseAddressSpaceSize: VkDeviceSize,
    maxBoundDescriptorSets: uint32_t,
    maxPerStageDescriptorSamplers: uint32_t,
    maxPerStageDescriptorUniformBuffers: uint32_t,
    maxPerStageDescriptorStorageBuffers: uint32_t,
    maxPerStageDescriptorSampledImages: uint32_t,
    maxPerStageDescriptorStorageImages: uint32_t,
    maxPerStageDescriptorInputAttachments: uint32_t,
    maxPerStageResources: uint32_t,
    maxDescriptorSetSamplers: uint32_t,
    maxDescriptorSetUniformBuffers: uint32_t,
    maxDescriptorSetUniformBuffersDynamic: uint32_t,
    maxDescriptorSetStorageBuffers: uint32_t,
    maxDescriptorSetStorageBuffersDynamic: uint32_t,
    maxDescriptorSetSampledImages: uint32_t,
    maxDescriptorSetStorageImages: uint32_t,
    maxDescriptorSetInputAttachments: uint32_t,
    maxVertexInputAttributes: uint32_t,
    maxVertexInputBindings: uint32_t,
    maxVertexInputAttributeOffset: uint32_t,
    maxVertexInputBindingStride: uint32_t,
    maxVertexOutputComponents: uint32_t,
    maxTessellationGenerationLevel: uint32_t,
    maxTessellationPatchSize: uint32_t,
    maxTessellationControlPerVertexInputComponents: uint32_t,
    maxTessellationControlPerVertexOutputComponents: uint32_t,
    maxTessellationControlPerPatchOutputComponents: uint32_t,
    maxTessellationControlTotalOutputComponents: uint32_t,
    maxTessellationEvaluationInputComponents: uint32_t,
    maxTessellationEvaluationOutputComponents: uint32_t,
    maxGeometryShaderInvocations: uint32_t,
    maxGeometryInputComponents: uint32_t,
    maxGeometryOutputComponents: uint32_t,
    maxGeometryOutputVertices: uint32_t,
    maxGeometryTotalOutputComponents: uint32_t,
    maxFragmentInputComponents: uint32_t,
    maxFragmentOutputAttachments: uint32_t,
    maxFragmentDualSrcAttachments: uint32_t,
    maxFragmentCombinedOutputResources: uint32_t,
    maxComputeSharedMemorySize: uint32_t,
    maxComputeWorkGroupCount: [uint32_t;3],
    maxComputeWorkGroupInvocations: uint32_t,
    maxComputeWorkGroupSize: [uint32_t;3],
    subPixelPrecisionBits: uint32_t,
    subTexelPrecisionBits: uint32_t,
    mipmapPrecisionBits: uint32_t,
    maxDrawIndexedIndexValue: uint32_t,
    maxDrawIndirectCount: uint32_t,
    maxSamplerLodBias: c_float,
    maxSamplerAnisotropy: c_float,
    maxViewports: uint32_t,
    maxViewportDimensions: [uint32_t;2],
    viewportBoundsRange: [c_float;2],
    viewportSubPixelBits: uint32_t,
    minMemoryMapAlignment: size_t,
    minTexelBufferOffsetAlignment: VkDeviceSize,
    minUniformBufferOffsetAlignment: VkDeviceSize,
    minStorageBufferOffsetAlignment: VkDeviceSize,
    minTexelOffset: int32_t,
    maxTexelOffset: uint32_t,
    minTexelGatherOffset: int32_t,
    maxTexelGatherOffset: uint32_t,
    minInterpolationOffset: c_float,
    maxInterpolationOffset: c_float,
    subPixelInterpolationOffsetBits: uint32_t,
    maxFramebufferWidth: uint32_t,
    maxFramebufferHeight: uint32_t,
    maxFramebufferLayers: uint32_t,
    framebufferColorSampleCounts: VkSampleCountFlags,
    framebufferDepthSampleCounts: VkSampleCountFlags,
    framebufferStencilSampleCounts: VkSampleCountFlags,
    framebufferNoAttachmentsSampleCounts: VkSampleCountFlags,
    maxColorAttachments: uint32_t,
    sampledImageColorSampleCounts: VkSampleCountFlags,
    sampledImageIntegerSampleCounts: VkSampleCountFlags,
    sampledImageDepthSampleCounts: VkSampleCountFlags,
    sampledImageStencilSampleCounts: VkSampleCountFlags,
    storageImageSampleCounts: VkSampleCountFlags,
    maxSampleMaskWords: uint32_t,
    timestampComputeAndGraphics: VkBool32,
    timestampPeriod: c_float,
    maxClipDistances: uint32_t,
    maxCullDistances: uint32_t,
    maxCombinedClipAndCullDistances: uint32_t,
    discreteQueuePriorities: uint32_t,
    pointSizeRange: [c_float;2],
    lineWidthRange: [c_float;2],
    pointSizeGranularity: c_float,
    lineWidthGranularity: c_float,
    strictLines: VkBool32,
    standardSampleLocations: VkBool32,
    optimalBufferCopyOffsetAlignment: VkDeviceSize,
    optimalBufferCopyRowPitchAlignment: VkDeviceSize,
    nonCoherentAtomSize: VkDeviceSize
}

#[repr(C)]
pub struct VkPhysicalDeviceSparseProperties {
    residencyStandard2DBlockShape: VkBool32,
    residencyStandard2DMultisampleBlockShape: VkBool32,
    residencyStandard3DBlockShape: VkBool32,
    residencyAlignedMipSize: VkBool32,
    residencyNonResidentStrict: VkBool32
}

#[repr(C)]
pub struct VkPhysicalDeviceProperties {
    apiVersion: uint32_t,
    driverVersion: uint32_t,
    vendorID: uint32_t,
    deviceID: uint32_t,
    deviceType: VkPhysicalDeviceType,
    deviceName: [c_char;VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
    pipelineCacheUUID: [uint8_t;VK_UUID_SIZE],
    limits: VkPhysicalDeviceLimits,
    sparseProperties: VkPhysicalDeviceSparseProperties,
}

#[repr(C)]
pub struct VkQueueFamilyProperties {
    queueFlags: VkQueueFlags,
    queueCount: uint32_t,
    timestampValidBits: uint32_t,
    minImageTransferGranularity: VkExtent3D
}

#[repr(C)]
pub struct VkMemoryType {
    propertyFlags: VkMemoryPropertyFlags,
    heapIndex: uint32_t
}

#[repr(C)]
pub struct VkMemoryHeap {
    size: VkDeviceSize,
    flags: VkMemoryHeapFlags
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryProperties {
    memoryTypeCount: uint32_t,
    memoryTypes: [VkMemoryType;VK_MAX_MEMORY_TYPES],
    memoryHeapCount: uint32_t,
    memoryHeaps: [VkMemoryHeap;VK_MAX_MEMORY_HEAPS]
}

#[repr(C)]
pub struct VkDeviceQueueCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkDeviceQueueCreateFlags,
    queueFamilyIndex: uint32_t,
    queueCount: uint32_t,
    pQueuePriorities: *const c_float
}

#[repr(C)]
pub struct VkDeviceCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkDeviceCreateFlags,
    queueCreateInfoCount: uint32_t,
    pQueueCreateInfos: *const VkDeviceQueueCreateInfo,
    enabledLayerCount: uint32_t,
    ppEnabledLayerNames: *const *const c_char,
    enabledExtensionCount: uint32_t,
    ppEnabledExtensionNames: *const *const c_char,
    pEnabledFeatures: *const VkPhysicalDeviceFeatures
}

#[repr(C)]
pub struct VkExtensionProperties {
    extensionName: [c_char; VK_MAX_EXTENSION_NAME_SIZE],
    specVersion: uint32_t
}

#[repr(C)]
pub struct VkLayerProperties {
    layerName: [c_char;VK_MAX_EXTENSION_NAME_SIZE],
    specVersion: uint32_t,
    implementationVersion: uint32_t,
    description: [c_char;VK_MAX_DESCRIPTION_SIZE]
}

#[repr(C)]
pub struct VkSubmitInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    waitSemaphoreCount: uint32_t,
    pWaitSemaphores: *const VkSemaphore,
    pWaitDstStageMask: *const VkPipelineStageFlags,
    commandBufferCount: uint32_t,
    pCommandBuffers: *const VkCommandBuffer,
    signalSemaphoreCount: uint32_t,
    pSignalSemaphores: *const VkSemaphore
}

#[repr(C)]
pub struct VkMemoryAllocateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    allocationSize: VkDeviceSize,
    memoryTypeIndex: uint32_t,
}

#[repr(C)]
pub struct VkMappedMemoryRange {
    sType: VkStructureType,
    pNext: *const c_void,
    memory: VkDeviceMemory,
    offset: VkDeviceSize,
    size: VkDeviceSize
}

#[repr(C)]
pub struct VkMemoryRequirements {
    size: VkDeviceSize,
    alignment: VkDeviceSize,
    memoryTypeBits: uint32_t
}

#[repr(C)]
pub struct VkSparseImageFormatProperties {
    aspectMask: VkImageAspectFlags,
    imageGranularity: VkExtent3D,
    flags: VkSparseImageFormatFlags
}

#[repr(C)]
pub struct VkSparseImageMemoryRequirements {
    formatProperties: VkSparseImageFormatProperties,
    imageMipTailFirstLod: uint32_t,
    imageMipTailSize: VkDeviceSize,
    imageMipTailOffset: VkDeviceSize,
    imageMipTailStride: VkDeviceSize
}

#[repr(C)]
pub struct VkSparseMemoryBind {
    resourceOffset: VkDeviceSize,
    size: VkDeviceSize,
    memory: VkDeviceMemory,
    memoryOffset: VkDeviceSize,
    flags: VkSparseMemoryBindFlags
}

#[repr(C)]
pub struct VkSparseBufferMemoryBindInfo {
    buffer: VkBuffer,
    bindCount: uint32_t,
    pBinds: *const VkSparseMemoryBind
}

#[repr(C)]
pub struct VkSparseImageOpaqueMemoryBindInfo {
    image: VkImage,
    bindCount: uint32_t,
    pBinds: *const VkSparseMemoryBind
}

#[repr(C)]
pub struct VkImageSubresource {
    aspectMask: VkImageAspectFlags,
    mipLevel: uint32_t,
    arrayLayer: uint32_t
}

#[repr(C)]
pub struct VkOffset3D {
    x: int32_t,
    y: int32_t,
    z: int32_t
}

#[repr(C)]
pub struct VkSparseImageMemoryBind {
    subresource: VkImageSubresource,
    offset: VkOffset3D,
    extent: VkExtent3D,
    memory: VkDeviceMemory,
    memoryOffset: VkDeviceSize,
    flags: VkSparseMemoryBindFlags
}

#[repr(C)]
pub struct VkSparseImageMemoryBindInfo {
    image: VkImage,
    bindCount: uint32_t,
    pBinds: *const VkSparseImageMemoryBind
}

#[repr(C)]
pub struct VkBindSparseInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    waitSemaphoreCount: uint32_t,
    pWaitSemaphores: *const VkSemaphore,
    bufferBindCount: uint32_t,
    pBufferBinds: *const VkSparseBufferMemoryBindInfo,
    imageOpaqueBindCount: uint32_t,
    pImageOpaqueBinds: *const VkSparseImageOpaqueMemoryBindInfo,
    imageBindCount: uint32_t,
    pImageBinds: *const VkSparseImageMemoryBindInfo,
    signalSemaphoreCount: uint32_t,
    pSignalSemaphores: *const VkSemaphore
}

#[repr(C)]
pub struct VkFenceCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkFenceCreateFlags
}

#[repr(C)]
pub struct VkSemaphoreCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkSemaphoreCreateFlags
}

#[repr(C)]
pub struct VkEventCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkEventCreateFlags
}

#[repr(C)]
pub struct VkQueryPoolCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkQueryPoolCreateFlags,
    queryType: VkQueryType,
    queryCount: uint32_t,
    pipelineStatistics: VkQueryPipelineStatisticFlags,
}

#[repr(C)]
pub struct VkBufferCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkBufferCreateFlags,
    size: VkDeviceSize,
    usage: VkBufferUsageFlags,
    sharingMode: VkSharingMode,
    queueFamilyIndexCount: uint32_t,
    pQueueFamilyIndices: *const uint32_t
}

#[repr(C)]
pub struct VkBufferViewCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkBufferViewCreateFlags,
    buffer: VkBuffer,
    format: VkFormat,
    offset: VkDeviceSize,
    range: VkDeviceSize
}

#[repr(C)]
pub struct VkImageCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkImageCreateFlags,
    imageType: VkImageType,
    format: VkFormat,
    extent: VkExtent3D,
    mipLevels: uint32_t,
    arrayLayers: uint32_t,
    samples: VkSampleCountFlagBits,
    tiling: VkImageTiling,
    usage: VkImageUsageFlags,
    sharingMode: VkSharingMode,
    queueFamilyIndexCount: uint32_t,
    pQueueFamilyIndices: *const uint32_t,
    initialLayout: VkImageLayout
}

#[repr(C)]
pub struct VkSubresourceLayout {
    offset: VkDeviceSize,
    size: VkDeviceSize,
    rowPitch: VkDeviceSize,
    arrayPitch: VkDeviceSize,
    depthPitch: VkDeviceSize
}

#[repr(C)]
pub struct VkComponentMapping {
    r: VkComponentSwizzle,
    g: VkComponentSwizzle,
    b: VkComponentSwizzle,
    a: VkComponentSwizzle
}

#[repr(C)]
pub struct VkImageSubresourceRange {
    aspectMask: VkImageAspectFlags,
    baseMipLevel: uint32_t,
    levelCount: uint32_t,
    baseArrayLayer: uint32_t,
    layerCount: uint32_t
}

#[repr(C)]
pub struct VkImageViewCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkImageViewCreateFlags,
    image: VkImage,
    viewType: VkImageViewType,
    format: VkFormat,
    components: VkComponentMapping,
    subresourceRange: VkImageSubresourceRange
}

#[repr(C)]
pub struct VkShaderModuleCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkShaderModuleCreateFlags,
    codeSize: size_t,
    pCode: *const uint32_t
}

#[repr(C)]
pub struct VkPipelineCacheCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkPipelineCacheCreateFlags,
    initialDataSize: size_t,
    pInitialData: *const c_void
}

#[repr(C)]
pub struct VkSpecializationMapEntry {
    constantID: uint32_t,
    offset: uint32_t,
    size: size_t
}

#[repr(C)]
pub struct VkSpecializationInfo {
    mapEntryCount: uint32_t,
    pMapEntries: *const VkSpecializationMapEntry,
    dataSize: size_t,
    pData: *const c_void
}

#[repr(C)]
pub struct VkPipelineShaderStageCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkPipelineShaderStageCreateFlags,
    stage: VkShaderStageFlagBits,
    module: VkShaderModule,
    pName: *const c_char,
    pSpecializationInfo: *const VkSpecializationInfo
}

#[repr(C)]
pub struct VkVertexInputBindingDescription {
    binding: uint32_t,
    stride: uint32_t,
    inputRate: VkVertexInputRate
}

#[repr(C)]
pub struct VkVertexInputAttributeDescription {
    location: uint32_t,
    binding: uint32_t,
    format: VkFormat,
    offset: uint32_t
}

#[repr(C)]
pub struct VkPipelineVertexInputStateCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkPipelineVertexInputStateCreateFlags,
    vertexBindingDescriptionCount: uint32_t,
    pVertexBindingDescriptions: *const VkVertexInputBindingDescription,
    vertexAttributeDescriptionCount: uint32_t,
    pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription
}

#[repr(C)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkPipelineInputAssemblyStateCreateFlags,
    topology: VkPrimitiveTopology,
    primitiveRestartEnable: VkBool32
}

#[repr(C)]
pub struct VkPipelineTessellationStateCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkPipelineTessellationStateCreateFlags,
    patchControlPoints: uint32_t
}

#[repr(C)]
pub struct VkViewport {
    x: c_float,
    y: c_float,
    width: c_float,
    height: c_float,
    minDepth: c_float,
    maxDepth: c_float
}

#[repr(C)]
pub struct VkOffset2D {
    x: int32_t,
    y: int32_t
}

#[repr(C)]
pub struct VkExtent2D {
    width: uint32_t,
    height: uint32_t
}

#[repr(C)]
pub struct VkRect2D {
    offset: VkOffset2D,
    extent: VkExtent2D
}

#[repr(C)]
pub struct VkPipelineViewportStateCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkPipelineViewportStateCreateFlags,
    viewportCount: uint32_t,
    pViewports: *const VkViewport,
    scissorCount: uint32_t,
    pScissors: *const VkRect2D
}

#[repr(C)]
pub struct VkPipelineRasterizationStateCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkPipelineRasterizationStateCreateFlags,
    depthClampEnable: VkBool32,
    rasterizerDiscardEnable: VkBool32,
    polygonMode: VkPolygonMode,
    cullMode: VkCullModeFlags,
    frontFace: VkFrontFace,
    depthBiasEnable: VkBool32,
    depthBiasConstantFactor: c_float,
    depthBiasClamp: c_float,
    depthBiasSlopeFactor: c_float,
    lineWidth: c_float
}

#[repr(C)]
pub struct VkPipelineMultisampleStateCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkPipelineMultisampleStateCreateFlags,
    rasterizationSamples: VkSampleCountFlagBits,
    sampleShadingEnable: VkBool32,
    minSampleShading: c_float,
    pSampleMask: *const VkSampleMask,
    alphaToCoverageEnable: VkBool32,
    alphaToOneEnable: VkBool32
}

#[repr(C)]
pub struct VkStencilOpState {
    failOp: VkStencilOp,
    passOp: VkStencilOp,
    depthFailOp: VkStencilOp,
    compareOp: VkCompareOp,
    compareMask: uint32_t,
    writeMask: uint32_t,
    reference: uint32_t
}

#[repr(C)]
pub struct VkPipelineDepthStencilStateCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkPipelineDepthStencilStateCreateFlags,
    depthTestEnable: VkBool32,
    depthWriteEnable: VkBool32,
    depthCompareOp: VkCompareOp,
    depthBoundsTestEnable: VkBool32,
    stencilTestEnable: VkBool32,
    front: VkStencilOpState,
    back: VkStencilOpState,
    minDepthBounds: c_float,
    maxDepthBounds: c_float
}

#[repr(C)]
pub struct VkPipelineColorBlendAttachmentState {
    blendEnable: VkBool32,
    srcColorBlendFactor: VkBlendFactor,
    dstColorBlendFactor: VkBlendFactor,
    colorBlendOp: VkBlendOp,
    srcAlphaBlendFactor: VkBlendFactor,
    dstAlphaBlendFactor: VkBlendFactor,
    alphaBlendOp: VkBlendOp,
    colorWriteMask: VkColorComponentFlags
}

#[repr(C)]
pub struct VkPipelineColorBlendStateCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkPipelineColorBlendStateCreateFlags,
    logicOpEnable: VkBool32,
    logicOp: VkLogicOp,
    attachmentCount: uint32_t,
    pAttachments: *const VkPipelineColorBlendAttachmentState,
    blendConstants: [c_float;4]
}

#[repr(C)]
pub struct VkPipelineDynamicStateCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkPipelineDynamicStateCreateFlags,
    dynamicStateCount: uint32_t,
    pDynamicStates: *const VkDynamicState
}

#[repr(C)]
pub struct VkGraphicsPipelineCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkPipelineCreateFlags,
    stageCount: uint32_t,
    pStages: *const VkPipelineShaderStageCreateInfo,
    pVertexInputState: *const VkPipelineVertexInputStateCreateInfo,
    pInputAssemblyState: *const VkPipelineInputAssemblyStateCreateInfo,
    pTessellationState: *const VkPipelineTessellationStateCreateInfo,
    pViewportState: *const VkPipelineViewportStateCreateInfo,
    pRasterizationState: *const VkPipelineRasterizationStateCreateInfo,
    pMultisampleState: *const VkPipelineMultisampleStateCreateInfo,
    pDepthStencilState: *const VkPipelineDepthStencilStateCreateInfo,
    pColorBlendState: *const VkPipelineColorBlendStateCreateInfo,
    pDynamicState: *const VkPipelineDynamicStateCreateInfo,
    layout: VkPipelineLayout,
    renderPass: VkRenderPass,
    subpass: uint32_t,
    basePipelineHandle: VkPipeline,
    basePipelineIndex: int32_t
}

#[repr(C)]
pub struct VkComputePipelineCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkPipelineCreateFlags,
    stage: VkPipelineShaderStageCreateInfo,
    layout: VkPipelineLayout,
    basePipelineHandle: VkPipeline,
    basePipelineIndex: int32_t
}

#[repr(C)]
pub struct VkPushConstantRange {
    stageFlags: VkShaderStageFlags,
    offset: uint32_t,
    size: uint32_t
}

#[repr(C)]
pub struct VkPipelineLayoutCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkPipelineLayoutCreateFlags,
    setLayoutCount: uint32_t,
    pSetLayouts: *const VkDescriptorSetLayout,
    pushConstantRangeCount: uint32_t,
    pPushConstantRanges: *const VkPushConstantRange,
}

#[repr(C)]
pub struct VkSamplerCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkSamplerCreateFlags,
    magFilter: VkFilter,
    minFilter: VkFilter,
    mipmapMode: VkSamplerMipmapMode,
    addressModeU: VkSamplerAddressMode,
    addressModeV: VkSamplerAddressMode,
    addressModeW: VkSamplerAddressMode,
    mipLodBias: c_float,
    anisotropyEnable: VkBool32,
    maxAnisotropy: c_float,
    compareEnable: VkBool32,
    compareOp: VkCompareOp,
    minLod: c_float,
    maxLod: c_float,
    borderColor: VkBorderColor,
    unnormalizedCoordinates: VkBool32
}

#[repr(C)]
pub struct VkDescriptorSetLayoutBinding {
    binding: uint32_t,
    descriptorType: VkDescriptorType,
    descriptorCount: uint32_t,
    stageFlags: VkShaderStageFlags,
    pImmutableSamplers: *const VkSampler,
}

#[repr(C)]
pub struct VkDescriptorSetLayoutCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkDescriptorSetLayoutCreateFlags,
    bindingCount: uint32_t,
    pBindings: *const VkDescriptorSetLayoutBinding,
}


// TODO
#[repr(C)]
pub struct VkDescriptorPoolSize {
    /// Renamed from type to dType due to keyword collision
    dType: VkDescriptorType,
    descriptorCount: uint32_t,
}

#[repr(C)]
pub struct VkDescriptorPoolCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkDescriptorPoolCreateFlags,
    maxSets: uint32_t,
    poolSizeCount: uint32_t,
    pPoolSizes: *const VkDescriptorPoolSize
}

#[repr(C)]
pub struct VkDescriptorSetAllocateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    descriptorPool: VkDescriptorPool,
    descriptorSetCount: uint32_t,
    pSetLayouts: *const VkDescriptorSetLayout
}

#[repr(C)]
pub struct VkDescriptorImageInfo {
    sampler: VkSampler,
    imageView: VkImageView,
    imageLayout: VkImageLayout
}

#[repr(C)]
pub struct VkDescriptorBufferInfo {
    buffer: VkBuffer,
    offset: VkDeviceSize,
    range: VkDeviceSize
}

#[repr(C)]
pub struct VkWriteDescriptorSet {
    sType: VkStructureType,
    pNext: *const c_void,
    dstSet: VkDescriptorSet,
    dstBinding: uint32_t,
    dstArrayElement: uint32_t,
    descriptorCount: uint32_t,
    descriptorType: VkDescriptorType,
    pImageInfo: *const VkDescriptorImageInfo,
    pBufferInfo: *const VkDescriptorBufferInfo,
    pTexelBufferView: *const VkBufferView
}

#[repr(C)]
pub struct VkCopyDescriptorSet {
    sType: VkStructureType,
    pNext: *const c_void,
    srcSet: VkDescriptorSet,
    srcBinding: uint32_t,
    srcArrayElement: uint32_t,
    dstSet: VkDescriptorSet,
    dstBinding: uint32_t,
    dstArrayElement: uint32_t,
    descriptorCount: uint32_t
}

#[repr(C)]
pub struct VkFramebufferCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkFramebufferCreateFlags,
    renderPass: VkRenderPass,
    attachmentCount: uint32_t,
    pAttachments: *const VkImageView,
    width: uint32_t,
    height: uint32_t,
    layers: uint32_t
}

#[repr(C)]
pub struct VkAttachmentDescription {
    flags: VkAttachmentDescriptionFlags,
    format: VkFormat,
    samples: VkSampleCountFlagBits,
    loadOp: VkAttachmentLoadOp,
    storeOp: VkAttachmentStoreOp,
    stencilLoadOp: VkAttachmentLoadOp,
    stencilStoreOp: VkAttachmentStoreOp,
    initialLayout: VkImageLayout,
    finalLayout: VkImageLayout
}

#[repr(C)]
pub struct VkAttachmentReference {
    attachment: uint32_t,
    layout: VkImageLayout
}

#[repr(C)]
pub struct VkSubpassDescription {
    flags: VkSubpassDescriptionFlags,
    pipelineBindPoint: VkPipelineBindPoint,
    inputAttachmentCount: uint32_t,
    pInputAttachments: *const VkAttachmentReference,
    colorAttachmentCount: uint32_t,
    pColorAttachments: *const VkAttachmentReference,
    pResolveAttachments: *const VkAttachmentReference,
    pDepthStencilAttachment: *const VkAttachmentReference,
    preserveAttachmentCount: uint32_t,
    pPreserveAttachments: *const uint32_t
}

#[repr(C)]
pub struct VkSubpassDependency {
    srcSubpass: uint32_t,
    dstSubpass: uint32_t,
    srcStageMask: VkPipelineStageFlags,
    dstStageMask: VkPipelineStageFlags,
    srcAccessMask: VkAccessFlags,
    dstAccessMask: VkAccessFlags,
    dependencyFlags: VkDependencyFlags
}

#[repr(C)]
pub struct VkRenderPassCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkRenderPassCreateFlags,
    attachmentCount: uint32_t,
    pAttachments: *const VkAttachmentDescription,
    subpassCount: uint32_t,
    pSubpasses: *const VkSubpassDescription,
    dependencyCount: uint32_t,
    pDependencies: *const VkSubpassDependency
}

#[repr(C)]
pub struct VkCommandPoolCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkCommandPoolCreateFlags,
    queueFamilyIndex: uint32_t
}

#[repr(C)]
pub struct VkCommandBufferAllocateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    commandPool: VkCommandPool,
    level: VkCommandBufferLevel,
    commandBufferCount: uint32_t
}

#[repr(C)]
pub struct VkCommandBufferInheritanceInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    renderPass: VkRenderPass,
    subpass: uint32_t,
    framebuffer: VkFramebuffer,
    occlusionQueryEnable: VkBool32,
    queryFlags: VkQueryControlFlags,
    pipelineStatistics: VkQueryPipelineStatisticFlags
}

#[repr(C)]
pub struct VkCommandBufferBeginInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkCommandBufferUsageFlags,
    pInheritanceInfo: *const VkCommandBufferInheritanceInfo
}

#[repr(C)]
pub struct VkBufferCopy {
    srcOffset: VkDeviceSize,
    dstOffset: VkDeviceSize,
    size: VkDeviceSize
}

#[repr(C)]
pub struct VkImageSubresourceLayers {
    aspectMask: VkImageAspectFlags,
    mipLevel: uint32_t,
    baseArrayLayer: uint32_t,
    layerCount: uint32_t
}

#[repr(C)]
pub struct VkImageCopy {
    srcSubresource: VkImageSubresourceLayers,
    srcOffset: VkOffset3D,
    dstSubresource: VkImageSubresourceLayers,
    dstOffset: VkOffset3D,
    extent: VkExtent3D
}

#[repr(C)]
pub struct VkImageBlit {
    srcSubresource: VkImageSubresourceLayers,
    srcOffsets: [VkOffset3D;2],
    dstSubresource: VkImageSubresourceLayers,
    dstOffsets: [VkOffset3D;2]
}

#[repr(C)]
pub struct VkBufferImageCopy {
    bufferOffset: VkDeviceSize,
    bufferRowLength: uint32_t,
    bufferImageHeight: uint32_t,
    imageSubresource: VkImageSubresourceLayers,
    imageOffset: VkOffset3D,
    imageExtent: VkExtent3D
}

#[repr(C)]
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
                    VkClearColorValue{union_data:std::mem::transmute(color4f)}
                },
                VkClearColorValueUnion::Int32(color4i) => {
                    VkClearColorValue{union_data:std::mem::transmute(color4i)}
                },
                VkClearColorValueUnion::UInt32(color4u) => {
                    VkClearColorValue{union_data:std::mem::transmute(color4u)}
                },
            }
        }
    }
}

#[repr(C)]
pub struct VkClearDepthStencilValue {
    depth: c_float,
    stencil: uint32_t
}

#[repr(C)]
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
                    let mut clear_value:VkClearValue = std::mem::zeroed();
                    {
                        let clear_color_bytes: [u8;8] = std::mem::transmute(clear_depth_stencil_value);
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
pub struct VkClearAttachment {
    aspectMask: VkImageAspectFlags,
    colorAttachment: uint32_t,
    clearValue: VkClearValue
}

#[repr(C)]
pub struct VkClearRect {
    rect: VkRect2D,
    baseArrayLayer: uint32_t,
    layerCount: uint32_t
}

#[repr(C)]
pub struct VkImageResolve {
    srcSubresource: VkImageSubresourceLayers,
    srcOffset: VkOffset3D,
    dstSubresource: VkImageSubresourceLayers,
    dstOffset: VkOffset3D,
    extent: VkExtent3D
}

#[repr(C)]
pub struct VkMemoryBarrier {
    sType: VkStructureType,
    pNext: *const c_void,
    srcAccessMask: VkAccessFlags,
    dstAccessMask: VkAccessFlags
}

#[repr(C)]
pub struct VkBufferMemoryBarrier {
    sType: VkStructureType,
    pNext: *const c_void,
    srcAccessMask: VkAccessFlags,
    dstAccessMask: VkAccessFlags,
    srcQueueFamilyIndex: uint32_t,
    dstQueueFamilyIndex: uint32_t,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    size: VkDeviceSize
}

#[repr(C)]
pub struct VkImageMemoryBarrier {
    sType: VkStructureType,
    pNext: *const c_void,
    srcAccessMask: VkAccessFlags,
    dstAccessMask: VkAccessFlags,
    oldLayout: VkImageLayout,
    newLayout: VkImageLayout,
    srcQueueFamilyIndex: uint32_t,
    dstQueueFamilyIndex: uint32_t,
    image: VkImage,
    subresourceRange: VkImageSubresourceRange
}

#[repr(C)]
pub struct VkRenderPassBeginInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    renderPass: VkRenderPass,
    framebuffer: VkFramebuffer,
    renderArea: VkRect2D,
    clearValueCount: uint32_t,
    pClearValues: *const VkClearValue
}

#[repr(C)]
pub struct VkDispatchIndirectCommand {
    x: uint32_t,
    y: uint32_t,
    z: uint32_t
}

#[repr(C)]
pub struct VkDrawIndexedIndirectCommand {
    indexCount: uint32_t,
    instanceCount: uint32_t,
    firstIndex: uint32_t,
    vertexOffset: int32_t,
    firstInstance: uint32_t
}

#[repr(C)]
pub struct VkDrawIndirectCommand {
    vertexCount: uint32_t,
    instanceCount: uint32_t,
    firstVertex: uint32_t,
    firstInstance: uint32_t
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
                                                                                       samples: VkSampleCountFlagBits,
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

// TODO: make sure this is working
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
                                                            pipelineStage: VkPipelineStageFlagBits,
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

#[derive(Default)]
pub struct VulkanCore {
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

static VULKAN_LIBRARY: &'static str = "vulkan-1.dll";

impl VulkanCore {
    unsafe fn load_command(&self, instance: VkInstance, name: &str) -> Result<vkVoidFunctionFn, String> {
        let fn_ptr = self.vkGetInstanceProcAddr(instance, CString::new(name).unwrap().as_ptr());
        if fn_ptr != std::ptr::null() {
            Ok(fn_ptr)
        } else {
            Err(format!("Failed to load {}",name))
        }
    }

    pub fn new() -> Result<VulkanCore, String> {
        let mut vulkan_core: VulkanCore = Default::default();
        let library_path = Path::new(VULKAN_LIBRARY);
        vulkan_core.library = match DynamicLibrary::open(Some(library_path)) {
            Err(error) => return Err(format!("Failed to load {}: {}",VULKAN_LIBRARY,error)),
            Ok(library) => Some(library),
        };
        unsafe {
            vulkan_core.vkGetInstanceProcAddr = Some(std::mem::transmute(try!(vulkan_core.library.as_ref().unwrap().symbol::<u8>("vkGetInstanceProcAddr"))));
            vulkan_core.vkCreateInstance = Some(std::mem::transmute(try!(vulkan_core.load_command(VkInstance::null(), "vkCreateInstance"))));
            vulkan_core.vkEnumerateInstanceExtensionProperties = Some(std::mem::transmute(try!(vulkan_core.load_command(VkInstance::null(), "vkEnumerateInstanceExtensionProperties"))));
            vulkan_core.vkEnumerateInstanceLayerProperties = Some(std::mem::transmute(try!(vulkan_core.load_command(VkInstance::null(), "vkEnumerateInstanceLayerProperties"))));
        }
        Ok(vulkan_core)
    }

    pub unsafe fn vkGetInstanceProcAddr(&self, instance: VkInstance, pName: *const c_char) -> vkVoidFunctionFn {
        (self.vkGetInstanceProcAddr.as_ref().unwrap())(instance, pName)
    }

    pub unsafe fn vkCreateInstance(&self, 
                                   pCreateInfo: *const VkInstanceCreateInfo, 
                                   pAllocator: *const VkAllocationCallbacks, 
                                   pInstance: *mut VkInstance) -> VkResult {
        (self.vkCreateInstance.as_ref().unwrap())(pCreateInfo, pAllocator, pInstance)
    }

    // TODO: Write the stubs!
    // TODO: Maybe group the functions, so I don't have to unwrap them every time!

    pub fn load(&mut self, instance: VkInstance) -> Result<(), String> {
        unsafe {
            //self.vkCreateInstance = Some(std::mem::transmute(try!(self.load_command(instance, "vkCreateInstance"))));
            self.vkDestroyInstance = Some(std::mem::transmute(try!(self.load_command(instance, "vkDestroyInstance"))));
            self.vkEnumeratePhysicalDevices = Some(std::mem::transmute(try!(self.load_command(instance, "vkEnumeratePhysicalDevices"))));
            self.vkGetPhysicalDeviceFeatures = Some(std::mem::transmute(try!(self.load_command(instance, "vkGetPhysicalDeviceFeatures"))));
            self.vkGetPhysicalDeviceFormatProperties = Some(std::mem::transmute(try!(self.load_command(instance, "vkGetPhysicalDeviceFormatProperties"))));
            self.vkGetPhysicalDeviceImageFormatProperties = Some(std::mem::transmute(try!(self.load_command(instance, "vkGetPhysicalDeviceImageFormatProperties"))));
            self.vkGetPhysicalDeviceProperties = Some(std::mem::transmute(try!(self.load_command(instance, "vkGetPhysicalDeviceProperties"))));
            self.vkGetPhysicalDeviceQueueFamilyProperties = Some(std::mem::transmute(try!(self.load_command(instance, "vkGetPhysicalDeviceQueueFamilyProperties"))));
            self.vkGetPhysicalDeviceMemoryProperties = Some(std::mem::transmute(try!(self.load_command(instance, "vkGetPhysicalDeviceMemoryProperties"))));
            self.vkGetInstanceProcAddr = Some(std::mem::transmute(try!(self.load_command(instance, "vkGetInstanceProcAddr"))));
            self.vkGetDeviceProcAddr = Some(std::mem::transmute(try!(self.load_command(instance, "vkGetDeviceProcAddr"))));
            self.vkCreateDevice = Some(std::mem::transmute(try!(self.load_command(instance, "vkCreateDevice"))));
            self.vkDestroyDevice = Some(std::mem::transmute(try!(self.load_command(instance, "vkDestroyDevice"))));
            //self.vkEnumerateInstanceExtensionProperties = Some(std::mem::transmute(try!(self.load_command(instance, "vkEnumerateInstanceExtensionProperties"))));
            self.vkEnumerateDeviceExtensionProperties = Some(std::mem::transmute(try!(self.load_command(instance, "vkEnumerateDeviceExtensionProperties"))));
            //self.vkEnumerateInstanceLayerProperties = Some(std::mem::transmute(try!(self.load_command(instance, "vkEnumerateInstanceLayerProperties"))));
            self.vkEnumerateDeviceLayerProperties = Some(std::mem::transmute(try!(self.load_command(instance, "vkEnumerateDeviceLayerProperties"))));
            self.vkGetDeviceQueue = Some(std::mem::transmute(try!(self.load_command(instance, "vkGetDeviceQueue"))));
            self.vkQueueSubmit = Some(std::mem::transmute(try!(self.load_command(instance, "vkQueueSubmit"))));
            self.vkQueueWaitIdle = Some(std::mem::transmute(try!(self.load_command(instance, "vkQueueWaitIdle"))));
            self.vkDeviceWaitIdle = Some(std::mem::transmute(try!(self.load_command(instance, "vkDeviceWaitIdle"))));
            self.vkAllocateMemory = Some(std::mem::transmute(try!(self.load_command(instance, "vkAllocateMemory"))));
            self.vkFreeMemory = Some(std::mem::transmute(try!(self.load_command(instance, "vkFreeMemory"))));
            self.vkMapMemory = Some(std::mem::transmute(try!(self.load_command(instance, "vkMapMemory"))));
            self.vkUnmapMemory = Some(std::mem::transmute(try!(self.load_command(instance, "vkUnmapMemory"))));
            self.vkFlushMappedMemoryRanges = Some(std::mem::transmute(try!(self.load_command(instance, "vkFlushMappedMemoryRanges"))));
            self.vkInvalidateMappedMemoryRanges = Some(std::mem::transmute(try!(self.load_command(instance, "vkInvalidateMappedMemoryRanges"))));
            self.vkGetDeviceMemoryCommitment = Some(std::mem::transmute(try!(self.load_command(instance, "vkGetDeviceMemoryCommitment"))));
            self.vkBindBufferMemory = Some(std::mem::transmute(try!(self.load_command(instance, "vkBindBufferMemory"))));
            self.vkBindImageMemory = Some(std::mem::transmute(try!(self.load_command(instance, "vkBindImageMemory"))));
            self.vkGetBufferMemoryRequirements = Some(std::mem::transmute(try!(self.load_command(instance, "vkGetBufferMemoryRequirements"))));
            self.vkGetImageMemoryRequirements = Some(std::mem::transmute(try!(self.load_command(instance, "vkGetImageMemoryRequirements"))));
            self.vkGetImageSparseMemoryRequirements = Some(std::mem::transmute(try!(self.load_command(instance, "vkGetImageSparseMemoryRequirements"))));
            self.vkGetPhysicalDeviceSparseImageFormatProperties = Some(std::mem::transmute(try!(self.load_command(instance, "vkGetPhysicalDeviceSparseImageFormatProperties"))));
            self.vkQueueBindSparse = Some(std::mem::transmute(try!(self.load_command(instance, "vkQueueBindSparse"))));
            self.vkCreateFence = Some(std::mem::transmute(try!(self.load_command(instance, "vkCreateFence"))));
            self.vkDestroyFence = Some(std::mem::transmute(try!(self.load_command(instance, "vkDestroyFence"))));
            self.vkResetFences = Some(std::mem::transmute(try!(self.load_command(instance, "vkResetFences"))));
            self.vkGetFenceStatus = Some(std::mem::transmute(try!(self.load_command(instance, "vkGetFenceStatus"))));
            self.vkWaitForFences = Some(std::mem::transmute(try!(self.load_command(instance, "vkWaitForFences"))));
            self.vkCreateSemaphore = Some(std::mem::transmute(try!(self.load_command(instance, "vkCreateSemaphore"))));
            self.vkDestroySemaphore = Some(std::mem::transmute(try!(self.load_command(instance, "vkDestroySemaphore"))));
            self.vkCreateEvent = Some(std::mem::transmute(try!(self.load_command(instance, "vkCreateEvent"))));
            self.vkDestroyEvent = Some(std::mem::transmute(try!(self.load_command(instance, "vkDestroyEvent"))));
            self.vkGetEventStatus = Some(std::mem::transmute(try!(self.load_command(instance, "vkGetEventStatus"))));
            self.vkSetEvent = Some(std::mem::transmute(try!(self.load_command(instance, "vkSetEvent"))));
            self.vkResetEvent = Some(std::mem::transmute(try!(self.load_command(instance, "vkResetEvent"))));
            self.vkCreateQueryPool = Some(std::mem::transmute(try!(self.load_command(instance, "vkCreateQueryPool"))));
            self.vkDestroyQueryPool = Some(std::mem::transmute(try!(self.load_command(instance, "vkDestroyQueryPool"))));
            self.vkGetQueryPoolResults = Some(std::mem::transmute(try!(self.load_command(instance, "vkGetQueryPoolResults"))));
            self.vkCreateBuffer = Some(std::mem::transmute(try!(self.load_command(instance, "vkCreateBuffer"))));
            self.vkDestroyBuffer = Some(std::mem::transmute(try!(self.load_command(instance, "vkDestroyBuffer"))));
            self.vkCreateBufferView = Some(std::mem::transmute(try!(self.load_command(instance, "vkCreateBufferView"))));
            self.vkDestroyBufferView = Some(std::mem::transmute(try!(self.load_command(instance, "vkDestroyBufferView"))));
            self.vkCreateImage = Some(std::mem::transmute(try!(self.load_command(instance, "vkCreateImage"))));
            self.vkDestroyImage = Some(std::mem::transmute(try!(self.load_command(instance, "vkDestroyImage"))));
            self.vkGetImageSubresourceLayout = Some(std::mem::transmute(try!(self.load_command(instance, "vkGetImageSubresourceLayout"))));
            self.vkCreateImageView = Some(std::mem::transmute(try!(self.load_command(instance, "vkCreateImageView"))));
            self.vkDestroyImageView = Some(std::mem::transmute(try!(self.load_command(instance, "vkDestroyImageView"))));
            self.vkCreateShaderModule = Some(std::mem::transmute(try!(self.load_command(instance, "vkCreateShaderModule"))));
            self.vkDestroyShaderModule = Some(std::mem::transmute(try!(self.load_command(instance, "vkDestroyShaderModule"))));
            self.vkCreatePipelineCache = Some(std::mem::transmute(try!(self.load_command(instance, "vkCreatePipelineCache"))));
            self.vkDestroyPipelineCache = Some(std::mem::transmute(try!(self.load_command(instance, "vkDestroyPipelineCache"))));
            self.vkGetPipelineCacheData = Some(std::mem::transmute(try!(self.load_command(instance, "vkGetPipelineCacheData"))));
            self.vkMergePipelineCaches = Some(std::mem::transmute(try!(self.load_command(instance, "vkMergePipelineCaches"))));
            self.vkCreateGraphicsPipelines = Some(std::mem::transmute(try!(self.load_command(instance, "vkCreateGraphicsPipelines"))));
            self.vkCreateComputePipelines = Some(std::mem::transmute(try!(self.load_command(instance, "vkCreateComputePipelines"))));
            self.vkDestroyPipeline = Some(std::mem::transmute(try!(self.load_command(instance, "vkDestroyPipeline"))));
            self.vkCreatePipelineLayout = Some(std::mem::transmute(try!(self.load_command(instance, "vkCreatePipelineLayout"))));
            self.vkDestroyPipelineLayout = Some(std::mem::transmute(try!(self.load_command(instance, "vkDestroyPipelineLayout"))));
            self.vkCreateSampler = Some(std::mem::transmute(try!(self.load_command(instance, "vkCreateSampler"))));
            self.vkDestroySampler = Some(std::mem::transmute(try!(self.load_command(instance, "vkDestroySampler"))));
            self.vkCreateDescriptorSetLayout = Some(std::mem::transmute(try!(self.load_command(instance, "vkCreateDescriptorSetLayout"))));
            self.vkDestroyDescriptorSetLayout = Some(std::mem::transmute(try!(self.load_command(instance, "vkDestroyDescriptorSetLayout"))));
            self.vkCreateDescriptorPool = Some(std::mem::transmute(try!(self.load_command(instance, "vkCreateDescriptorPool"))));
            self.vkDestroyDescriptorPool = Some(std::mem::transmute(try!(self.load_command(instance, "vkDestroyDescriptorPool"))));
            self.vkResetDescriptorPool = Some(std::mem::transmute(try!(self.load_command(instance, "vkResetDescriptorPool"))));
            self.vkAllocateDescriptorSets = Some(std::mem::transmute(try!(self.load_command(instance, "vkAllocateDescriptorSets"))));
            self.vkFreeDescriptorSets = Some(std::mem::transmute(try!(self.load_command(instance, "vkFreeDescriptorSets"))));
            self.vkUpdateDescriptorSets = Some(std::mem::transmute(try!(self.load_command(instance, "vkUpdateDescriptorSets"))));
            self.vkCreateFramebuffer = Some(std::mem::transmute(try!(self.load_command(instance, "vkCreateFramebuffer"))));
            self.vkDestroyFramebuffer = Some(std::mem::transmute(try!(self.load_command(instance, "vkDestroyFramebuffer"))));
            self.vkCreateRenderPass = Some(std::mem::transmute(try!(self.load_command(instance, "vkCreateRenderPass"))));
            self.vkDestroyRenderPass = Some(std::mem::transmute(try!(self.load_command(instance, "vkDestroyRenderPass"))));
            self.vkGetRenderAreaGranularity = Some(std::mem::transmute(try!(self.load_command(instance, "vkGetRenderAreaGranularity"))));
            self.vkCreateCommandPool = Some(std::mem::transmute(try!(self.load_command(instance, "vkCreateCommandPool"))));
            self.vkDestroyCommandPool = Some(std::mem::transmute(try!(self.load_command(instance, "vkDestroyCommandPool"))));
            self.vkResetCommandPool = Some(std::mem::transmute(try!(self.load_command(instance, "vkResetCommandPool"))));
            self.vkAllocateCommandBuffers = Some(std::mem::transmute(try!(self.load_command(instance, "vkAllocateCommandBuffers"))));
            self.vkFreeCommandBuffers = Some(std::mem::transmute(try!(self.load_command(instance, "vkFreeCommandBuffers"))));
            self.vkBeginCommandBuffer = Some(std::mem::transmute(try!(self.load_command(instance, "vkBeginCommandBuffer"))));
            self.vkEndCommandBuffer = Some(std::mem::transmute(try!(self.load_command(instance, "vkEndCommandBuffer"))));
            self.vkResetCommandBuffer = Some(std::mem::transmute(try!(self.load_command(instance, "vkResetCommandBuffer"))));
            self.vkCmdBindPipeline = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdBindPipeline"))));
            self.vkCmdSetViewport = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdSetViewport"))));
            self.vkCmdSetScissor = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdSetScissor"))));
            self.vkCmdSetLineWidth = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdSetLineWidth"))));
            self.vkCmdSetDepthBias = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdSetDepthBias"))));
            self.vkCmdSetBlendConstants = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdSetBlendConstants"))));
            self.vkCmdSetDepthBounds = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdSetDepthBounds"))));
            self.vkCmdSetStencilCompareMask = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdSetStencilCompareMask"))));
            self.vkCmdSetStencilWriteMask = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdSetStencilWriteMask"))));
            self.vkCmdSetStencilReference = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdSetStencilReference"))));
            self.vkCmdBindDescriptorSets = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdBindDescriptorSets"))));
            self.vkCmdBindIndexBuffer = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdBindIndexBuffer"))));
            self.vkCmdBindVertexBuffers = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdBindVertexBuffers"))));
            self.vkCmdDraw = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdDraw"))));
            self.vkCmdDrawIndexed = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdDrawIndexed"))));
            self.vkCmdDrawIndirect = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdDrawIndirect"))));
            self.vkCmdDrawIndexedIndirect = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdDrawIndexedIndirect"))));
            self.vkCmdDispatch = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdDispatch"))));
            self.vkCmdDispatchIndirect = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdDispatchIndirect"))));
            self.vkCmdCopyBuffer = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdCopyBuffer"))));
            self.vkCmdCopyImage = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdCopyImage"))));
            self.vkCmdBlitImage = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdBlitImage"))));
            self.vkCmdCopyBufferToImage = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdCopyBufferToImage"))));
            self.vkCmdCopyImageToBuffer = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdCopyImageToBuffer"))));
            self.vkCmdUpdateBuffer = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdUpdateBuffer"))));
            self.vkCmdFillBuffer = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdFillBuffer"))));
            self.vkCmdClearColorImage = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdClearColorImage"))));
            self.vkCmdClearDepthStencilImage = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdClearDepthStencilImage"))));
            self.vkCmdClearAttachments = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdClearAttachments"))));
            self.vkCmdResolveImage = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdResolveImage"))));
            self.vkCmdSetEvent = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdSetEvent"))));
            self.vkCmdResetEvent = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdResetEvent"))));
            self.vkCmdWaitEvents = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdWaitEvents"))));
            self.vkCmdPipelineBarrier = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdPipelineBarrier"))));
            self.vkCmdBeginQuery = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdBeginQuery"))));
            self.vkCmdEndQuery = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdEndQuery"))));
            self.vkCmdResetQueryPool = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdResetQueryPool"))));
            self.vkCmdWriteTimestamp = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdWriteTimestamp"))));
            self.vkCmdCopyQueryPoolResults = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdCopyQueryPoolResults"))));
            self.vkCmdPushConstants = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdPushConstants"))));
            self.vkCmdBeginRenderPass = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdBeginRenderPass"))));
            self.vkCmdNextSubpass = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdNextSubpass"))));
            self.vkCmdEndRenderPass = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdEndRenderPass"))));
            self.vkCmdExecuteCommands = Some(std::mem::transmute(try!(self.load_command(instance, "vkCmdExecuteCommands"))));
        }
        Ok(())
    }
}
