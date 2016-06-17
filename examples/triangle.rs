extern crate winapi;
extern crate gdi32;
extern crate user32;
extern crate kernel32;
extern crate libc;
#[macro_use]
extern crate dvk;

use std::mem::{size_of};
use std::ffi::{CString};
use std::ptr::*;
use winapi::*;
use gdi32::*;
use user32::*;
use kernel32::*;
use dvk::*;

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

struct Application {
    vulkan_core: VulkanCore,
    vk_instance: VkInstance
}

impl Application {
    pub fn new() -> Application{
        Application{vulkan_core: VulkanCore::new().unwrap(),
                    vk_instance: VkInstance::null()}
    }

    pub fn initialize(&mut self) {
        unsafe {
            let application_info = VkApplicationInfo {
                sType: VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
                pNext: null(),
                pApplicationName: CString::new("Triangle").unwrap().as_ptr(),
                applicationVersion: 1,
                pEngineName: null(),
                engineVersion: 0,
                apiVersion: VK_MAKE_VERSION!(1,0,0)};

            let instance_create_info = VkInstanceCreateInfo {
                sType: VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
                pNext: null(),
                flags: 0,
                pApplicationInfo: &application_info,
                enabledLayerCount: 0,
                ppEnabledLayerNames: null(),
                enabledExtensionCount: 0,
                ppEnabledExtensionNames: null()
            };
            self.vulkan_core.vkCreateInstance(&instance_create_info, null(), &mut self.vk_instance);
        }
    }
}

fn main() {
    Application::new().initialize();
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
