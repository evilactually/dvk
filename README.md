Dvk is a library providing bindings to Vulkan API. Unlike many other alternatives Dvk loads all Vulkan commands dynamically at run time, making compilation much more straightforward, to the point that you don't even need Vulkan installed to compile it. 

This library is designed following the principle of minimum surprise, it deviates very little from the official headers and does not needlessly pollute official Vulkan namespace.  There are only a handful of places where either language differences or the requirment to load dynamically had forced design to deviate from canonical, all such peculiarities are thoroughly documented on this page. Regular Khronos documentation should be sufficient to learn about all the types and functions provided by this library. 

NOTE: In current version only khr_win32_surface is complete out of all platform-specific WSI extensions.

## Organization
All definitions are orginized into modules, the main one is *core*, the rest *khr_surface*, *ext_debug_report*, *khr_display*, *khr_display_swapchain*, *khr_swapchain*, *khr_win32_surface* are all extensions. This library does not export any read-to-use command prototypes. All definitions are in the same order as in *vulkan.h* header file.

## Changes to official API

### Types
* VkClearValueUnion
* VkClearColorValueUnion 
* VulkanCore
* VulkanKhrSurface
* VulkanKhrSwapchain
* VulkanKhrDisplay
* VulkanKhrDisplaySwapchain
* VulkanKhrWin32Surface
* VulkanExtDebugReport
* No separate ```*FlagBits``` and ```*Flags``` types just ```*Flags```
* VkDescriptorPoolSize.type is renamed to dType due to naming collision with Rust keyword type

### Functions
* Vulkan*::new() and Vulkan*::load(&mut self, VkInstance)
* ::null() and ::is_null(&self) for all handles
* From trait implementation for *Union types

## Loading

Dynamic loading has advantage over static linking in that no static library is needed to compile. Vulkan standard is fairly conservative on that point and only guarantees that a single command will be exported from the dynamic library. That command is *vkGetInstanceProcAddr*. Once that command is acquired, it can be used to load the next tier of API consisting of three *global commands*: 

1. vkCreateInstance
2. vkEnumerateInstanceExtensionProperties
3. vkEnumerateInstanceLayerProperties

The rest of the API, consisting of 134 core functions can similarly loaded with *vkGetInstanceProcAddr*, but require a *VkInstance* object to load them. A *VkInstance* object not surprisingly can be created via global command *vkCreateInstance*. Extension commands are loaded in exactly the same way.

This library does not export any read-to-use command prototypes, instead you get all commands dynamically loaded and returned in structs. 

The core of Vulkan functionality resides in *VulkanCore* struct. It provides all the core *Vulkan* commands as methods. When *VulkanCore* is initially created by calling *VulkanCore::new()*, it will already have the *3 global commands* loaded and ready to use. If you attempt to call any of the unloaded commands at this point it will result in *panic*. The next step should be to create a *VkInstance* object and call *VulkanCore::load(&mut self, VkInstance instance) method passing it as argument. Vulkan is ready to use.

Extensions are loaded similarly by *VulkanKhrSurface*, *VulkanKhrSwapchain*, *VulkanKhrDisplay*, *VulkanKhrDisplaySwapchain*, *VulkanKhrWin32Surface*

*One thing this library does not support is loading device optimized command pointers using vkGetDeviceProcAddr. The reason for this omission is that loading functions in this way introduces a lot of incidental complexity and makes library awkward to use.*

## Platform types

Platform types are redefined to avoid operating system specific dependencies, use ```std::mem::transmute``` to cast between them. The current platform types are:

* dvk::khr_win32_surface::platform::HINSTANCE
* dvk::khr_win32_surface::platform::HWND

## Unions
Since Rust has no analog to *C unions* they are simulated using combination of *tagged union types* and a *From trait*. Whenever Vulkan demands a union with a name VkSomeTypeName, construct a value of type VkSomeTypeNameUnion and call into(self) method on it to get VkSomeTypeName. For example:

	let foo: VkClearColorValue = VkClearColorValueUnion::Float32([1,2,3]).into();

## Handles
All handles are type-safe, which unfortunately makes it awkward to produce NULL handles. For that reason all handle types implement *null* function to construct empty handles, as well as corresponding method *is_null* to check if a handle is empty.

## Usage
Here's a short example to illustrate basing use

	#[macro_use]
	extern crate dvk;
	
	use dvk::core::*;
	use dvk::khr_surface::*;
	use dvk::khr_win32_surface::*;
	
	...
	// This will load vulkan shared library and 3 global commands
	let mut core = VulkanCore::new().unwrap(); 
	
	// The null method is used to get type-safe "NULL" handles
	let mut instance = VkInstance::null();
	
	// vkCreateInstance is one of the 3 global commands
	// that can be loaded without an instance object
	core.vkCreateInstance(&instance_create_info, null(), &mut context.instance);
	
	// Calling unloaded command will cause a panic
	core.vkEnumeratePhysicalDevices(...); // ERROR!
	
	// After you've acquired an instance object the remaining commands can be loaded
	core.load(instance).unwrap(); 
	
	// The rest of commands are loaded and ready to use now
	core.vkEnumeratePhysicalDevices(...); 
	core.vkCreateDevice(...); 
	core.vkQueueSubmit(...);
	
	// Using intermediate VkClearValueUnion Rust-style enum to 
	// construct VkClearValue corresponding to C-style union
	let clear_depth_stencil_value = VkClearDepthStencilValue{depth:0.0f32, stencil: 0u32};
	let clear_value: VkClearValue = VkClearValueUnion::DepthStencil(clear_depth_stencil_value).into();

# Sample code
A more complete example is available in ```examples/triangle.rs```. To compile( or run) it do:
```
> cargo build(or run) --examples triangle
```

