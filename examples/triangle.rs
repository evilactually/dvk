////////////////////////////////////////////////////////////////////////////////////////////////
//     Based on C++ demo & tutorial by José Henriques and released under equivalent terms.    //
//                 https://bitbucket.org/jose_henriques/vulkan_tutorial/src                   //
//                      http://av.dfki.de/~jhenriques/development.html                        //
////////////////////////////////////////////////////////////////////////////////////////////////

#![feature(associated_consts)]
#![feature(box_syntax)]
extern crate winapi;
extern crate gdi32;
extern crate user32;
extern crate kernel32;
extern crate libc;
#[macro_use]
extern crate dvk;

use std::mem::{size_of};
use std::ffi::{CString, CStr};
use std::ptr::{null, null_mut};
use winapi::{SW_SHOW, MSG, WM_CLOSE, HWND, WPARAM, LPARAM, LRESULT, CS_OWNDC, CS_VREDRAW, CS_HREDRAW,
             WM_QUIT, RDW_INTERNALPAINT, WS_OVERLAPPEDWINDOW, WS_VISIBLE, ATOM, LPCWSTR, BLACK_BRUSH,
             IDC_ARROW, UINT, WNDCLASSEXW, HBRUSH, PM_REMOVE, RECT};
use gdi32::{GetStockObject};
use user32::{CreateWindowExW, RedrawWindow, RegisterClassExW, PostQuitMessage, LoadCursorW, DefWindowProcA,
             ShowWindow, PeekMessageW, DispatchMessageW, TranslateMessage, AdjustWindowRect};
use kernel32::{GetModuleHandleA};
use libc::{uint32_t, uint64_t, int32_t, size_t, c_char, c_void};
use std::mem::{transmute};
use std::iter::{Repeat, repeat, FromIterator};
use dvk::core::*;
use dvk::khr_surface::*;
use dvk::khr_win32_surface::*;
use dvk::ext_debug_report::*;
use dvk::khr_swapchain::*;

static vk_context: *mut VulkanContext = 0 as *mut VulkanContext;

unsafe extern "system" fn WindowProc(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    match uMsg {
        WM_CLOSE => { 
            PostQuitMessage(0);
        },
        WM_PAINT => {
            //render();
        },
        //_ => {}
    }
    // a pass-through for now. We will return to this callback
    DefWindowProcA(hwnd, uMsg, wParam, lParam)
}

unsafe extern "stdcall" fn DebugReportCallback(flags: VkDebugReportFlagsEXT,
                                               objectType: VkDebugReportObjectTypeEXT,
                                               object: uint64_t,
                                               location: size_t,
                                               messageCode: int32_t,
                                               pLayerPrefix: *const c_char,
                                               pMessage: *const c_char,
                                               pUserData: *mut c_void) -> VkBool32 {
    println!("{:?} {:?}", CStr::from_ptr(pLayerPrefix), CStr::from_ptr(pMessage));
    VK_FALSE
}

struct VulkanContext {
    pub fragmanet_shader: &'static[u8],
    pub width: uint32_t,
    pub height: uint32_t,
    pub core: VkCoreCommands,
    pub ext_debug_report: VkExtDebugReportCommands,
    pub khr_surface: VkKhrSurfaceCommands,
    pub khr_win32_surface: VkKhrWin32SurfaceCommands,
    pub khr_swapchain: VkKhrSwapchainCommands,
    pub instance: VkInstance,
    pub debug_callback: VkDebugReportCallbackEXT,
    pub surface: VkSurfaceKHR,
    pub physicalDevice: VkPhysicalDevice,
    pub physicalDeviceProperties: VkPhysicalDeviceProperties,
    pub presentQueueIdx: uint32_t,
    pub presentQueue: VkQueue,
    pub device: VkDevice,
    pub swapChain: VkSwapchainKHR,
    pub presentImages:Vec<VkImage>,
    pub setupCmdBuffer: VkCommandBuffer,
    pub drawCmdBuffer: VkCommandBuffer
}

impl VulkanContext {
    pub fn new() -> VulkanContext {
        unsafe {
            let mut context = std::mem::zeroed::<VulkanContext>();
            context.width = 640;
            context.height = 480;
            context.core = VkCoreCommands::new().unwrap();
            context.khr_surface = VkKhrSurfaceCommands::new().unwrap();
            context.khr_win32_surface = VkKhrWin32SurfaceCommands::new().unwrap();
            context.khr_swapchain = VkKhrSwapchainCommands::new().unwrap();
            context.ext_debug_report = VkExtDebugReportCommands::new().unwrap();
            context.fragmanet_shader = include_bytes!("frag.spv");
            context
        }
    }
}

unsafe fn render(context: &VulkanContext) {
    let mut nextImageIdx: usize = 0;
    context.khr_swapchain.vkAcquireNextImageKHR(context.device,
                                                context.swapChain,
                                                u64::max_value(),
                                                VkSemaphore::null(),
                                                VkFence::null(),
                                                &mut nextImageIdx as *mut usize as *mut uint32_t);
    // render black
    let presentInfo = VkPresentInfoKHR {
        sType: VkStructureType::VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
        waitSemaphoreCount: 0,
        pWaitSemaphores: null(),
        swapchainCount: 1,
        pSwapchains: &context.swapChain,
        pImageIndices: &nextImageIdx as *const usize as *const uint32_t,
        .. std::mem::zeroed()
    };
    context.khr_swapchain.vkQueuePresentKHR(context.presentQueue, &presentInfo);
}

fn main() {
    unsafe {
        let mut context = VulkanContext::new();
        let hInstance = GetModuleHandleA(null());
        let window_class = WNDCLASSEXW {
            cbSize:size_of::<WNDCLASSEXW>() as UINT,
            style: CS_OWNDC | CS_VREDRAW | CS_HREDRAW,
            lpfnWndProc: Some(WindowProc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hInstance,
            hIcon: std::ptr::null_mut(),
            hCursor: LoadCursorW(std::ptr::null_mut(), IDC_ARROW),
            hbrBackground: null_mut(),//GetStockObject(BLACK_BRUSH) as HBRUSH,
            lpszMenuName: std::ptr::null_mut(),
            lpszClassName: CString::new("VulkanWindowClass").unwrap().as_ptr() as LPCWSTR,
            hIconSm: std::ptr::null_mut()
        };
        let class_atom : ATOM = RegisterClassExW(&window_class);
        let mut window_rectangle = RECT {left: 100,
                                         top: 100,
                                         right: 100 + context.width as i32,
                                         bottom: 100 + context.height as i32};
        AdjustWindowRect(&mut window_rectangle, WS_OVERLAPPEDWINDOW, 0);
        let hwnd = CreateWindowExW(0,
                                   class_atom as LPCWSTR,
                                   CString::new("Triangle").unwrap().as_ptr() as LPCWSTR,
                                   WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                                   window_rectangle.left,
                                   window_rectangle.top,
                                   window_rectangle.right - window_rectangle.left,
                                   window_rectangle.bottom - window_rectangle.top,
                                   null_mut(),
                                   null_mut(),
                                   hInstance,
                                   null_mut());
        let VK_LUNARG_STANDARD_VALIDATION_NAME = CString::new("VK_LAYER_LUNARG_standard_validation").unwrap();
        
        // check validation layers
        let mut layerCount: uint32_t = 0;
        context.core.vkEnumerateInstanceLayerProperties(&mut layerCount, null_mut());
        let mut layersAvailable:Vec<VkLayerProperties> = Vec::with_capacity(layerCount as usize);
        context.core.vkEnumerateInstanceLayerProperties(&mut layerCount, layersAvailable.as_mut_ptr());
        layersAvailable.set_len(layerCount as usize);

        assert!(layersAvailable.iter().any(|layer| {
            let name = CStr::from_ptr(&layer.layerName as *const c_char);
            name == VK_LUNARG_STANDARD_VALIDATION_NAME.as_ref()
        }));
        let layers = [VK_LUNARG_STANDARD_VALIDATION_NAME.as_ptr()];
        std::mem::drop(layersAvailable);
        std::mem::drop(layerCount);

        // check extensions
        let mut extensionCount: uint32_t = 0;
        context.core.vkEnumerateInstanceExtensionProperties(null(), &mut extensionCount, null_mut());
        let mut extensionsAvailable:Vec<VkExtensionProperties> = Vec::with_capacity(extensionCount as usize);
        context.core.vkEnumerateInstanceExtensionProperties(null(), &mut extensionCount, extensionsAvailable.as_mut_ptr());
        extensionsAvailable.set_len(extensionCount as usize);
        std::mem::drop(extensionCount);

        let extensions = [VK_EXT_DEBUG_REPORT_EXTENSION_NAME, VK_KHR_SURFACE_EXTENSION_NAME, VK_KHR_WIN32_SURFACE_EXTENSION_NAME];
        assert_eq!(extensionsAvailable.iter().fold(0, |count, extension| {
            let name = CStr::from_ptr(&extension.extensionName as *const c_char);
            let found = extensions.iter().any(|x| {
                CStr::from_ptr(*x) == name
            });
            if found {
                count + 1
            } else {
                count
            }
        }), extensions.len());
        std::mem::drop(extensionsAvailable);

        let application_info = VkApplicationInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: null(),
            pApplicationName: CString::new("Triangle").unwrap().as_ptr(),
            applicationVersion: 1,
            pEngineName: null(),
            engineVersion: 0,
            apiVersion: VK_MAKE_VERSION!(1,0,0)
        };

        let instance_create_info = VkInstanceCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: null(),
            flags: VkInstanceCreateFlags::empty(),
            pApplicationInfo: &application_info,
            enabledLayerCount: layers.len() as u32,
            ppEnabledLayerNames: &layers as *const *const c_char,
            enabledExtensionCount: extensions.len() as u32,
            ppEnabledExtensionNames: &extensions as *const *const c_char
        };
        std::mem::drop(layers);
        std::mem::drop(extensions);

        assert_eq!(context.core.vkCreateInstance(&instance_create_info, null(), &mut context.instance), VkResult::VK_SUCCESS);

        // load functions
        context.core.load(context.instance).unwrap();
        context.ext_debug_report.load(context.instance).unwrap();
        context.khr_surface.load(context.instance).unwrap();
        context.khr_win32_surface.load(context.instance).unwrap();
        context.khr_swapchain.load(context.instance).unwrap();
        
        // debug callback
        let callbackCreateInfo = VkDebugReportCallbackCreateInfoEXT {
            sType: VkStructureType::VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
            pNext: null(),
            flags: VK_DEBUG_REPORT_ERROR_BIT_EXT |
                   VK_DEBUG_REPORT_WARNING_BIT_EXT |
                   VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT,
            pfnCallback: Some(DebugReportCallback),
            pUserData: null_mut()
        };

        let result = context.ext_debug_report.vkCreateDebugReportCallbackEXT(context.instance, 
                                                                             &callbackCreateInfo,
                                                                             null_mut(),
                                                                             &mut context.debug_callback);
        assert_eq!(result, VkResult::VK_SUCCESS);
        
        // surface
        let surfaceCreateInfo = VkWin32SurfaceCreateInfoKHR {
            sType: VkStructureType::VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
            pNext: null(),
            flags: VkWin32SurfaceCreateFlagsKHR::empty(),
            hinstance: transmute(hInstance),
            hwnd: transmute(hwnd)};

        let result = context.khr_win32_surface.vkCreateWin32SurfaceKHR(context.instance, &surfaceCreateInfo, null(), &mut context.surface);
        assert_eq!(result, VkResult::VK_SUCCESS);

        // find physical device and queue family index
        let mut physicalDeviceCount: uint32_t = 0;
        context.core.vkEnumeratePhysicalDevices(context.instance, &mut physicalDeviceCount, null_mut());
        let mut physicalDevices:Vec<VkPhysicalDevice> = Vec::with_capacity(physicalDeviceCount as usize);
        context.core.vkEnumeratePhysicalDevices(context.instance, &mut physicalDeviceCount, physicalDevices.as_mut_ptr());
        physicalDevices.set_len(physicalDeviceCount as usize);
        
        for physicalDevice in physicalDevices {
            let mut deviceProperties = std::mem::zeroed::<VkPhysicalDeviceProperties>();
            context.core.vkGetPhysicalDeviceProperties(physicalDevice, &mut deviceProperties);

            let mut queueFamilyCount: uint32_t = 0;
            context.core.vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice, &mut queueFamilyCount, null_mut());
            let mut queueFamilyProperties:Vec<VkQueueFamilyProperties> = Vec::with_capacity(queueFamilyCount as usize);
            context.core.vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice, 
                                                                  &mut queueFamilyCount, 
                                                                  queueFamilyProperties.as_mut_ptr());
            queueFamilyProperties.set_len(queueFamilyCount as usize);

            let queueIndexSearchResult = queueFamilyProperties.iter().enumerate().find(|&(index, properties)| {
                let mut supportsPresent: VkBool32 = 0;
                context.khr_surface.vkGetPhysicalDeviceSurfaceSupportKHR(physicalDevice,
                                                                         index as uint32_t,
                                                                         context.surface, 
                                                                         &mut supportsPresent);
                (supportsPresent == VK_TRUE) && properties.queueFlags.contains(VK_QUEUE_GRAPHICS_BIT)
            });

            if let Some((queueFamilyIndex, _)) = queueIndexSearchResult {
                context.physicalDevice = physicalDevice;
                context.physicalDeviceProperties = deviceProperties;
                context.presentQueueIdx = queueFamilyIndex as uint32_t;
                break;
            };
        }
        std::mem::drop(physicalDeviceCount);
        assert!(!context.physicalDevice.is_null());

        // acquire logical device
        let mut queueCreateInfo = std::mem::zeroed::<VkDeviceQueueCreateInfo>();
        queueCreateInfo.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
        queueCreateInfo.queueFamilyIndex = context.presentQueueIdx;
        queueCreateInfo.queueCount = 1;
        let queuePriorities = [1.0f32];
        queueCreateInfo.pQueuePriorities = &queuePriorities as *const f32;

        let mut deviceInfo = std::mem::zeroed::<VkDeviceCreateInfo>();
        deviceInfo.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
        deviceInfo.queueCreateInfoCount = 1;
        deviceInfo.pQueueCreateInfos = &queueCreateInfo;
        deviceInfo.enabledLayerCount = 1;
        deviceInfo.ppEnabledLayerNames = &layers as *const *const c_char;

        let deviceExtensions = [VK_KHR_SWAPCHAIN_EXTENSION_NAME];
        deviceInfo.enabledExtensionCount = 1;
        deviceInfo.ppEnabledExtensionNames = &deviceExtensions as *const *const c_char;

        let mut features = std::mem::zeroed::<VkPhysicalDeviceFeatures>();
        features.shaderClipDistance = VK_TRUE;
        deviceInfo.pEnabledFeatures = &features;

        let result = context.core.vkCreateDevice(context.physicalDevice, &deviceInfo, null(), &mut context.device);
        assert_eq!(result, VkResult::VK_SUCCESS);

        // get queue
        context.core.vkGetDeviceQueue(context.device, context.presentQueueIdx, 0, &mut context.presentQueue);

        // swap chain

        // format
        let mut formatCount: uint32_t = 0;
        context.khr_surface.vkGetPhysicalDeviceSurfaceFormatsKHR(context.physicalDevice, context.surface, &mut formatCount, null_mut());
        let mut surfaceFormats:Vec<VkSurfaceFormatKHR> = Vec::with_capacity(formatCount as usize);
        context.khr_surface.vkGetPhysicalDeviceSurfaceFormatsKHR(context.physicalDevice,
                                                                 context.surface,
                                                                 &mut formatCount,
                                                                 surfaceFormats.as_mut_ptr());
        surfaceFormats.set_len(formatCount as usize);

        // If the format list includes just one entry of VK_FORMAT_UNDEFINED, the surface has
        // no preferred format. Otherwise, at least one supported format will be returned.
        let colorFormat: VkFormat = if(formatCount == 1 && surfaceFormats[0].format == VkFormat::VK_FORMAT_UNDEFINED) {
            VkFormat::VK_FORMAT_B8G8R8_UNORM
        } else {
            surfaceFormats[0].format
        };

        let colorSpace: VkColorSpaceKHR = surfaceFormats[0].colorSpace;
        std::mem::drop(surfaceFormats);

        let mut surfaceCapabilities = std::mem::zeroed::<VkSurfaceCapabilitiesKHR>();
        context.khr_surface.vkGetPhysicalDeviceSurfaceCapabilitiesKHR(context.physicalDevice, context.surface, &mut surfaceCapabilities);

        // image count

        // we are effectively looking for double-buffering:
        // if surfaceCapabilities.maxImageCount == 0 there is actually no limit on the number of images! 
        let mut desiredImageCount: uint32_t = 2;
        if desiredImageCount < surfaceCapabilities.minImageCount {
            desiredImageCount = surfaceCapabilities.minImageCount;
        } else if surfaceCapabilities.maxImageCount != 0 && 
                  desiredImageCount > surfaceCapabilities.maxImageCount {
            desiredImageCount = surfaceCapabilities.maxImageCount;
        }

        // resolution

        let mut surfaceResolution: VkExtent2D = surfaceCapabilities.currentExtent;
        if surfaceResolution.width == (-1i32) as u32 {
            surfaceResolution.width = context.width;
            surfaceResolution.height = context.height;
        } else {
            context.width = surfaceResolution.width;
            context.height = surfaceResolution.height;
        }

        let preTransform = if surfaceCapabilities.supportedTransforms.contains(VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR) {
            VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR
        } else {
            surfaceCapabilities.currentTransform
        };

        // present mode
        let mut presentModeCount: uint32_t = 0;
        context.khr_surface.vkGetPhysicalDeviceSurfacePresentModesKHR(context.physicalDevice,
                                                                      context.surface, 
                                                                      &mut presentModeCount,
                                                                      null_mut());
        let mut presentModes:Vec<VkPresentModeKHR> = Vec::with_capacity(presentModeCount as usize);
        context.khr_surface.vkGetPhysicalDeviceSurfacePresentModesKHR(context.physicalDevice,
                                                                      context.surface, 
                                                                      &mut presentModeCount,
                                                                      presentModes.as_mut_ptr());
        presentModes.set_len(presentModeCount as usize);

        let mail_box_supported = presentModes.iter().any(|presentMode| {
            *presentMode == VkPresentModeKHR::VK_PRESENT_MODE_MAILBOX_KHR
        });

        let presentationMode = if mail_box_supported {
            VkPresentModeKHR::VK_PRESENT_MODE_MAILBOX_KHR
        } else {
            VkPresentModeKHR::VK_PRESENT_MODE_FIFO_KHR
        };
        std::mem::drop(presentModes);

        // create swap chain
        let swapChainCreateInfo = VkSwapchainCreateInfoKHR {
            sType: VkStructureType::VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
            surface: context.surface,
            minImageCount: desiredImageCount,
            imageFormat: colorFormat,
            imageColorSpace: colorSpace,
            imageExtent: surfaceResolution,
            imageArrayLayers: 1,
            imageUsage: VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
            imageSharingMode: VkSharingMode::VK_SHARING_MODE_EXCLUSIVE,   // <--
            preTransform: preTransform,
            compositeAlpha: VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
            presentMode: presentationMode,
            clipped: VK_TRUE, // If we want clipping outside the extents
            .. std::mem::zeroed()
        }; 

        let result = context.khr_swapchain.vkCreateSwapchainKHR(context.device,
                                                                &swapChainCreateInfo,
                                                                null(),
                                                                &mut context.swapChain);
        assert_eq!(result, VkResult::VK_SUCCESS);

        // command pool
        let commandPoolCreateInfo = VkCommandPoolCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
            flags: VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
            queueFamilyIndex: context.presentQueueIdx,
            .. std::mem::zeroed()
        };

        let mut commandPool = VkCommandPool::null();
        let result = context.core.vkCreateCommandPool(context.device, &commandPoolCreateInfo, null(), &mut commandPool);
        assert_eq!(result, VkResult::VK_SUCCESS);

        let commandBufferAllocationInfo = VkCommandBufferAllocateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
            commandPool: commandPool,
            level: VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY,
            commandBufferCount: 1,
            .. std::mem::zeroed()
        };

        let result = context.core.vkAllocateCommandBuffers(context.device, &commandBufferAllocationInfo, &mut context.setupCmdBuffer);
        assert_eq!(result, VkResult::VK_SUCCESS);

        let result = context.core.vkAllocateCommandBuffers(context.device, &commandBufferAllocationInfo, &mut context.drawCmdBuffer);
        assert_eq!(result, VkResult::VK_SUCCESS);

        // get swapchain images
        let mut imageCount: uint32_t = 0;
        context.khr_swapchain.vkGetSwapchainImagesKHR(context.device, context.swapChain, &mut imageCount, null_mut());
        context.presentImages = Vec::with_capacity(imageCount as usize);
        context.khr_swapchain.vkGetSwapchainImagesKHR(context.device, context.swapChain, &mut imageCount, context.presentImages.as_mut_ptr());
        context.presentImages.set_len(imageCount as usize);

        // create VkImageViews for our swap chain VkImages buffers:
        let presentImagesViewCreateInfoTemplate = VkImageViewCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
            viewType: VkImageViewType::VK_IMAGE_VIEW_TYPE_2D,
            format: colorFormat,
            components: VkComponentMapping {
                r: VkComponentSwizzle::VK_COMPONENT_SWIZZLE_R,
                g: VkComponentSwizzle::VK_COMPONENT_SWIZZLE_G,
                b: VkComponentSwizzle::VK_COMPONENT_SWIZZLE_B,
                a: VkComponentSwizzle::VK_COMPONENT_SWIZZLE_A 
            },
            subresourceRange: VkImageSubresourceRange {
                aspectMask: VK_IMAGE_ASPECT_COLOR_BIT,
                baseMipLevel: 0,
                levelCount: 1,
                baseArrayLayer: 0,
                layerCount: 1 },
            .. std::mem::zeroed()
        };

        let beginInfo = VkCommandBufferBeginInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
            flags: VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
            .. std::mem::zeroed()
        };

        let fenceCreateInfo = VkFenceCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
            flags: VkFenceCreateFlags::empty(),
            .. std::mem::zeroed()
        };
        let mut submitFence = VkFence::null();
        context.core.vkCreateFence(context.device, &fenceCreateInfo, null(), &mut submitFence);

        let mut transitioned:Vec<bool> = Vec::from_iter(repeat(false).take(imageCount as usize));

        let mut doneCount: uint32_t = 0;
        while(doneCount != imageCount) {
            println!("{:?}", doneCount);
            let mut presentCompleteSemaphore = VkSemaphore::null();
            let semaphoreCreateInfo = VkSemaphoreCreateInfo {
                sType: VkStructureType::VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
                .. std::mem::zeroed()
            };
            context.core.vkCreateSemaphore(context.device, &semaphoreCreateInfo, null(), &mut presentCompleteSemaphore);

            let mut nextImageIdx: usize = 0;
            context.khr_swapchain.vkAcquireNextImageKHR(context.device,
                                                        context.swapChain,
                                                        u64::max_value(),
                                                        presentCompleteSemaphore,
                                                        VkFence::null(),
                                                        &mut nextImageIdx as *mut usize as *mut uint32_t);
         
            if(!transitioned[nextImageIdx]) {
                // start recording out image layout change barrier on our setup command buffer:
                context.core.vkBeginCommandBuffer(context.setupCmdBuffer, &beginInfo);

                let layoutTransitionBarrier = VkImageMemoryBarrier {
                    sType: VkStructureType::VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
                    srcAccessMask: VkAccessFlags::empty(),
                    dstAccessMask: VK_ACCESS_MEMORY_READ_BIT,
                    oldLayout: VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED,
                    newLayout: VkImageLayout::VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
                    srcQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
                    dstQueueFamilyIndex: VK_QUEUE_FAMILY_IGNORED,
                    image: context.presentImages[nextImageIdx],
                    subresourceRange: VkImageSubresourceRange { 
                        aspectMask: VK_IMAGE_ASPECT_COLOR_BIT,
                        baseMipLevel: 0,
                        levelCount: 1,
                        baseArrayLayer: 0,
                        layerCount:1
                    },
                    .. std::mem::zeroed()
                };

                context.core.vkCmdPipelineBarrier(context.setupCmdBuffer,
                                                  VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT, 
                                                  VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT, 
                                                  VkDependencyFlags::empty(),
                                                  0,
                                                  null(),
                                                  0,
                                                  null(), 
                                                  1,
                                                  &layoutTransitionBarrier);

                context.core.vkEndCommandBuffer(context.setupCmdBuffer);

                let waitStageMask = [VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT];
                let submitInfo = VkSubmitInfo {
                    sType: VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO,
                    waitSemaphoreCount: 1,
                    pWaitSemaphores: &presentCompleteSemaphore,
                    pWaitDstStageMask: &waitStageMask as *const VkPipelineStageFlags,
                    commandBufferCount: 1,
                    pCommandBuffers: &context.setupCmdBuffer,
                    signalSemaphoreCount: 0,
                    pSignalSemaphores: null(),
                    .. std::mem::zeroed()
                };

                let result = context.core.vkQueueSubmit(context.presentQueue, 1, &submitInfo, submitFence);

                context.core.vkWaitForFences(context.device, 1, &submitFence, VK_TRUE, u64::max_value());
                context.core.vkResetFences(context.device, 1, &submitFence);
                context.core.vkDestroySemaphore(context.device, presentCompleteSemaphore, null());
                context.core.vkResetCommandBuffer(context.setupCmdBuffer, VkCommandBufferResetFlags::empty());
                
                transitioned[nextImageIdx] = true;
                doneCount += 1;
            }
        }
        std::mem::drop(transitioned);

        // create image views
        let mut presentImageViews:Vec<VkImageView> = Vec::with_capacity(imageCount as usize);
        presentImageViews.set_len(imageCount as usize);
        for i in 0..(imageCount as usize) {
            let presentImagesViewCreateInfo = VkImageViewCreateInfo {
                image: context.presentImages[i],
                .. presentImagesViewCreateInfoTemplate
            };
            let result = context.core.vkCreateImageView(context.device, &presentImagesViewCreateInfo, null(), &mut presentImageViews[i]);
            assert_eq!(result, VkResult::VK_SUCCESS);
        }

        ShowWindow(hwnd, SW_SHOW);
        let mut msg: MSG = std::mem::zeroed();
        let mut done: bool = false;
        while (!done)
        {
            PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE);
            if (msg.message == WM_QUIT)
            {
                done = true;
            } else {
                // Translate keystroke messages into the right format
                TranslateMessage(&mut msg);
                // Send the message to the WindowProc function
                DispatchMessageW(&mut msg);
            }
            RedrawWindow(hwnd, null_mut(), null_mut(), RDW_INTERNALPAINT);
        }
        // TODO: this crashes
        //context.ext_debug_report.vkDestroyDebugReportCallbackEXT(context.instance, context.debug_callback, null());
    }
}
