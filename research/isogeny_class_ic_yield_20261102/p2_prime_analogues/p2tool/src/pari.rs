//! Minimal libpari binding through dlopen (no Python). Used only by the
//! class builder, on a machine that has libpari; the run itself needs none.
use std::ffi::{c_char, c_int, c_ulong, c_void, CStr, CString};

extern "C" {
    fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlerror() -> *const c_char;
    fn free(p: *mut c_void);
}

pub struct Pari {
    read_str: unsafe extern "C" fn(*const c_char) -> *mut c_void,
    to_str: unsafe extern "C" fn(*mut c_void) -> *mut c_char,
    pub version: String,
}

impl Pari {
    pub fn open(path: &str) -> Result<Pari, String> {
        unsafe {
            let c = CString::new(path).unwrap();
            let h = dlopen(c.as_ptr(), 2 | 0x100);
            if h.is_null() {
                return Err(format!("dlopen {path}: {:?}", CStr::from_ptr(dlerror())));
            }
            let sym = |n: &str| -> Result<*mut c_void, String> {
                let c = CString::new(n).unwrap();
                let s = dlsym(h, c.as_ptr());
                if s.is_null() {
                    Err(format!("dlsym {n}"))
                } else {
                    Ok(s)
                }
            };
            let init: unsafe extern "C" fn(usize, c_ulong) = std::mem::transmute(sym("pari_init")?);
            init(1 << 30, 2_000_000);
            let mut p = Pari {
                read_str: std::mem::transmute(sym("gp_read_str")?),
                to_str: std::mem::transmute(sym("GENtostr")?),
                version: String::new(),
            };
            p.version = p.eval("version()").unwrap_or_default();
            Ok(p)
        }
    }

    /// Evaluate a GP expression and return its printed value.
    pub fn eval(&self, expr: &str) -> Result<String, String> {
        unsafe {
            let c = CString::new(expr).unwrap();
            let g = (self.read_str)(c.as_ptr());
            let s = (self.to_str)(g);
            let out = CStr::from_ptr(s).to_string_lossy().into_owned();
            free(s as *mut c_void);
            Ok(out)
        }
    }
}
