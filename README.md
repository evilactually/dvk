# Welcome To The Family
Dvk is a family of packages providing definitions and dynamic loaders for core Vulkan API and it's extensions. Dynamic loading is the correct way to load Vulkan. Every package in Dvk family loads Vulkan commands dynamically in two stages, first loading global commands, then instance specific commands, ensuring in this way a correct operation even on the most conservative of systems. Changes to the API interface where they were dictated by language differences or the requirement to separate extensions into their own crates are introduced sparingly and in such a way that they will not alter or limit the functionality of the API as outlined by the official standard(s).

# About This Crate
This crate provides the definitions and a dynamic loader for core Vulkan API. To get access to commands you have to create a new instance of VulkanCore struct, by calling VulkanCore::new(). Initially only the 3 global commands will be loaded: vkEnumerateInstanceExtensionProperties, vkEnumerateInstanceLayerProperties, and vkCreateInstance. To load the remaining 134 commands you have to first create an instance using vkCreateInstance command and pass it to load method of VulkanCore. Current thread will panic if you attempt to call any of instance-specific functions before they were loaded.

Here's a short example to give you a feel this library:

```
#[macro_use]
extern crate dvk;

use dvk::*;

...
// This will load vulkan shared library and 3 global commands
let mut core = VulkanCore::new().unwrap(); 

// The null method is used to get type-safe "NULL" handles
let mut instance = VkInstance::null();

// vkCreateInstance is one of the 3 global commands
// that can be loaded without an instance object
core.vkCreateInstance(&instance_create_info, null(), &mut context.instance);

// After you've acquired an instance object the remaining commands can be loaded
core.load(instance).unwrap(); 

// The rest of commands are loaded and ready to use now
core.vkEnumeratePhysicalDevices(...); 
core.vkCreateDevice(...); 
core.vkQueueSubmit(...);
```
