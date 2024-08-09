# HTTP Reverse Proxy Sys Plugin

## this is a example plugin for [reverse_proxy_sys](https://github.com/thedtvn/reverse_proxy_sys)

## main file is `src/lib.rs`

### usage

compile with `cargo build --release`

when code is compiled, it will generate `target\release\reverse_proxy_sys_plugin_example.(dll|so|dylib)`

dll: windows

so: linux

dylib: macos

## IMPORTANT

DO NOT UNWARP OR PANIC IN PLUGIN OR IT WILL CRASH THE PROGRAM
SO HANDLE ERRORS PROPERLY
EXAMPLE: 
BAD WAY: 
```rs
let _ = test_err().unwrap();
```
GOOD WAY: 
```rs
let test = test_err();
if test.is_err() {
   return
}
let _ = test.unwrap();
```

#### Project by [thedtvn](https://github.com/thedtvn)