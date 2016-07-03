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
             IDC_ARROW, UINT, WNDCLASSEXW, HBRUSH, PM_REMOVE};
use gdi32::{GetStockObject};
use user32::{CreateWindowExW, RedrawWindow, RegisterClassExW, PostQuitMessage, LoadCursorW, DefWindowProcA,
             ShowWindow, PeekMessageW, DispatchMessageW, TranslateMessage};
use kernel32::{GetModuleHandleA};
use libc::{uint32_t, uint64_t, int32_t, size_t, c_char, c_void};
use std::mem::{transmute};
use dvk::core::*;
use dvk::khr_surface::*;
use dvk::khr_win32_surface::*;
use dvk::ext_debug_report::*;

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
    pub core: VulkanCore,
    pub ext_debug_report: VulkanExtDebugReport,
    pub khr_surface: VulkanKhrSurface,
    pub khr_win32_surface: VulkanKhrWin32Surface,
    pub instance: VkInstance,
    pub debug_callback: VkDebugReportCallbackEXT,
    pub surface: VkSurfaceKHR,
    pub physicalDevice: VkPhysicalDevice,
    pub physicalDeviceProperties: VkPhysicalDeviceProperties,
    pub presentQueueIdx: uint32_t
}

impl VulkanContext {
    pub fn new() -> VulkanContext {
        unsafe {
            let mut context = std::mem::zeroed::<VulkanContext>();
            context.core = VulkanCore::new().unwrap();
            context.khr_surface = VulkanKhrSurface::new().unwrap();
            context.khr_win32_surface = VulkanKhrWin32Surface::new().unwrap();
            context.ext_debug_report = VulkanExtDebugReport::new().unwrap();
            context
        }
    }
}

fn main() {
    unsafe {
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
        let hwnd = CreateWindowExW(0,
                                   class_atom as LPCWSTR,
                                   CString::new("Triangle").unwrap().as_ptr() as LPCWSTR,
                                   WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                                   100,
                                   100,
                                   800,
                                   600,
                                   null_mut(),
                                   null_mut(),
                                   hInstance,
                                   null_mut());

        let VK_LUNARG_STANDARD_VALIDATION_NAME = CString::new("VK_LAYER_LUNARG_standard_validation").unwrap();
        
        let mut context = VulkanContext::new();

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
        
        // debug callback
        let callbackCreateInfo = VkDebugReportCallbackCreateInfoEXT {
            sType: VkStructureType::VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
            pNext: null(),
            flags: VK_DEBUG_REPORT_ERROR_BIT_EXT |
                   VK_DEBUG_REPORT_WARNING_BIT_EXT |
                   VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT,
            pfnCallback: DebugReportCallback,
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

        // device
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
