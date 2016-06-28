#![feature(box_syntax)]
extern crate winapi;
extern crate gdi32;
extern crate user32;
extern crate kernel32;
extern crate libc;
#[macro_use]
extern crate dvk;
extern crate dvk_khr_surface;
extern crate dvk_khr_win32_surface;
extern crate dvk_ext_debug_report;

use std::mem::{size_of};
use std::ffi::{CString, CStr};
use std::ptr::{null, null_mut};
use winapi::*;
use gdi32::*;
use user32::*;
use kernel32::*;
use libc::{uint32_t, uint64_t, int32_t};
use dvk::*;
use dvk_khr_surface::*;
use dvk_khr_win32_surface::*;
use dvk_ext_debug_report::*;

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

unsafe extern "stdcall" fn DebugReportCallbackEXT(flags: VkDebugReportFlagsEXT,
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
    pub callback: VkDebugReportCallbackEXT
}

impl VulkanContext {
    pub fn new() -> VulkanContext {
        VulkanContext{core: VulkanCore::new().unwrap(),
                      khr_surface: VulkanKhrSurface::new().unwrap(),
                      khr_win32_surface: VulkanKhrWin32Surface::new().unwrap(),
                      ext_debug_report: VulkanExtDebugReport::new().unwrap(),
                      instance: VkInstance::null(),
                      callback: VkDebugReportCallbackEXT::null()}
    }
}

fn main() {
    unsafe {
        let instance = GetModuleHandleA(null());
        let window_class = WNDCLASSEXW {
            cbSize:size_of::<WNDCLASSEXW>() as UINT,
            style: CS_OWNDC | CS_VREDRAW | CS_HREDRAW,
            lpfnWndProc: Some(WindowProc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: instance,
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
                                   instance,
                                   null_mut());

        let VK_LUNARG_STANDARD_VALIDATION_NAME = CString::new("VK_LAYER_LUNARG_standard_validation").unwrap();

        // check validation layers
        let mut context = VulkanContext::new();
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

        // check extensions
        let mut extensionCount: uint32_t = 0;
        context.core.vkEnumerateInstanceExtensionProperties(null(), &mut extensionCount, null_mut());
        let mut extensionsAvailable:Vec<VkExtensionProperties> = Vec::with_capacity(extensionCount as usize);
        context.core.vkEnumerateInstanceExtensionProperties(null(), &mut extensionCount, extensionsAvailable.as_mut_ptr());
        extensionsAvailable.set_len(extensionCount as usize);

        let extensions = [VK_KHR_SURFACE_EXTENSION_NAME, VK_KHR_WIN32_SURFACE_EXTENSION_NAME];
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

        assert_eq!(context.core.vkCreateInstance(&instance_create_info, null(), &mut context.instance), VkResult::VK_SUCCESS);
        context.core.load(context.instance).unwrap();
        context.ext_debug_report.load(context.instance).unwrap();
        //context.khr_surface.load(context.instance).unwrap();
        //context.khr_win32_surface.load(context.instance).unwrap();
        

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
    }
}
