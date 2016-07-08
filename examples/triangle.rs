////////////////////////////////////////////////////////////////////////////////////////////////
//     Based on C++ demo & tutorial by José Henriques and released under equivalent terms.    //
//                 https://bitbucket.org/jose_henriques/vulkan_tutorial/src                   //
//                      http://av.dfki.de/~jhenriques/development.html                        //
////////////////////////////////////////////////////////////////////////////////////////////////

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
use dvk::core::*;
use dvk::khr_surface::*;
use dvk::khr_win32_surface::*;
use dvk::ext_debug_report::*;
use dvk::khr_swapchain::*;

unsafe extern "system" fn WindowProc(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    match uMsg {
        WM_CLOSE => { 
            PostQuitMessage(0);
        },
        _ => {}
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
    pub width: uint32_t,
    pub height: uint32_t,
    pub core: VulkanCore,
    pub ext_debug_report: VulkanExtDebugReport,
    pub khr_surface: VulkanKhrSurface,
    pub khr_win32_surface: VulkanKhrWin32Surface,
    pub khr_swapchain: VulkanKhrSwapchain,
    pub instance: VkInstance,
    pub debug_callback: VkDebugReportCallbackEXT,
    pub surface: VkSurfaceKHR,
    pub physicalDevice: VkPhysicalDevice,
    pub physicalDeviceProperties: VkPhysicalDeviceProperties,
    pub presentQueueIdx: uint32_t,
    pub device: VkDevice,
    pub swapChain: VkSwapchainKHR
}

impl VulkanContext {
    pub fn new() -> VulkanContext {
        unsafe {
            let mut context = std::mem::zeroed::<VulkanContext>();
            context.width = 640;
            context.height = 480;
            context.core = VulkanCore::new().unwrap();
            context.khr_surface = VulkanKhrSurface::new().unwrap();
            context.khr_win32_surface = VulkanKhrWin32Surface::new().unwrap();
            context.khr_swapchain = VulkanKhrSwapchain::new().unwrap();
            context.ext_debug_report = VulkanExtDebugReport::new().unwrap();
            context
        }
    }
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
            hbrBackground: GetStockObject(BLACK_BRUSH) as HBRUSH,
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
            flags: 0,
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
            flags: 0,
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
            .. std::mem::zeroed::<VkSwapchainCreateInfoKHR>()}; 

        let result = context.khr_swapchain.vkCreateSwapchainKHR(context.device,
                                                                &swapChainCreateInfo,
                                                                null(),
                                                                &mut context.swapChain);
        assert_eq!(result, VkResult::VK_SUCCESS);

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
