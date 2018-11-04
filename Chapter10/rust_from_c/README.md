
in our lib.rs file, we'll create a function with an extern block, specifying the ABI we want to expose
to this function which is "C". It corresponds to the cdecl calling convention for functions. We also need to add a #[no_mangle] attribute so that Rust won't add random sequence of characters to our method for distinguishing between multiple implementations.