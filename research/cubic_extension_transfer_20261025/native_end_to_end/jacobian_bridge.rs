// Appended to the unchanged pinned quartic backend by build_jacobian.sh.
// No source logarithm or synthetic target is accepted by this interface.
pub struct JacobianBridge {
    ctx: Ctx,
    fc: [[u64; 5]; 5],
    arena: Vec<Div>,
}
fn bridge_context(p: u64, fc: [[u64; 5]; 5], seed: u64) -> JacobianBridge {
    assert!(p > 13 && is_prime(p));
    let f = Fq(p);
    assert!(
        is_smooth(f, &fc),
        "independent smoothness certificate failed"
    );
    let mut rng = Rng::new(seed);
    let mut origin = None;
    for _ in 0..10000 {
        let o = random_point(f, &fc, &mut rng);
        let d = derivs(f, &fc, o.0, o.1);
        let dx = d[1];
        let dy = f.neg(d[0]);
        let g2 = f.add(
            f.add(f.mul(d[2], f.mul(dx, dx)), f.mul(d[3], f.mul(dx, dy))),
            f.mul(d[4], f.mul(dy, dy)),
        );
        if d[1] != 0 && g2 != 0 {
            origin = Some(o);
            break;
        }
    }
    let mut ctx = Ctx::new(
        Curve::new(f, fc, origin.expect("ordinary affine origin budget")),
        seed ^ 0x5678,
    );
    for _ in 0..1000 {
        if ctx.pool.len() == 8 {
            break;
        }
        let d = random_class(&mut ctx, &fc);
        if let Ok(n) = ctx.neg(&d) {
            ctx.pool.push(d);
            ctx.pool.push(n);
        } else {
            ctx.fails += 1;
        }
    }
    assert_eq!(ctx.pool.len(), 8, "helper pool budget");
    let id = ctx.id();
    JacobianBridge {
        ctx,
        fc,
        arena: vec![id],
    }
}
fn bridge_store(b: &mut JacobianBridge, d: Option<Div>) -> u64 {
    if let Some(d) = d {
        b.arena.push(d);
        b.arena.len() as u64
    } else {
        0
    }
}
fn bridge_load(b: &JacobianBridge, handle: u64) -> Div {
    assert!(
        handle > 0 && handle as usize <= b.arena.len(),
        "invalid divisor handle"
    );
    b.arena[handle as usize - 1].clone()
}
// The reference group law is partial: a binary chain can hit an unsupported
// effective-divisor chart even when its final result is representable. Retry
// the same scalar with (k+t)D-tD, without reducing k+t modulo an assumed order.
// Return the first representable result; never retry a mismatching result.
fn scalar_with_splits(ctx: &mut Ctx, k: u64, d: &Div, shift_bound: u64) -> (Option<Div>, u32) {
    if let Some(value) = ctx.smul(k, d) {
        return (Some(value), 0);
    }
    for attempt in 1..=32 {
        let shift = 1 + ctx.rng.below(shift_bound.max(2) - 1);
        let Some(sum) = k.checked_add(shift) else {
            continue;
        };
        let Some(left) = ctx.smul(sum, d) else {
            continue;
        };
        let Some(right) = ctx.smul(shift, d) else {
            continue;
        };
        if let Some(value) = ctx.sub_r(&left, &right) {
            return (Some(value), attempt);
        }
    }
    (None, 32)
}
#[no_mangle]
pub unsafe extern "C" fn cubic_jacobian_new(
    p: u64,
    coefficients: *const u64,
    seed: u64,
) -> *mut JacobianBridge {
    std::panic::catch_unwind(|| {
        assert!(!coefficients.is_null());
        let input = std::slice::from_raw_parts(coefficients, 25);
        let mut fc = [[0; 5]; 5];
        for i in 0..5 {
            for j in 0..5 {
                fc[i][j] = input[5 * i + j];
                assert!(fc[i][j] < p && (i + j <= 4 || fc[i][j] == 0));
            }
        }
        Box::into_raw(Box::new(bridge_context(p, fc, seed)))
    })
    .unwrap_or(std::ptr::null_mut())
}
#[no_mangle]
pub unsafe extern "C" fn cubic_jacobian_free(b: *mut JacobianBridge) {
    if !b.is_null() {
        drop(Box::from_raw(b));
    }
}
#[no_mangle]
pub unsafe extern "C" fn cubic_jacobian_divisor(
    b: *mut JacobianBridge,
    u: *const u64,
    v: *const u64,
) -> u64 {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        assert!(!b.is_null() && !u.is_null() && !v.is_null());
        let b = &mut *b;
        let f = b.ctx.c.fq;
        let a = std::slice::from_raw_parts(u, 4).to_vec();
        let mut y = std::slice::from_raw_parts(v, 3).to_vec();
        trim(&mut y);
        assert_eq!(a[3], 1);
        assert!(a.iter().chain(y.iter()).all(|&c| c < f.0));
        let mut check = vec![];
        for j in (0..5).rev() {
            check = padd(
                f,
                &pmulmod(f, &check, &y, &a),
                &pmod(f, &b.ctx.c.fpoly[j], &a),
            );
        }
        assert!(
            pmod(f, &check, &a).is_empty(),
            "input divisor is off the curve"
        );
        bridge_store(b, Some(Div { a, b: y }))
    }))
    .unwrap_or(0)
}
#[no_mangle]
pub unsafe extern "C" fn cubic_jacobian_op(
    b: *mut JacobianBridge,
    operation: u32,
    left: u64,
    right: u64,
) -> u64 {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        assert!(!b.is_null()); let b = &mut *b; let a = bridge_load(b, left);
        let d = match operation {
            0 => { let c = bridge_load(b, right); b.ctx.add_r(&a, &c) },
            1 => { let c = bridge_load(b, right); b.ctx.sub_r(&a, &c) },
            2 => b.ctx.neg_r(&a),
            3 => {
                let (result, attempts) = scalar_with_splits(&mut b.ctx, right, &a, right);
                if attempts > 0 { eprintln!("NATIVE_SCALAR_SPLITS k={right} attempts={attempts} success={}",result.is_some()); }
                result
            },
            _ => panic!("unknown group operation"),
        };
        if d.is_none() {
            let retry = match operation {
                0 => {let c=bridge_load(b,right); b.ctx.add(&a,&c).err()},
                1 => {let c=bridge_load(b,right); b.ctx.sub(&a,&c).err()},
                2 => b.ctx.neg(&a).err(),
                _ => None,
            };
            eprintln!("NATIVE_GROUP_FAILURE operation={operation} left={left} right={right} direct_error={retry:?} origin={:?} left_u={:?} left_v={:?} fc={:?}",b.ctx.c.o,a.a,a.b,b.fc);
            if operation<=1 {let c=bridge_load(b,right);eprintln!("NATIVE_GROUP_RIGHT u={:?} v={:?}",c.a,c.b);}
        }
        bridge_store(b, d)
    })).unwrap_or(0)
}
#[no_mangle]
pub unsafe extern "C" fn cubic_jacobian_equal(
    b: *const JacobianBridge,
    left: u64,
    right: u64,
) -> i32 {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        assert!(!b.is_null());
        let b = &*b;
        (bridge_load(b, left) == bridge_load(b, right)) as i32
    }))
    .unwrap_or(-1)
}
#[no_mangle]
pub unsafe extern "C" fn cubic_jacobian_export(
    b: *const JacobianBridge,
    handle: u64,
    u: *mut u64,
    v: *mut u64,
    origin: *mut u64,
) -> i32 {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        assert!(!b.is_null() && !u.is_null() && !v.is_null() && !origin.is_null());
        let b = &*b;
        let d = bridge_load(b, handle);
        assert_eq!(d.a.len(), 4);
        assert!(d.b.len() <= 3);
        std::ptr::copy_nonoverlapping(d.a.as_ptr(), u, 4);
        for i in 0..3 {
            *v.add(i) = d.b.get(i).copied().unwrap_or(0);
        }
        *origin = b.ctx.c.o.0;
        *origin.add(1) = b.ctx.c.o.1;
        1
    }))
    .unwrap_or(0)
}
fn upper_weil_bound_below_twice_r(p: u64, r: u64) -> bool {
    let n = 256u128 * p as u128;
    let mut lo = 0u128;
    let mut hi = 1u128;
    while hi * hi < n {
        hi *= 2;
    }
    while lo + 1 < hi {
        let m = (lo + hi) / 2;
        if m * m >= n {
            hi = m;
        } else {
            lo = m;
        }
    }
    (hi + 16).pow(6) < 2 * r as u128 * 16u128.pow(6)
}
#[no_mangle]
pub unsafe extern "C" fn cubic_jacobian_certify_order(b: *mut JacobianBridge, r: u64) -> u64 {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        assert!(!b.is_null());
        let b = &mut *b;
        assert!(
            is_prime(r) && upper_weil_bound_below_twice_r(b.ctx.c.fq.0, r),
            "prime/Weil certificate gate"
        );
        for _ in 0..32 {
            let d = random_class(&mut b.ctx, &b.fc);
            if d == b.ctx.id() {
                continue;
            }
            if b.ctx.smul(r, &d) == Some(b.ctx.id()) && b.ctx.smul(r + 1, &d) == Some(d.clone()) {
                return bridge_store(b, Some(d));
            }
        }
        0
    }))
    .unwrap_or(0)
}
#[no_mangle]
pub unsafe extern "C" fn cubic_jacobian_solve(
    b: *mut JacobianBridge,
    p_handle: u64,
    q_handle: u64,
    r: u64,
    arm: u32,
    seed: u64,
    answer: *mut u64,
    report: *mut u8,
    capacity: usize,
) -> i32 {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        assert!(!b.is_null() && !answer.is_null() && !report.is_null() && capacity > 1);
        let before=snap();let b = &mut *b;
        assert!(is_prime(r) && upper_weil_bound_below_twice_r(b.ctx.c.fq.0,r));
        let p = bridge_load(b,p_handle); let qq = bridge_load(b,q_handle);
        let (rp,precheck_p)=scalar_with_splits(&mut b.ctx,r,&p,r);
        let (rq,precheck_q)=scalar_with_splits(&mut b.ctx,r,&qq,r);
        assert!(p != b.ctx.id() && rp == Some(b.ctx.id()) && rq == Some(b.ctx.id()));
        let f = b.ctx.c.fq; let fc = b.fc; let origin = b.ctx.c.o;
        // Instance owns its context. A valid empty replacement prevents an
        // unwinding solver from leaving an uninitialized/double-freed bridge.
        let placeholder = Ctx::new(Curve::new(f,fc,origin),seed ^ 0x98);
        let ctx = std::mem::replace(&mut b.ctx,placeholder);
        let n1 = rational_point_count(f,&fc,&mut Rng::new(seed ^ 0x91));
        let mut instance = Instance {ctx,q:f.0,n_group:r,ell:r,cof:1,p,qq,k:0,n1,curve_tries:0};
        let mut result = match arm {0 => run_ic(&mut instance,300,seed),1 => run_rho(&mut instance,false,seed),2 => run_rho(&mut instance,true,seed),_ => panic!("unknown target solver")};
        let direct_verified=result.verified;let mut verification_splits=0;let mut failure="none";
        if let Some(k)=result.scalar {
            *answer=k;
            if !result.verified {
                let (check,attempts)=scalar_with_splits(&mut instance.ctx,k,&instance.p,r);
                verification_splits=attempts;
                if let Some(actual)=check {
                    result.verified=actual==instance.qq;
                    if !result.verified { failure="candidate_mismatch"; }
                } else {failure="arithmetic_chart_budget";}
            }
        }
        b.ctx = instance.ctx;
        let value = result.scalar;
        let count=diff(snap(),before);
        let text = format!("{{\"found\":{},\"verified\":{},\"direct_verification_passed\":{direct_verified},\"verification_split_attempts\":{verification_splits},\"subgroup_precheck_split_attempts\":{},\"verification_failure\":\"{failure}\",\"steps\":{},\"adds\":{},\"restarts\":{},\"M\":{},\"I\":{},\"NAT\":{},\"LA\":{},\"count_scope\":\"entire target-solver C ABI call including checks and verification retries\",\"detail\":\"{}\",\"known_log_provided\":false}}",result.found,result.verified,precheck_p+precheck_q,result.steps,result.adds,result.restarts,count.m,count.i,count.nat,count.la,result.extra);
        assert!(text.len() < capacity);std::ptr::copy_nonoverlapping(text.as_ptr(),report,text.len());*report.add(text.len())=0;
        if result.found && result.verified { if let Some(k)=value { *answer=k;return 1; } } 0
    })).unwrap_or(-1)
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(
        args.len(),
        2,
        "jacobian_order PRIME_R_AND_25_COEFFICIENTS_FILE"
    );
    let data = std::fs::read_to_string(&args[1]).unwrap();
    let v: Vec<u64> = data
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    assert_eq!(v.len(), 27);
    let (p, r) = (v[0], v[1]);
    unsafe {
        let b = cubic_jacobian_new(p, v[2..].as_ptr(), 20261029 + p);
        assert!(!b.is_null(), "group context failed");
        let witness = cubic_jacobian_certify_order(b, r);
        assert!(witness > 1, "prime-order witness budget exhausted");
        let state = &*b;
        let d = bridge_load(state, witness);
        println!("{{\"status\":\"passed\",\"p\":{p},\"jacobian_order\":{r},\"witness_u\":{:?},\"witness_v\":{:?},\"nonzero_witness\":true,\"annihilator_verified\":true,\"r_plus_one_control\":true,\"weil_upper_below_twice_r\":true,\"benchmark\":false}}",d.a,d.b);
        cubic_jacobian_free(b);
    }
}
