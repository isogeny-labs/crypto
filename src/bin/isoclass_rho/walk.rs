//! The multi-`ℓ` isogeny-class walk: breadth-first over the active primes
//! from the root, keyed by `j`; the tree edges to every measured vertex
//! realised as explicit kernels and verified on points; the root's ECDLP
//! instance (and the shifted instances drawn on other source curves)
//! carried to every measured vertex; the level of every measured vertex in
//! each `ℓ`-volcano, hence its endomorphism-ring conductor; and the
//! transport cost of each curve.  Writes `walk.json`.

use std::any::Any;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::Path;
use std::time::Instant;

use crypto_lib::cryptanalysis::isogeny_walk::field::Field as LibField;
use crypto_lib::cryptanalysis::isogeny_walk::modpoly::{self, ModPoly};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use serde_json::json;

use super::classno;
use super::field::{Curve, F2m, Fp, Fq};
use super::isogeny::{self, Kernel};
use super::search::{self, Found};

pub struct WalkParams {
    pub ells: Vec<u64>,
    /// Primes applied only at the first `hard_from` vertices (BFS order):
    /// the expensive, "hard to find" degrees.
    pub hard_ells: Vec<u64>,
    pub hard_from: usize,
    /// Largest division-polynomial degree the division route may factor;
    /// above it (binary fields) the torsion route samples kernels.
    pub max_psi_degree: usize,
    pub sample_kernels: usize,
    pub radius: u32,
    pub max_vertices: usize,
    pub per_stratum: usize,
    pub hard: usize,
    pub sources: usize,
    pub seed: u64,
    pub route: String,
    pub command: String,
}

#[derive(Clone)]
pub struct Vertex {
    pub j: u64,
    pub curve: Option<Curve>,
    pub dist: u32,
    pub parent: Option<usize>,
    pub parent_ell: u64,
    pub nbrs: Vec<(usize, u64)>,
    pub expanded: bool,
    pub edge: Option<Kernel>,
    pub push_ms: f64,
    pub levels: HashMap<u64, u32>,
    pub kernel_counts: HashMap<u64, usize>,
    /// Instance of each source at this vertex, when carried here.
    pub inst: Vec<Option<((u64, u64), (u64, u64))>>,
    pub measured: bool,
    pub level_only: bool,
}

/// How an `ℓ` behaves on the class.
#[derive(Clone, Debug)]
pub struct EllInfo {
    pub ell: u64,
    pub active: bool,
    pub reason: String,
    pub depth: u32,
    pub crater: &'static str,
    pub expected_neighbours: Option<u64>,
}

pub struct Walker<'a, F: Fq> {
    pub f: &'a F,
    pub q: u64,
    pub found: Found,
    pub ells: Vec<EllInfo>,
    pub vs: Vec<Vertex>,
    pub by_j: HashMap<u64, usize>,
    /// Kernels computed at a vertex for an `ℓ` (division route, or an
    /// Elkies fallback).
    pub kernels: HashMap<(usize, u64), Vec<Kernel>>,
    /// Neighbour `j`'s with multiplicity at a `j` for an `ℓ` (Φ route).
    pub j_nbrs: HashMap<(u64, u64), Vec<u64>>,
    lib: Option<LibField>,
    phis: HashMap<u64, ModPoly>,
    pub rng: SmallRng,
    pub max_psi_degree: usize,
    pub sample_kernels: usize,
    pub hard_ells: Vec<u64>,
    pub hard_from: usize,
    /// Primes whose kernels are sampled (torsion route), not enumerated.
    pub sampled: Vec<u64>,
    /// Extension-field data of sampled kernels: (vertex, ℓ) → the modulus
    /// and one ℓ-torsion abscissa per kernel (for dual edges).
    pub torsion_ctx: HashMap<(usize, u64), isogeny::TorsionCtx>,
    pub edges_verified: u64,
    pub route_counts: HashMap<&'static str, u64>,
    pub ell_ms: HashMap<u64, (f64, u64)>,
}

impl<'a, F: Fq> Walker<'a, F> {
    pub fn new(f: &'a F, q: u64, found: Found, requested: &[u64], seed: u64, route: &str) -> Self {
        let is_prime = (f as &dyn Any).downcast_ref::<Fp>().is_some();
        let lib = if is_prime && route != "division" {
            let p = (f as &dyn Any).downcast_ref::<Fp>().unwrap().p;
            Some(isogeny::lib_field(p))
        } else {
            None
        };
        let mut ells = Vec::new();
        for &ell in requested {
            let mut info = EllInfo {
                ell,
                active: true,
                reason: String::new(),
                depth: 0,
                crater: "",
                expected_neighbours: None,
            };
            let v = {
                let mut e = 0;
                let mut x = found.f;
                while x.is_multiple_of(ell) {
                    x /= ell;
                    e += 1;
                }
                e
            };
            info.depth = v;
            let kr = classno::kronecker(found.dk, ell);
            info.crater = classno::crater_kind(kr);
            if found.n.is_multiple_of(ell) {
                info.active = false;
                info.reason = "ℓ divides n".into();
            } else if f.char2() && ell == 2 {
                info.active = false;
                info.reason = "ℓ = 2 in characteristic 2 (inseparable)".into();
            } else if v == 0 && kr == -1 {
                info.active = false;
                info.reason = "inert and ℓ ∤ f: no rational ℓ-isogeny".into();
            } else {
                info.expected_neighbours = Some(if v > 0 {
                    ell + 1
                } else {
                    match kr {
                        1 => 2,
                        0 => 1,
                        _ => 0,
                    }
                });
            }
            ells.push(info);
        }
        Walker {
            f,
            q,
            found,
            ells,
            vs: Vec::new(),
            by_j: HashMap::new(),
            kernels: HashMap::new(),
            j_nbrs: HashMap::new(),
            lib,
            phis: HashMap::new(),
            rng: SmallRng::seed_from_u64(seed ^ 0x77a1_4e55),
            max_psi_degree: 1500,
            sample_kernels: 3,
            hard_ells: Vec::new(),
            hard_from: 2,
            sampled: Vec::new(),
            torsion_ctx: HashMap::new(),
            edges_verified: 0,
            route_counts: HashMap::new(),
            ell_ms: HashMap::new(),
        }
    }

    fn active_ells(&self) -> Vec<u64> {
        self.ells
            .iter()
            .filter(|e| e.active)
            .map(|e| e.ell)
            .collect()
    }

    fn phi(&mut self, ell: u64) -> &ModPoly {
        if !self.phis.contains_key(&ell) {
            let lf = self.lib.as_ref().unwrap();
            let t0 = Instant::now();
            let phi = modpoly::modular_polynomial(lf, ell).expect("Φ_ℓ");
            eprintln!("  Φ_{ell} built in {:.2}s", t0.elapsed().as_secs_f64());
            self.phis.insert(ell, phi);
        }
        &self.phis[&ell]
    }

    /// Neighbour `j`'s with multiplicity of the curve with invariant `j`
    /// (prime fields, Φ route) — no model needed.
    fn neighbours_by_phi(&mut self, j: u64, ell: u64) -> Vec<u64> {
        if let Some(v) = self.j_nbrs.get(&(j, ell)) {
            return v.clone();
        }
        let fp = *(self.f as &dyn Any).downcast_ref::<Fp>().unwrap();
        let seed = self.rng.gen();
        let t0 = Instant::now();
        let phi = self.phi(ell).clone();
        let lf = self.lib.as_ref().unwrap();
        let out = isogeny::phi_neighbours(&fp, lf, &phi, j, seed);
        let e = self.ell_ms.entry(ell).or_insert((0.0, 0));
        e.0 += t0.elapsed().as_secs_f64() * 1e3;
        e.1 += 1;
        self.j_nbrs.insert((j, ell), out.clone());
        out
    }

    /// All rational `ℓ`-kernels of a realised vertex (cached).
    fn kernels_at(&mut self, v: usize, ell: u64) -> Vec<Kernel> {
        if let Some(k) = self.kernels.get(&(v, ell)) {
            return k.clone();
        }
        let c = self.vs[v].curve.expect("kernels of an unrealised vertex");
        let t0 = Instant::now();
        let ks = if ell == 2 {
            let fp = (self.f as &dyn Any)
                .downcast_ref::<Fp>()
                .expect("ℓ = 2 needs F_p");
            isogeny::kernels_two(fp, &c, &mut self.rng)
        } else if self.lib.is_some() {
            let fp = *(self.f as &dyn Any).downcast_ref::<Fp>().unwrap();
            let seed = self.rng.gen();
            let phi = self.phi(ell).clone();
            let lf = self.lib.as_ref().unwrap();
            match isogeny::kernels_elkies(&fp, lf, &phi, &c, seed) {
                Ok(ks) => ks,
                Err(e) => {
                    eprintln!("  Elkies declined at v{v}, ℓ = {ell} ({e}); division route");
                    isogeny::kernels_division(self.f, &c, ell, &mut self.rng)
                }
            }
        } else if ((ell * ell - 1) / 2) as usize > self.max_psi_degree {
            // too large for the division route: sample kernels through
            // ℓ-torsion abscissae in F_{q^d}
            let q = self.q;
            let d = isogeny::eigen_orbit(self.found.t, q, ell);
            eprintln!(
                "  torsion route at v{v}, ℓ = {ell}: F_q^{d}, {} kernels wanted",
                self.sample_kernels
            );
            if !self.sampled.contains(&ell) {
                self.sampled.push(ell);
            }
            let (ks, ctx) = isogeny::kernels_torsion(
                self.f,
                &c,
                ell,
                self.found.t,
                q,
                d,
                self.sample_kernels,
                &mut self.rng,
            );
            self.torsion_ctx.insert((v, ell), ctx);
            ks
        } else {
            isogeny::kernels_division(self.f, &c, ell, &mut self.rng)
        };
        let e = self.ell_ms.entry(ell).or_insert((0.0, 0));
        e.0 += t0.elapsed().as_secs_f64() * 1e3;
        e.1 += 1;
        self.kernels.insert((v, ell), ks.clone());
        ks
    }

    /// Neighbour `j`'s with multiplicity of vertex `v` for `ℓ`, by the
    /// cheapest available route.
    fn neighbours(&mut self, v: usize, ell: u64) -> Vec<u64> {
        let j = self.vs[v].j;
        // at j = 0 or 1728 the automorphisms identify quotients of
        // non-rational subgroups, so Φ_ℓ(j, Y) over-counts: enumerate kernels
        if self.lib.is_some() && !self.is_special_j(j) {
            return self.neighbours_by_phi(j, ell);
        }
        if self.vs[v].curve.is_none() {
            let n_src = self.vs[0].inst.len();
            self.realize(v, n_src);
        }
        self.kernels_at(v, ell)
            .iter()
            .map(|k| self.f.ec_j(&k.codomain).unwrap_or(u64::MAX))
            .collect()
    }

    fn kernel_count(&mut self, v: usize, ell: u64) -> usize {
        if let Some(&c) = self.vs[v].kernel_counts.get(&ell) {
            return c;
        }
        let c = self.neighbours(v, ell).len();
        self.vs[v].kernel_counts.insert(ell, c);
        c
    }

    fn add_vertex(
        &mut self,
        j: u64,
        dist: u32,
        parent: Option<usize>,
        ell: u64,
        n_src: usize,
    ) -> usize {
        let id = self.vs.len();
        self.vs.push(Vertex {
            j,
            curve: None,
            dist,
            parent,
            parent_ell: ell,
            nbrs: Vec::new(),
            expanded: false,
            edge: None,
            push_ms: 0.0,
            levels: HashMap::new(),
            kernel_counts: HashMap::new(),
            inst: vec![None; n_src],
            measured: false,
            level_only: false,
        });
        self.by_j.insert(j, id);
        id
    }

    /// Breadth-first exploration over the active primes to `radius`.
    pub fn explore(&mut self, radius: u32, max_vertices: usize, n_src: usize) {
        let ells = self.active_ells();
        let mut queue = VecDeque::from([0usize]);
        while let Some(v) = queue.pop_front() {
            if self.vs[v].dist >= radius || self.vs[v].expanded {
                continue;
            }
            // binary fields need the model to expand: realise the tree edge first
            if self.vs[v].curve.is_none() && self.lib.is_none() {
                self.realize(v, n_src);
            }
            self.vs[v].expanded = true;
            for &ell in &ells {
                if self.hard_ells.contains(&ell) && v >= self.hard_from {
                    continue;
                }
                // a sampled prime cannot be enumerated at the floor of its
                // volcano (the one rational kernel is the ascending edge)
                if self.is_sampled_ell(ell) {
                    let depth = self.ells.iter().find(|e| e.ell == ell).unwrap().depth;
                    if depth > 0 && self.level_of(v, ell) >= depth {
                        continue;
                    }
                }
                let nb = self.neighbours(v, ell);
                self.vs[v].kernel_counts.insert(ell, nb.len());
                for j2 in nb {
                    if j2 == u64::MAX || self.is_special_j(j2) && j2 != self.vs[0].j {
                        continue;
                    }
                    let w = match self.by_j.get(&j2) {
                        Some(&w) => w,
                        None => {
                            if self.vs.len() >= max_vertices {
                                continue;
                            }
                            let w = self.add_vertex(j2, self.vs[v].dist + 1, Some(v), ell, n_src);
                            queue.push_back(w);
                            w
                        }
                    };
                    if w != v {
                        self.vs[v].nbrs.push((w, ell));
                        if !self.vs[w].nbrs.contains(&(v, ell)) {
                            self.vs[w].nbrs.push((v, ell));
                        }
                    }
                }
            }
        }
    }

    fn is_sampled_ell(&self, ell: u64) -> bool {
        self.lib.is_none() && ((ell * ell - 1) / 2) as usize > self.max_psi_degree
    }

    fn is_special_j(&self, j: u64) -> bool {
        if self.f.char2() {
            false
        } else {
            let p = (self.f as &dyn Any).downcast_ref::<Fp>().unwrap().p;
            j == 0 || j == 1728 % p
        }
    }

    /// Realise the tree edge into `v` (and recursively its ancestors):
    /// find the kernel from the parent whose codomain has `j(v)`, verify it
    /// on the carried points, carry every present instance across.
    pub fn realize(&mut self, v: usize, n_src: usize) {
        if self.vs[v].curve.is_some() {
            return;
        }
        let par = self.vs[v].parent.expect("root is realised at start");
        if self.vs[par].curve.is_none() {
            self.realize(par, n_src);
        }
        let ell = self.vs[v].parent_ell;
        let pc = self.vs[par].curve.unwrap();
        let jv = self.vs[v].j;
        let mut ks = self.kernels_at(par, ell);
        let pos = ks
            .iter()
            .position(|k| self.f.ec_j(&k.codomain) == Some(jv))
            .unwrap_or_else(|| panic!("internal: no kernel from v{par} to j = {jv} at ℓ = {ell}"));
        let k = ks.swap_remove(pos);
        let n = self.found.n;
        let t0 = Instant::now();
        let mut carried = Vec::new();
        for s in 0..n_src {
            if let Some((p, q)) = self.vs[par].inst[s] {
                isogeny::verify_map(self.f, &pc, &k, &[p, q], n)
                    .unwrap_or_else(|e| panic!("edge v{par} → v{v} (ℓ = {ell}, {}): {e}", k.route));
                let ip = k.map(self.f, &pc, Some(p)).unwrap();
                let iq = k.map(self.f, &pc, Some(q)).unwrap();
                carried.push((s, (ip, iq)));
            }
        }
        if carried.is_empty() {
            // verify on a fresh point of order n anyway
            let cof = self.found.cofactor;
            let r = loop {
                if let Some(pt) =
                    self.f
                        .ec_mul(&pc, Some(self.f.ec_random(&pc, &mut self.rng)), cof)
                {
                    break pt;
                }
            };
            let r2 = self.f.ec_dbl(&pc, Some(r)).unwrap();
            isogeny::verify_map(self.f, &pc, &k, &[r, r2], n)
                .unwrap_or_else(|e| panic!("edge v{par} → v{v} (ℓ = {ell}): {e}"));
        }
        self.edges_verified += 1;
        *self.route_counts.entry(k.route).or_insert(0) += 1;
        self.vs[v].push_ms = t0.elapsed().as_secs_f64() * 1e3;
        self.vs[v].curve = Some(k.codomain);
        for (s, inst) in carried {
            self.vs[v].inst[s] = Some(inst);
        }
        self.vs[v].edge = Some(k);
    }

    /// Level of `v` in the `ℓ`-volcano (0 = crater, depth = floor) by
    /// walking non-backtracking paths from up to three neighbours to the
    /// floor (vertices with one rational `ℓ`-kernel).
    pub fn level_of(&mut self, v: usize, ell: u64) -> u32 {
        if let Some(&l) = self.vs[v].levels.get(&ell) {
            return l;
        }
        let info = self.ells.iter().find(|e| e.ell == ell).unwrap().clone();
        let depth = info.depth;
        let lvl = if depth == 0 {
            0
        } else if self.sampled.contains(&ell)
            || ((ell * ell - 1) / 2) as usize > self.max_psi_degree && self.lib.is_none()
        {
            // sampled kernels cannot be counted: a non-loop ℓ-edge from a
            // vertex above the floor is taken as descending (exact when the
            // crater has one vertex, as on a Koblitz curve)
            match self.vs[v].parent {
                None => 0,
                Some(par) => {
                    let pl = self.level_of(par, ell);
                    if self.vs[v].parent_ell == ell && self.vs[v].j != self.vs[par].j && pl < depth
                    {
                        pl + 1
                    } else {
                        pl
                    }
                }
            }
        } else if self.kernel_count(v, ell) == 1 {
            depth
        } else {
            let jv = self.vs[v].j;
            let nb = self.neighbours_at_j(jv, ell);
            let mut distinct: Vec<u64> = Vec::new();
            for j in nb {
                if j != u64::MAX && !distinct.contains(&j) && !self.is_special_j(j) {
                    distinct.push(j);
                }
            }
            let mut best: Option<u32> = None;
            for &w in distinct.iter().take(3) {
                let mut prev = self.vs[v].j;
                let mut cur = w;
                let mut steps = 0u32;
                loop {
                    if self.count_at_j(cur, ell) == 1 {
                        best = Some(best.map_or(steps, |b| b.min(steps)));
                        break;
                    }
                    if steps >= depth {
                        break;
                    }
                    let nxt = self.neighbours_at_j(cur, ell);
                    let Some(next) = nxt
                        .iter()
                        .copied()
                        .find(|&j| j != prev && j != u64::MAX && !self.is_special_j(j))
                    else {
                        break;
                    };
                    prev = cur;
                    cur = next;
                    steps += 1;
                }
            }
            match best {
                Some(s) => depth - 1 - s,
                None => {
                    eprintln!("isoclass-rho: warning: level of v{v} at ℓ = {ell} not determined; recorded as crater");
                    0
                }
            }
        };
        self.vs[v].levels.insert(ell, lvl);
        lvl
    }

    /// Neighbour `j`'s of the curve with invariant `j`, creating a
    /// level-only vertex when the model is needed (binary fields).
    fn neighbours_at_j(&mut self, j: u64, ell: u64) -> Vec<u64> {
        if self.lib.is_some() && !self.is_special_j(j) {
            return self.neighbours_by_phi(j, ell);
        }
        let v = self.vertex_for_j(j);
        let nb = self.neighbours(v, ell);
        let n_src = self.vs[0].inst.len();
        for &j2 in &nb {
            if j2 != u64::MAX && !self.is_special_j(j2) && !self.by_j.contains_key(&j2) {
                let w = self.add_vertex(j2, self.vs[v].dist + 1, Some(v), ell, n_src);
                self.vs[w].level_only = true;
            }
            if let Some(&w) = self.by_j.get(&j2) {
                if w != v && !self.vs[v].nbrs.contains(&(w, ell)) {
                    self.vs[v].nbrs.push((w, ell));
                    self.vs[w].nbrs.push((v, ell));
                }
            }
        }
        nb
    }

    fn count_at_j(&mut self, j: u64, ell: u64) -> usize {
        if self.lib.is_some() && !self.is_special_j(j) {
            return self.neighbours_by_phi(j, ell).len();
        }
        let v = self.vertex_for_j(j);
        self.kernel_count(v, ell)
    }

    /// The vertex with invariant `j`; it must already exist (it was found
    /// as a neighbour of an explored vertex) — realise it for its model.
    fn vertex_for_j(&mut self, j: u64) -> usize {
        let v = *self.by_j.get(&j).expect("level walk reached an unknown j");
        if self.vs[v].curve.is_none() {
            let n_src = self.vs[0].inst.len();
            self.realize(v, n_src);
        }
        if !self.vs[v].expanded && self.vs[v].level_only {
            // nothing: neighbours are computed on demand
        }
        v
    }

    /// Make sure the neighbours of `v` for `ℓ` exist as vertices (so a
    /// level walk can continue through them).
    fn ensure_neighbours(&mut self, v: usize, ell: u64, n_src: usize) {
        if self.vs[v].curve.is_none() {
            self.realize(v, n_src);
        }
        let nb = self.neighbours(v, ell);
        for j2 in nb {
            if j2 == u64::MAX || self.is_special_j(j2) {
                continue;
            }
            if !self.by_j.contains_key(&j2) {
                let w = self.add_vertex(j2, self.vs[v].dist + 1, Some(v), ell, n_src);
                self.vs[w].level_only = true;
            }
            let w = self.by_j[&j2];
            if w != v && !self.vs[v].nbrs.contains(&(w, ell)) {
                self.vs[v].nbrs.push((w, ell));
                self.vs[w].nbrs.push((v, ell));
            }
        }
    }

    /// Levels of `v` in every volcano prime, extending the graph as needed.
    pub fn levels(&mut self, v: usize, n_src: usize) -> HashMap<u64, u32> {
        let vol: Vec<(u64, u32)> = self
            .ells
            .iter()
            .filter(|e| e.active && e.depth > 0)
            .map(|e| (e.ell, e.depth))
            .collect();
        for &(ell, depth) in &vol {
            let sampled = ((ell * ell - 1) / 2) as usize > self.max_psi_degree;
            if self.lib.is_none() && !sampled {
                // binary: the level walk needs models up to `depth` steps out
                let mut frontier = vec![v];
                for _ in 0..depth {
                    let mut next = Vec::new();
                    for u in frontier {
                        self.ensure_neighbours(u, ell, n_src);
                        for &(w, l) in &self.vs[u].nbrs.clone() {
                            if l == ell {
                                next.push(w);
                            }
                        }
                    }
                    frontier = next;
                }
            }
            self.level_of(v, ell);
        }
        self.vs[v].levels.clone()
    }

    /// Conductor of `End(E_v)` from its levels: `∏ ℓ^{level_ℓ}` over the
    /// volcano primes (relative to the walked primes; other primes of `f`
    /// are reported separately).
    pub fn conductor(&self, v: usize) -> u64 {
        self.vs[v]
            .levels
            .iter()
            .map(|(&ell, &lvl)| ell.pow(lvl))
            .product()
    }

    /// The dual direction of the tree edge into `v`: a kernel at `v` whose
    /// codomain has the parent's `j`, with the isomorphism onto the
    /// parent's model.  Returns the map on points.
    pub fn up_map(&mut self, v: usize) -> Box<dyn Fn(&F, (u64, u64)) -> (u64, u64)> {
        let par = self.vs[v].parent.expect("root has no parent");
        let ell = self.vs[v].parent_ell;
        let jp = self.vs[par].j;
        let cv = self.vs[v].curve.unwrap();
        let pc = self.vs[par].curve.unwrap();
        let k = if self.is_sampled_ell(ell) {
            // the dual of a sampled edge: push another ℓ-torsion abscissa of
            // the parent through the edge; its image generates ker φ̂
            let edge = self.vs[v].edge.clone().unwrap();
            let ctx = self
                .torsion_ctx
                .get(&(par, ell))
                .expect("torsion context of a sampled edge")
                .clone();
            isogeny::dual_torsion(self.f, &pc, &edge, &ctx, self.found.t, self.q).unwrap_or_else(
                |e| panic!("dual of the sampled edge v{par} → v{v} (ℓ = {ell}): {e}"),
            )
        } else {
            let ks = self.kernels_at(v, ell);
            ks.into_iter()
                .find(|k| self.f.ec_j(&k.codomain) == Some(jp))
                .unwrap_or_else(|| panic!("internal: no dual edge from v{v} to its parent"))
        };
        let cod = k.codomain;
        let f = *self.f;
        if f.char2() {
            assert_eq!(cod, pc, "binary dual codomain is not the parent's model");
            Box::new(move |f: &F, p| k.map(f, &cv, Some(p)).expect("order-n point maps to O"))
        } else {
            let fp = *(self.f as &dyn Any).downcast_ref::<Fp>().unwrap();
            // (u⁴a, u⁶b) = (a_par, b_par): u² from a and b, a sixth root when
            // a = 0 (j = 0), a fourth root when b = 0 (j = 1728)
            let u = if pc.a == 0 {
                fp.sixth_root(fp.fmul(pc.b, fp.finv(cod.b)))
                    .expect("dual codomain is not isomorphic to the j = 0 parent model")
            } else if pc.b == 0 {
                let r = fp.fmul(pc.a, fp.finv(cod.a));
                let u2 = fp
                    .sqrt(r)
                    .expect("dual codomain is not isomorphic to the j = 1728 parent model");
                fp.sqrt(u2)
                    .or_else(|| fp.sqrt(fp.fneg(u2)))
                    .expect("dual codomain is the twist of the parent model")
            } else {
                let u2 = fp.fmul(fp.fmul(pc.b, cod.a), fp.finv(fp.fmul(cod.b, pc.a)));
                fp.sqrt(u2)
                    .expect("dual codomain is the twist of the parent model")
            };
            let (u2, u3) = (fp.fmul(u, u), fp.fmul(fp.fmul(u, u), u));
            assert_eq!(fp.fmul(fp.fmul(u2, u2), cod.a), pc.a);
            assert_eq!(fp.fmul(fp.fmul(u3, u3), cod.b), pc.b);
            Box::new(move |f: &F, p| {
                let q = k.map(f, &cv, Some(p)).expect("order-n point maps to O");
                (fp.fmul(u2, q.0), fp.fmul(u3, q.1))
            })
        }
    }
}

/// Run the whole walk and write `walk.json`.
pub fn run_walk<F: Fq>(
    f: &F,
    q: u64,
    found: Found,
    params: &WalkParams,
    field_json: serde_json::Value,
    out: &Path,
) {
    let t_all = Instant::now();
    let mut rng = SmallRng::seed_from_u64(params.seed);
    let n = found.n;
    let cof = found.cofactor;
    let e0 = found.curve;
    // the instance
    let p0 = loop {
        if let Some(pt) = f.ec_mul(&e0, Some(f.ec_random(&e0, &mut rng)), cof) {
            break pt;
        }
    };
    assert!(f.ec_mul(&e0, Some(p0), n).is_none());
    let k_secret = rng.gen_range(1..n);
    let q0 = f.ec_mul(&e0, Some(p0), k_secret).unwrap();
    let n_src = 1 + params.sources;
    let class = classno::class_summary(found.t, q, found.d_abs, true);
    let mut all_ells: Vec<u64> = params.ells.clone();
    for &l in &params.hard_ells {
        if !all_ells.contains(&l) {
            all_ells.push(l);
        }
    }
    let mut w = Walker::new(f, q, found.clone(), &all_ells, params.seed, &params.route);
    w.max_psi_degree = params.max_psi_degree;
    w.sample_kernels = params.sample_kernels;
    w.hard_ells = params.hard_ells.clone();
    w.hard_from = params.hard_from;
    let root = w.add_vertex(f.ec_j(&e0).expect("root j"), 0, None, 0, n_src);
    w.vs[root].curve = Some(e0);
    w.vs[root].inst[0] = Some((p0, q0));
    for e in &w.ells {
        eprintln!(
            "ℓ = {}: {} depth {} crater {} expected neighbours {:?} {}",
            e.ell,
            if e.active { "active" } else { "inactive" },
            e.depth,
            e.crater,
            e.expected_neighbours,
            e.reason
        );
    }
    let t_explore = Instant::now();
    w.explore(params.radius, params.max_vertices, n_src);
    let explored = w.vs.len();
    eprintln!(
        "explored {} vertices in {:.1}s",
        explored,
        t_explore.elapsed().as_secs_f64()
    );

    // levels and conductor of every explored vertex within the radius
    let t_lv = Instant::now();
    let ids: Vec<usize> = (0..explored).collect();
    for &v in &ids {
        w.levels(v, n_src);
    }
    eprintln!(
        "levels of {} vertices in {:.1}s ({} vertices now)",
        explored,
        t_lv.elapsed().as_secs_f64(),
        w.vs.len()
    );

    // measured set: per conductor stratum, spread over the BFS order; the
    // root; the `hard` vertices reached through the largest ℓ
    let mut by_stratum: HashMap<u64, Vec<usize>> = HashMap::new();
    for &v in &ids {
        by_stratum.entry(w.conductor(v)).or_default().push(v);
    }
    let mut measured: Vec<usize> = vec![root];
    let mut strata: Vec<u64> = by_stratum.keys().copied().collect();
    strata.sort();
    for s in &strata {
        let list = &by_stratum[s];
        let take = params.per_stratum.min(list.len());
        if take == 0 {
            continue;
        }
        let step = (list.len() as f64 / take as f64).max(1.0);
        for i in 0..take {
            let v = list[(i as f64 * step) as usize];
            if !measured.contains(&v) {
                measured.push(v);
            }
        }
    }
    let max_ell = w.active_ells().into_iter().max().unwrap_or(0);
    let hard: Vec<usize> = ids
        .iter()
        .copied()
        .filter(|&v| {
            v != root
                && (w.vs[v].parent_ell == max_ell || params.hard_ells.contains(&w.vs[v].parent_ell))
        })
        .take(params.hard)
        .collect();
    for v in hard {
        if !measured.contains(&v) {
            measured.push(v);
        }
    }
    measured.sort();
    for &v in &measured {
        w.realize(v, n_src);
        w.vs[v].measured = true;
    }
    eprintln!(
        "realised the tree to {} measured vertices; {} edges verified",
        measured.len(),
        w.edges_verified
    );

    // shifted instances: sources among the measured vertices, carried up to
    // the root and down to every measured vertex
    let mut sources =
        vec![json!({"index": 0, "vertex": root, "P": [p0.0, p0.1], "Q": [q0.0, q0.1]})];
    let mut src_vertices: Vec<usize> = measured.iter().copied().filter(|&v| v != root).collect();
    // spread the sources over the measured list
    let mut chosen = Vec::new();
    if params.sources > 0 && !src_vertices.is_empty() {
        let step = (src_vertices.len() as f64 / params.sources as f64).max(1.0);
        for i in 0..params.sources.min(src_vertices.len()) {
            chosen.push(src_vertices[(i as f64 * step) as usize]);
        }
    }
    src_vertices = chosen;
    for (si, &sv) in src_vertices.iter().enumerate() {
        let s = si + 1;
        let cs = w.vs[sv].curve.unwrap();
        let ps = loop {
            if let Some(pt) = f.ec_mul(&cs, Some(f.ec_random(&cs, &mut rng)), cof) {
                break pt;
            }
        };
        let qs = f.ec_mul(&cs, Some(ps), k_secret).unwrap();
        w.vs[sv].inst[s] = Some((ps, qs));
        // up to the root
        let mut cur = sv;
        let (mut p, mut qq) = (ps, qs);
        while let Some(par) = w.vs[cur].parent {
            let up = w.up_map(cur);
            p = up(f, p);
            qq = up(f, qq);
            let pc = w.vs[par].curve.unwrap();
            assert!(f.ec_on(&pc, Some(p)) && f.ec_on(&pc, Some(qq)));
            assert!(f.ec_mul(&pc, Some(p), n).is_none() && f.ec_mul(&pc, Some(qq), n).is_none());
            assert_eq!(
                f.ec_mul(&pc, Some(p), k_secret),
                Some(qq),
                "shifted instance lost its logarithm on the way up"
            );
            w.vs[par].inst[s] = Some((p, qq));
            cur = par;
        }
        // down the tree to every measured vertex (and the vertices on the way)
        let order: Vec<usize> = {
            let mut o: Vec<usize> = (0..w.vs.len())
                .filter(|&v| w.vs[v].curve.is_some())
                .collect();
            o.sort_by_key(|&v| w.vs[v].dist);
            o
        };
        for v in order {
            if w.vs[v].inst[s].is_some() {
                continue;
            }
            let Some(par) = w.vs[v].parent else { continue };
            let Some((pp, pq)) = w.vs[par].inst[s] else {
                continue;
            };
            let k = w.vs[v].edge.clone().unwrap();
            let pc = w.vs[par].curve.unwrap();
            let ip = k.map(f, &pc, Some(pp)).unwrap();
            let iq = k.map(f, &pc, Some(pq)).unwrap();
            let cv = w.vs[v].curve.unwrap();
            assert_eq!(f.ec_mul(&cv, Some(ip), k_secret), Some(iq));
            w.vs[v].inst[s] = Some((ip, iq));
        }
        sources.push(json!({"index": s, "vertex": sv, "P": [ps.0, ps.1], "Q": [qs.0, qs.1]}));
    }

    // independent order check on every measured curve
    let mut order_checked = 0;
    for &v in &measured {
        let c = w.vs[v].curve.unwrap();
        if search::order_holds(f, &c, found.order, &mut rng) {
            order_checked += 1;
        } else {
            panic!("measured vertex v{v} does not have order #E");
        }
    }

    // records
    let vertices: Vec<serde_json::Value> = w
        .vs
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let path: Vec<u64> = {
                let mut p = Vec::new();
                let mut cur = i;
                while let Some(par) = w.vs[cur].parent {
                    p.push(w.vs[cur].parent_ell);
                    cur = par;
                }
                p.reverse();
                p
            };
            let log2deg: f64 = path.iter().map(|&l| (l as f64).log2()).sum();
            let transport_ms: f64 = {
                let mut t = 0.0;
                let mut cur = i;
                while let Some(par) = w.vs[cur].parent {
                    if let Some(e) = &w.vs[cur].edge {
                        t += e.find_ms + w.vs[cur].push_ms;
                    }
                    cur = par;
                }
                t
            };
            let levels: serde_json::Map<String, serde_json::Value> =
                v.levels.iter().map(|(l, lv)| (l.to_string(), json!(lv))).collect();
            let counts: serde_json::Map<String, serde_json::Value> =
                v.kernel_counts.iter().map(|(l, c)| (l.to_string(), json!(c))).collect();
            let cond = w.conductor(i);
            json!({
                "id": i, "j": v.j, "a": v.curve.map(|c| c.a), "b": v.curve.map(|c| c.b),
                "dist": v.dist, "parent": v.parent, "parent_ell": if v.parent.is_some() { Some(v.parent_ell) } else { None },
                "path": path, "log2_degree": log2deg, "levels": levels, "conductor": cond,
                "stratum_class_number": if v.levels.is_empty() { None } else { Some(classno::class_number_order(class["h_fundamental"].as_u64().unwrap(), found.dk, cond)) },
                "kernels": counts, "nbrs": v.nbrs.iter().map(|&(w, l)| json!([w, l])).collect::<Vec<_>>(),
                "measured": v.measured, "level_only": v.level_only, "explored": v.expanded,
                "edge": v.edge.as_ref().map(|e| json!({"ell": e.ell, "route": e.route, "field_degree": e.field_degree,
                                                         "find_ms": e.find_ms, "push_ms": v.push_ms, "h_degree": e.h.len().saturating_sub(1),
                                                         "h": e.h})),
                "transport": {"edges": path.len(), "log2_degree": log2deg, "ms": transport_ms},
                "inst": v.inst.iter().map(|i| i.map(|(p, q)| json!([p.0, p.1, q.0, q.1]))).collect::<Vec<_>>(),
            })
        })
        .collect();
    let ells_json: Vec<serde_json::Value> = w
        .ells
        .iter()
        .map(|e| {
            let (ms, cnt) = w.ell_ms.get(&e.ell).copied().unwrap_or((0.0, 0));
            json!({"ell": e.ell, "active": e.active, "reason": e.reason, "depth": e.depth, "crater": e.crater,
                   "expected_neighbours": e.expected_neighbours, "kernel_calls": cnt, "mean_find_ms": if cnt > 0 { ms / cnt as f64 } else { 0.0 },
                   "sampled": w.sampled.contains(&e.ell), "eigen_orbit_degree": isogeny::eigen_orbit(found.t, q, e.ell)})
        })
        .collect();
    let per_ell_edges: HashMap<String, u64> =
        w.vs.iter()
            .filter(|v| v.edge.is_some())
            .fold(HashMap::new(), |mut m, v| {
                *m.entry(v.parent_ell.to_string()).or_insert(0) += 1;
                m
            });
    let out_json = json!({
        "schema": "isoclass-walk/v1",
        "command": params.command,
        "params": {"ells": params.ells, "hard_ells": params.hard_ells, "hard_from": params.hard_from, "radius": params.radius, "max_vertices": params.max_vertices,
                   "max_psi_degree": params.max_psi_degree, "sample_kernels": params.sample_kernels, "sampled_ells": w.sampled,
                   "per_stratum": params.per_stratum, "hard": params.hard, "sources": params.sources,
                   "seed": params.seed, "route": params.route},
        "field": field_json,
        "root": {"a": e0.a, "b": e0.b, "j": w.vs[root].j, "order": found.order, "n": n, "cofactor": cof,
                 "trace": found.t, "k": k_secret, "P": [p0.0, p0.1], "Q": [q0.0, q0.1],
                 "search_curves_tried": found.tried},
        "class": class,
        "ells": ells_json,
        "walk": {"radius": params.radius, "vertices": w.vs.len(), "vertices_explored": explored,
                 "vertices_measured": measured.len(), "edges_realized": w.vs.iter().filter(|v| v.edge.is_some()).count(),
                 "edges_verified": w.edges_verified, "routes": w.route_counts, "per_ell_edges": per_ell_edges,
                 "order_checked": order_checked, "time_s": t_all.elapsed().as_secs_f64()},
        "measured": measured,
        "sources": sources,
        "vertices": vertices,
    });
    fs::create_dir_all(out).unwrap();
    fs::write(
        out.join("walk.json"),
        serde_json::to_string_pretty(&out_json).unwrap(),
    )
    .unwrap();
    eprintln!("wrote {}", out.join("walk.json").display());
    let _ = F2m::irreducible;
}
