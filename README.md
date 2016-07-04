# QUIRKS & PECULARITIES 
This library is designed following the principle of minimum surprise, it deviates very little from the official headers and does not needlessly pollute official Vulkan namespace. 
There are only a handful of places where either language differences or the requirment to load dynamically had forced design to deviate from cannonical.
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

### Functions
* Vulkan*::new() and Vulkan::load(&mut self, VkInstance)
* ::null() and ::is_null(&self) for all handles
* From trait implementation for *Union types

## Other notable deviations

* All commands are **loaded dynamically and returned in one of the a ```Vulkan*``` structs**, this library exports no ready-to-use function prototypes.

* Core commands are loaded in two stages, first global when you call ```Vulkan*::new()``` then instance-specific, when you call ```Vulkan::load(&mut self, VkInstance)```

* When you call ```VulkanCore::new()``` only the following 3 commands are loaded: ```vkCreateInstance```, ```vkEnumerateInstanceExtensionProperties``` and ```vkEnumerateInstanceLayerProperties```.

* Extension commands are always instance-specific, but still follow same loading pattern, first  ```VulkanKhr/Ext*::new()```, then ```VulkanKhr/Ext*::load(&mut self, VkInstance)```

* Platform types are redefined to reduce operating system specific dependencies, use ```std::mem::transmute``` to cast between them.

* Structs are not copyable/clonable because fixed arrays and function pointers in Rust are not. Enums, bitflags, handles, scalar-types are.

* Since Rust has no analog to C Unions they are simulated using combination of tagged union types and a From trait.

* All handle types implement ```null``` function to construct empty handles, as well as corresponding 
method ```is_null``` to check if a handle is empty.

## Usage
```
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
```
## Sample code
Sample code is available in ```examples/triangle.rs```. To compile( or run) do:
```
> cargo build(or run) --examples triangle
```

