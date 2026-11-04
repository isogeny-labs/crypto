//! Minimal libpari binding through dlopen (no Python), the same mechanism as
//! `p2_prime_analogues/p2tool/src/pari.rs`.  Used only by the three construction subcommands
//! (`build-neighbours`, `build-tower`, `make-targets`) on a machine that has libpari; the
//! measurement (`run`, `analyze`) needs none.
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
    /// `stack_bytes`: initial PARI stack (virtual; committed lazily).
    pub fn open(path: &str, stack_bytes: usize) -> Result<Pari, String> {
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
            init(stack_bytes, 2_000_000);
            let mut p = Pari {
                read_str: std::mem::transmute(sym("gp_read_str")?),
                to_str: std::mem::transmute(sym("GENtostr")?),
                version: String::new(),
            };
            // "[2, 17, 2]" -> "(2, 17, 2)" (the tuple spelling the Python builder recorded)
            let v = p.eval("version()")?;
            p.version = format!("({})", v.trim_start_matches('[').trim_end_matches(']'));
            Ok(p)
        }
    }

    /// Evaluate a GP expression and return its printed value.
    pub fn eval(&self, expr: &str) -> Result<String, String> {
        unsafe {
            let c = CString::new(expr).map_err(|e| e.to_string())?;
            let g = (self.read_str)(c.as_ptr());
            let s = (self.to_str)(g);
            let out = CStr::from_ptr(s).to_string_lossy().into_owned();
            free(s as *mut c_void);
            Ok(out)
        }
    }

    /// Evaluate a GP expression that returns a string and strip the quotes.
    pub fn eval_str(&self, expr: &str) -> String {
        let s = self.eval(expr).unwrap();
        s.strip_prefix('"').and_then(|t| t.strip_suffix('"')).unwrap_or(&s).to_string()
    }

    /// Load GP source: one definition per chunk (a line starting in column 0 opens a new one;
    /// indented lines continue it); whole-line `\\` comments and blank lines are dropped.  A GP
    /// function body extends to the end of the evaluated string, so each definition is evaluated
    /// on its own.
    pub fn load(&self, src: &str) {
        let mut chunks: Vec<String> = Vec::new();
        for l in src.lines() {
            if l.trim().is_empty() || l.trim_start().starts_with("\\\\") {
                continue;
            }
            if l.starts_with(char::is_whitespace) {
                chunks.last_mut().expect("continuation line without definition").push_str(&format!(" {}", l.trim()));
            } else {
                chunks.push(l.trim().to_string());
            }
        }
        for c in chunks {
            self.eval(&c).unwrap();
        }
    }
}

pub fn find_libpari() -> String {
    for k in ["P1_LIBPARI", "P2_LIBPARI"] {
        if let Ok(v) = std::env::var(k) {
            return v;
        }
    }
    panic!("set --libpari or P1_LIBPARI to the libpari .so (e.g. the cypari2.libs/libpari-*.so.2.17.2 of a venv)");
}
