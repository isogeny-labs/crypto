//! Private owned FLINT values. Only allocation/ABI marshaling lives in C.
use num_bigint::BigUint;
use std::ffi::{c_char, c_int, c_long, c_void, CStr, CString};
use std::path::Path;
use std::ptr::NonNull;
use std::rc::Rc;

type Unary = unsafe extern "C" fn(*mut c_void, *const c_void, *const c_void);
type Binary = unsafe extern "C" fn(*mut c_void, *const c_void, *const c_void, *const c_void);
type SizedUnary = unsafe extern "C" fn(*mut c_void, *const c_void, c_long, *const c_void);
type MulMod =
    unsafe extern "C" fn(*mut c_void, *const c_void, *const c_void, *const c_void, *const c_void);
type MulPreinv = unsafe extern "C" fn(
    *mut c_void,
    *const c_void,
    *const c_void,
    *const c_void,
    *const c_void,
    *const c_void,
);

#[cfg(target_os = "linux")]
#[link(name = "dl")]
unsafe extern "C" {
    fn dlopen(path: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
    fn dlerror() -> *const c_char;
}

struct Library(NonNull<c_void>);
impl Library {
    fn open(path: &Path) -> Result<Self, String> {
        #[cfg(target_os = "linux")]
        {
            use std::os::unix::ffi::OsStrExt;
            let path = CString::new(path.as_os_str().as_bytes()).map_err(|e| e.to_string())?;
            // SAFETY: NUL-terminated path; RTLD_NOW. Handle remains owned until
            // every Context/Poly/function pointer using it has been dropped.
            NonNull::new(unsafe { dlopen(path.as_ptr(), 2) })
                .map(Self)
                .ok_or_else(Self::error)
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = path;
            Err("native FLINT evaluator currently requires Linux".into())
        }
    }
    fn error() -> String {
        #[cfg(target_os = "linux")]
        {
            // SAFETY: libdl owns this NUL-terminated diagnostic, if present.
            let p = unsafe { dlerror() };
            if !p.is_null() {
                return unsafe { CStr::from_ptr(p) }.to_string_lossy().into_owned();
            }
        }
        "native library operation failed".into()
    }
    unsafe fn symbol<T: Copy>(&self, name: &CStr) -> Result<T, String> {
        #[cfg(target_os = "linux")]
        {
            // SAFETY: caller supplies the exact registered C function signature.
            let p = unsafe { dlsym(self.0.as_ptr(), name.as_ptr()) };
            if p.is_null() {
                return Err(Self::error());
            }
            if std::mem::size_of::<T>() != std::mem::size_of_val(&p) {
                return Err("function-pointer ABI width".into());
            }
            Ok(unsafe { std::mem::transmute_copy(&p) })
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = name;
            Err("native library loading requires Linux".into())
        }
    }
}
impl Drop for Library {
    fn drop(&mut self) {
        #[cfg(target_os = "linux")]
        // SAFETY: owned handle; no remaining Context or Poly outlives this Api.
        unsafe {
            dlclose(self.0.as_ptr());
        }
    }
}

pub(super) struct Api {
    _library: Library,
    prime: unsafe extern "C" fn(*const c_char) -> c_int,
    context_new: unsafe extern "C" fn(*const c_char) -> *mut c_void,
    context_free: unsafe extern "C" fn(*mut c_void),
    poly_new: unsafe extern "C" fn(*const c_void) -> *mut c_void,
    poly_free: unsafe extern "C" fn(*const c_void, *mut c_void),
    length: unsafe extern "C" fn(*const c_void, *const c_void) -> c_long,
    set_decimal: unsafe extern "C" fn(*const c_void, *mut c_void, c_long, *const c_char) -> c_int,
    get_decimal: unsafe extern "C" fn(*const c_void, *const c_void, c_long) -> *mut c_char,
    scale: unsafe extern "C" fn(*const c_void, *mut c_void, *const c_void, *const c_char) -> c_int,
    roots: unsafe extern "C" fn(*const c_void, *const *const c_char, c_long) -> *mut c_void,
    dot: unsafe extern "C" fn(*const c_void, *const c_void, *const c_void) -> *mut c_char,
    free_string: unsafe extern "C" fn(*mut c_char),
    set: Unary,
    add: Binary,
    sub: Binary,
    mul: Binary,
    gcd: Binary,
    reverse: SizedUnary,
    inv_series: SizedUnary,
    derivative: Unary,
    mullow: unsafe extern "C" fn(*mut c_void, *const c_void, *const c_void, c_long, *const c_void),
    invmod: unsafe extern "C" fn(*mut c_void, *const c_void, *const c_void, *const c_void) -> c_int,
    mulmod: MulMod,
    mulmod_preinv: MulPreinv,
}
impl Api {
    pub(super) fn load(path: &Path, expected: &str) -> Result<Rc<Self>, String> {
        let (sha, _) = crypto_lib::store::hash_file(path).map_err(|e| e.to_string())?;
        if sha != expected {
            return Err("FLINT bridge SHA256 mismatch".into());
        }
        let library = Library::open(path)?;
        // SAFETY: exact C ABI version and function signatures are defined by
        // native/rust_flint_bridge.c and the dynamically linked FLINT headers.
        unsafe {
            let abi: unsafe extern "C" fn() -> u64 = library.symbol(c"crypto_flint_abi")?;
            if abi() != 1 {
                return Err("unsupported FLINT bridge ABI".into());
            }
            macro_rules! sym {
                ($s:literal) => {
                    library.symbol($s)?
                };
            }
            let api = Self {
                prime: sym!(c"crypto_flint_prime"),
                context_new: sym!(c"crypto_flint_ctx_new"),
                context_free: sym!(c"crypto_flint_ctx_free"),
                poly_new: sym!(c"crypto_flint_poly_new"),
                poly_free: sym!(c"crypto_flint_poly_free"),
                length: sym!(c"crypto_flint_poly_length"),
                set_decimal: sym!(c"crypto_flint_poly_set_decimal"),
                get_decimal: sym!(c"crypto_flint_poly_get_decimal"),
                scale: sym!(c"crypto_flint_poly_scale"),
                roots: sym!(c"crypto_flint_poly_roots"),
                dot: sym!(c"crypto_flint_poly_dot"),
                free_string: sym!(c"crypto_flint_string_free"),
                set: sym!(c"fmpz_mod_poly_set"),
                add: sym!(c"fmpz_mod_poly_add"),
                sub: sym!(c"fmpz_mod_poly_sub"),
                mul: sym!(c"fmpz_mod_poly_mul"),
                gcd: sym!(c"fmpz_mod_poly_gcd"),
                reverse: sym!(c"fmpz_mod_poly_reverse"),
                inv_series: sym!(c"fmpz_mod_poly_inv_series"),
                derivative: sym!(c"fmpz_mod_poly_derivative"),
                mullow: sym!(c"fmpz_mod_poly_mullow"),
                invmod: sym!(c"fmpz_mod_poly_invmod"),
                mulmod: sym!(c"fmpz_mod_poly_mulmod"),
                mulmod_preinv: sym!(c"fmpz_mod_poly_mulmod_preinv"),
                _library: library,
            };
            Ok(Rc::new(api))
        }
    }
    pub(super) fn is_prime(&self, value: &BigUint) -> bool {
        let s = CString::new(value.to_string()).expect("decimal has no NUL");
        unsafe { (self.prime)(s.as_ptr()) == 1 }
    }
    fn integer(&self, text: *mut c_char) -> Result<BigUint, String> {
        let p = NonNull::new(text).ok_or("native decimal allocation")?;
        // SAFETY: bridge returns an owned NUL-terminated FLINT allocation.
        let result = unsafe { CStr::from_ptr(p.as_ptr()) }.to_bytes();
        let parsed = BigUint::parse_bytes(result, 10).ok_or("native noninteger decimal");
        unsafe {
            (self.free_string)(p.as_ptr());
        }
        parsed.map_err(Into::into)
    }
}

pub(super) struct Context {
    raw: NonNull<c_void>,
    api: Rc<Api>,
}
impl Context {
    pub(super) fn new(api: Rc<Api>, p: &BigUint) -> Result<Rc<Self>, String> {
        if !api.is_prime(p) {
            return Err("nonprime field modulus".into());
        }
        let text = CString::new(p.to_string()).expect("decimal");
        let raw = NonNull::new(unsafe { (api.context_new)(text.as_ptr()) })
            .ok_or("native context allocation")?;
        Ok(Rc::new(Self { raw, api }))
    }
}
impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            (self.api.context_free)(self.raw.as_ptr());
        }
    }
}

pub(super) struct Poly {
    raw: NonNull<c_void>,
    ctx: Rc<Context>,
}
impl Drop for Poly {
    fn drop(&mut self) {
        unsafe {
            (self.ctx.api.poly_free)(self.ctx.raw.as_ptr(), self.raw.as_ptr());
        }
    }
}
impl Clone for Poly {
    fn clone(&self) -> Self {
        let out = Self::zero(self.ctx.clone());
        unsafe {
            (self.ctx.api.set)(out.raw.as_ptr(), self.raw.as_ptr(), self.ctx.raw.as_ptr());
        }
        out
    }
}
impl Poly {
    pub(super) fn zero(ctx: Rc<Context>) -> Self {
        let raw = NonNull::new(unsafe { (ctx.api.poly_new)(ctx.raw.as_ptr()) })
            .expect("FLINT allocation");
        Self { raw, ctx }
    }
    pub(super) fn constant(ctx: Rc<Context>, n: &BigUint) -> Self {
        let mut p = Self::zero(ctx);
        p.set_coefficient(0, n);
        p
    }
    pub(super) fn set_coefficient(&mut self, i: usize, n: &BigUint) {
        let text = CString::new(n.to_string()).expect("decimal");
        assert_eq!(
            unsafe {
                (self.ctx.api.set_decimal)(
                    self.ctx.raw.as_ptr(),
                    self.raw.as_ptr(),
                    i.try_into().expect("polynomial index"),
                    text.as_ptr(),
                )
            },
            1
        );
    }
    pub(super) fn coefficient(&self, i: usize) -> Result<BigUint, String> {
        self.ctx.api.integer(unsafe {
            (self.ctx.api.get_decimal)(
                self.ctx.raw.as_ptr(),
                self.raw.as_ptr(),
                i.try_into().map_err(|_| "coefficient index")?,
            )
        })
    }
    pub(super) fn len(&self) -> usize {
        unsafe { (self.ctx.api.length)(self.ctx.raw.as_ptr(), self.raw.as_ptr()) }
            .try_into()
            .expect("nonnegative native length")
    }
    fn compatible(&self, other: &Self) {
        assert!(
            Rc::ptr_eq(&self.ctx, &other.ctx),
            "cross-context polynomial"
        );
    }
    fn binary(&self, b: &Self, f: Binary) -> Self {
        self.compatible(b);
        let out = Self::zero(self.ctx.clone());
        unsafe {
            f(
                out.raw.as_ptr(),
                self.raw.as_ptr(),
                b.raw.as_ptr(),
                self.ctx.raw.as_ptr(),
            );
        }
        out
    }
    pub(super) fn add(&self, b: &Self) -> Self {
        self.binary(b, self.ctx.api.add)
    }
    pub(super) fn sub(&self, b: &Self) -> Self {
        self.binary(b, self.ctx.api.sub)
    }
    pub(super) fn mul(&self, b: &Self) -> Self {
        self.binary(b, self.ctx.api.mul)
    }
    pub(super) fn gcd(&self, b: &Self) -> Self {
        self.binary(b, self.ctx.api.gcd)
    }
    pub(super) fn scaled(&self, n: &BigUint) -> Self {
        let out = Self::zero(self.ctx.clone());
        let text = CString::new(n.to_string()).expect("decimal");
        assert_eq!(
            unsafe {
                (self.ctx.api.scale)(
                    self.ctx.raw.as_ptr(),
                    out.raw.as_ptr(),
                    self.raw.as_ptr(),
                    text.as_ptr(),
                )
            },
            1
        );
        out
    }
    fn sized(&self, n: usize, f: SizedUnary) -> Self {
        let out = Self::zero(self.ctx.clone());
        unsafe {
            f(
                out.raw.as_ptr(),
                self.raw.as_ptr(),
                n.try_into().expect("polynomial extent"),
                self.ctx.raw.as_ptr(),
            );
        }
        out
    }
    pub(super) fn reverse(&self, n: usize) -> Self {
        self.sized(n, self.ctx.api.reverse)
    }
    pub(super) fn inverse_series(&self, n: usize) -> Self {
        self.sized(n, self.ctx.api.inv_series)
    }
    pub(super) fn derivative(&self) -> Self {
        let out = Self::zero(self.ctx.clone());
        unsafe {
            (self.ctx.api.derivative)(out.raw.as_ptr(), self.raw.as_ptr(), self.ctx.raw.as_ptr());
        }
        out
    }
    pub(super) fn mullow(&self, b: &Self, n: usize) -> Self {
        self.compatible(b);
        let out = Self::zero(self.ctx.clone());
        unsafe {
            (self.ctx.api.mullow)(
                out.raw.as_ptr(),
                self.raw.as_ptr(),
                b.raw.as_ptr(),
                n.try_into().expect("polynomial extent"),
                self.ctx.raw.as_ptr(),
            );
        }
        out
    }
    pub(super) fn inverse_mod(&self, h: &Self) -> Result<Self, String> {
        self.compatible(h);
        let out = Self::zero(self.ctx.clone());
        if unsafe {
            (self.ctx.api.invmod)(
                out.raw.as_ptr(),
                self.raw.as_ptr(),
                h.raw.as_ptr(),
                self.ctx.raw.as_ptr(),
            )
        } != 1
        {
            return Err("native polynomial inverse failed".into());
        }
        Ok(out)
    }
    pub(super) fn mulmod(&self, b: &Self, h: &Self) -> Self {
        self.compatible(b);
        self.compatible(h);
        let out = Self::zero(self.ctx.clone());
        unsafe {
            (self.ctx.api.mulmod)(
                out.raw.as_ptr(),
                self.raw.as_ptr(),
                b.raw.as_ptr(),
                h.raw.as_ptr(),
                self.ctx.raw.as_ptr(),
            );
        }
        out
    }
    pub(super) fn mulmod_preinv(&self, b: &Self, h: &Self, hi: &Self) -> Self {
        self.compatible(b);
        self.compatible(h);
        self.compatible(hi);
        let out = Self::zero(self.ctx.clone());
        unsafe {
            (self.ctx.api.mulmod_preinv)(
                out.raw.as_ptr(),
                self.raw.as_ptr(),
                b.raw.as_ptr(),
                h.raw.as_ptr(),
                hi.raw.as_ptr(),
                self.ctx.raw.as_ptr(),
            );
        }
        out
    }
    pub(super) fn dot(&self, b: &Self) -> Result<BigUint, String> {
        self.compatible(b);
        self.ctx.api.integer(unsafe {
            (self.ctx.api.dot)(self.ctx.raw.as_ptr(), self.raw.as_ptr(), b.raw.as_ptr())
        })
    }
    pub(super) fn roots(ctx: Rc<Context>, values: &[BigUint]) -> Result<Self, String> {
        let strings: Vec<CString> = values
            .iter()
            .map(|v| CString::new(v.to_string()).expect("decimal"))
            .collect();
        let pointers: Vec<_> = strings.iter().map(|s| s.as_ptr()).collect();
        let raw = NonNull::new(unsafe {
            (ctx.api.roots)(
                ctx.raw.as_ptr(),
                pointers.as_ptr(),
                values.len().try_into().map_err(|_| "root vector extent")?,
            )
        })
        .ok_or("native root vector")?;
        Ok(Self { raw, ctx })
    }
}
