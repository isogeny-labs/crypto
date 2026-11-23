#[derive(Clone, Copy, Default, Debug)]
pub struct State {
    pub u: f64,
    pub f: f64,
    pub v: f64,
}
fn derivative(s: State, l: f64, a: f64, b: f64, c: f64) -> State {
    let du = (b + 2.0 * c * s.u) * (1.0 - s.u) / l;
    State {
        u: du,
        f: a + b * s.u + c * s.u * s.u,
        v: du + 2.0 * c / l * (1.0 - s.u) * s.v,
    }
}
fn add(s: State, d: State, h: f64) -> State {
    State {
        u: s.u + h * d.u,
        f: s.f + h * d.f,
        v: s.v + h * d.v,
    }
}
pub fn integrate(s: &mut State, from: u64, to: u64, l: f64, a: f64, b: f64, c: f64) {
    for _ in 0..(to - from) * 4 {
        let h = 0.25;
        let p = derivative(*s, l, a, b, c);
        let q = derivative(add(*s, p, h / 2.0), l, a, b, c);
        let r = derivative(add(*s, q, h / 2.0), l, a, b, c);
        let t = derivative(add(*s, r, h), l, a, b, c);
        s.u += h * (p.u + 2.0 * q.u + 2.0 * r.u + t.u) / 6.0;
        s.f += h * (p.f + 2.0 * q.f + 2.0 * r.f + t.f) / 6.0;
        s.v += h * (p.v + 2.0 * q.v + 2.0 * r.v + t.v) / 6.0;
    }
}
fn close(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= 1e-7 * a.abs().max(b.abs()).max(1e-9),
        "{a} != {b}"
    );
}
pub fn gate() {
    let mut checks = 0;
    for l in [10.0, 1000.0] {
        for a in [0.0, 0.01] {
            for b in [0.001, 0.05] {
                for c in [0.0, 0.02] {
                    for time in [1, 10, 100] {
                        let mut s = State::default();
                        integrate(&mut s, 0, time, l, a, b, c);
                        let t = time as f64;
                        let (u, f, v) = if c == 0.0 {
                            let u = -(-b * t / l).exp_m1();
                            (u, (a + b) * t - l * u, u)
                        } else {
                            let w = b / (b + 2.0 * c) * ((b + 2.0 * c) * t / l).exp_m1();
                            let u = w / (1.0 + w);
                            let m = 1.0 + 2.0 * c / b * u;
                            (
                                u,
                                (a - b * b / (4.0 * c)) * t
                                    + l / 2.0 * ((1.0 + b / (2.0 * c)) * w.ln_1p() - u),
                                b / (2.0 * c) * m * m.ln(),
                            )
                        };
                        close(s.u, u);
                        close(s.f, f);
                        close(s.v, v);
                        checks += 1;
                    }
                }
            }
        }
    }
    // No FP seed: PP alone cannot grow the deliberately rooted simplified tree.
    let mut s = State::default();
    integrate(&mut s, 0, 100, 10.0, 0.01, 0.0, 0.02);
    close(s.u, 0.0);
    close(s.v, 0.0);
    close(s.f, 1.0);
    println!("ODE_GATE cases={checks} plus_zero_seed=true tolerance=1e-7");
}
