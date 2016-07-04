# QUIRKS & PECULARITIES 

During design of these bindings I had to make some hard decisions, that may not necessarily please everyone, but due to the language differences between Rust and C they nevertheless had to be made.

* Core functions are loaded dynamically and returned in a struct.

* Core functions are loaded in two stages, first global, then instance-specific. 

* Extension functions are always instance-specific, but still follow same pattern.

* Platform types are redefined to reduce dependencies, use transmute to cast between them.

* Structs are not copyable/clonable. Enums, bitflags, handles, scalar-types are.

* C unions are simulated using combination of tagged union types and a From trait.

* There are no separate *FlagBits and *Flags types just *Flags

* All handle types implement "null" function to construct empty handles, as well as corresponding 
method "is_null" to check if a handle is empty.

```
#[macro_use]
extern crate dvk;

use dvk::core::*;

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

// Using intermediate VkClearValueUnion Rust-style enum to 
// construct VkClearValue corresponding to C-style union
let clear_depth_stencil_value = VkClearDepthStencilValue{depth:0.0f32, stencil: 0u32};
let clear_value: VkClearValue = VkClearValueUnion::DepthStencil(clear_depth_stencil_value).into();
