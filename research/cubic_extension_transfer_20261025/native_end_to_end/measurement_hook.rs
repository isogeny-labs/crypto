// Optional resident observation hook. Appended before source_adapter.rs.
mod cubic_measure_hook {
    use std::ffi::{c_char, c_int, c_void, CString};
    #[link(name = "dl")]
    extern "C" {
        fn dlopen(path: *const c_char, flags: c_int) -> *mut c_void;
        fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
        fn dlclose(handle: *mut c_void) -> c_int;
    }
    type Begin = unsafe extern "C" fn(c_int) -> *mut c_void;
    type Finish = unsafe extern "C" fn(*mut c_void) -> c_int;
    pub struct Guard {
        library: *mut c_void,
        state: *mut c_void,
        finish: Option<Finish>,
    }
    impl Guard {
        pub fn start() -> Self {
            let library = std::env::var_os("CUBIC_MEASURE_LIBRARY");
            let fd = std::env::var_os("CUBIC_MEASURE_FD");
            if library.is_none() && fd.is_none() {
                return Self {
                    library: std::ptr::null_mut(),
                    state: std::ptr::null_mut(),
                    finish: None,
                };
            }
            let library = CString::new(
                library
                    .expect("measurement library missing")
                    .into_encoded_bytes(),
            )
            .expect("library path NUL");
            let fd: c_int = fd
                .expect("measurement descriptor missing")
                .to_str()
                .expect("descriptor UTF8")
                .parse()
                .expect("descriptor integer");
            assert!(fd >= 3, "measurement descriptor reserved");
            unsafe {
                let library = dlopen(library.as_ptr(), 2); // RTLD_NOW | RTLD_LOCAL
                assert!(!library.is_null(), "measurement library unavailable");
                let begin = dlsym(library, b"cubic_measure_begin\0".as_ptr().cast());
                let finish = dlsym(library, b"cubic_measure_finish\0".as_ptr().cast());
                if begin.is_null() || finish.is_null() {
                    dlclose(library);
                    panic!("measurement ABI unavailable");
                }
                let begin: Begin = std::mem::transmute(begin);
                let finish: Finish = std::mem::transmute(finish);
                let state = begin(fd);
                if state.is_null() {
                    dlclose(library);
                    panic!("measurement initialization failed");
                }
                Self {
                    library,
                    state,
                    finish: Some(finish),
                }
            }
        }
    }
    impl Drop for Guard {
        fn drop(&mut self) {
            if let Some(finish) = self.finish {
                unsafe {
                    let status = finish(self.state);
                    dlclose(self.library);
                    if status != 0 {
                        std::process::exit(126);
                    }
                }
            }
        }
    }
}
