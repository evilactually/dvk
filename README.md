QUIRKS & NOTES
--------------

Some extended enums are meant to be used with core functions, for those I have to provide a conversion trait From to convert to original enum being extended. Existance of this trait gives you a "legal" right to use VkKhrSwapchainImageLayout anywhere VkImageLayout is used, provided you use from or into methods to convert them. Conversion is provided only one way down to "master" type.

Because in rust you can't just simply call a function pointer stored in a struct using normal syntax, unlike in C, and have to wrap it up
in a pair useless parenthesis I had to write 500 lines of wrapper functions. Thanks Rust.

The reason for these wrappers here are Rust's Option<non-nullable function pointers> and (odd)(call,syntax)

Right now vulkan has no extended bitflags, but if it ever will, I will use the same convention as for enums

Extension modules have a custom VkResult even if they don't extend VkResult, this is done in case if in the future some of the function might add new rturn types.

I don't like macros. Take it whichever way you like, but this is reflected in my code. I'd rather repeat and copy-paste things than have code that looks like a print out of internal syntax tree data structure, besides I found them not capabale of even doing the most trivial things like generating compile time byte strings.

Vulkan has a little annoying convention to lump enums from different extension together, this is at odds with my decision to separate
extensions into their own packages. I wonder what Joe did here? But here's what I will do. For each type the extension needs to extend, it will
have it's own version of that type, the type will have everything needed by the extension for return types, and for input types, everything it had before, plus the extended stuff.

Naming follows a pattern:
Vk<NAME OF EXTENSION><ORIGINAL ENUM NAME>

Examples:
VkResult -> VkExtDebugReportResult, VkKhrSurfaceResult, VkKhrWin32SurfaceResult

My strings are &str type not C-strings, so you have to convert them yourself

load_global_commands - you use it to get initial global commands and create an instance
load_instance_commands - once you have an instance you can load the rest of the core api with this
                         it will not reload global commands!
                         you don't have to load globals to call this
load_device_commands - optionally you can load device specific commands if you have a device object
                       it will overwrite any functions loaded by load_instance_commands with device specific ones
                       you don't have to load globals or instance to call this, but you will probably want to if you want a complete api

vkVoidFunctionFn is not defined as a void function, but instead as a raw pointer to allow possibility that it might be null. This type 
is used by vulkan to return generic function pointers.

We don't panic on bad loads. Instead of mutating a struct, i take ownership, and return it back wrapped in result
This design eluminates possibility of having VulkanCore in undefined state.

I don't have VK_NULL_HANDLE, instead there is ::null method on each handle type, that constructs a null handle of that type

NonDispatchable handle are simply declared tuple structs, not macros or fancy crap, the whole point is to keep them type safe
and 64-bit, that's what tuple struct can achieve in a single line. There's no need to copy C++ hacks into Rust, they had other problems.

Dispatchable handles have a slightly different requirement, they must be same size as native pointer, maybe tuple struct with a pointer?

function arguments are const by default, so all const in front of arguments are dropped as in vkCmdSetBlendConstantsFn
Unions
"type" -> dType, iType, ...
Loading
enums are u32 or i32, MAX field dropped
enums no begin, end - questionable usefulness, the correct way is to use iterator in Rust, but it results in a lot of 
repetetive code, ugly macros, or both. Dropped for now, if there's a solution that doesn't involve boiler plate for every enum, then I will add it.

I would prefer this, but Rust doesn't allow it.

trait VulkanEnum {
	begin
	end
	size
}

impl<T:VulkanEnum> Iterator for T
...


Used header from 5de77cf of Vulkan-Docs

Call load_minimal() to load first 3 functions, it will allow yo to create an instance
Then call load(instance) to load the rest of the API


vulkan::initialize();
vulkan::vkCreateInstance();
vulkan::load(instance);
