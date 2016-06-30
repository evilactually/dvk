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

// Calling unloaded command will cause a panic
core.vkEnumeratePhysicalDevices(...); // ERROR!

// After you've acquired an instance object the remaining commands can be loaded
core.load(instance).unwrap(); 

// The rest of commands are loaded and ready to use now
core.vkEnumeratePhysicalDevices(...); 
core.vkCreateDevice(...); 
core.vkQueueSubmit(...);
```
