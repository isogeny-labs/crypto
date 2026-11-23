// Included inside namespace explicit_relations, only in diagnostic builds.
// These block-cycle intervals include barriers and scheduler time. They are
// neither summed GPU wall time nor uninstrumented instruction costs.
struct QueryProfile {
  unsigned long long cycles[6]{};
  unsigned long long tiles = 0, rows = 0;
  QueryProfile &operator+=(const QueryProfile &other) {
    for (unsigned i = 0; i < 6; ++i)
      cycles[i] += other.cycles[i];
    tiles += other.tiles;
    rows += other.rows;
    return *this;
  }
};
__device__ __forceinline__ void profile_fence(U &v) {
  asm volatile("" : "+l"(v) : : "memory");
}
__device__ __forceinline__ void profile_fence(Jac &p) {
  profile_fence(p.x); profile_fence(p.y); profile_fence(p.z);
}
__device__ __forceinline__ void profile_fence(Point &p) {
  profile_fence(p.x); profile_fence(p.y);
  unsigned inf = p.inf;
  asm volatile("" : "+r"(inf) : : "memory");
  p.inf = inf;
}
__device__ __forceinline__ unsigned long long profile_stamp() {
  __syncthreads();
  unsigned long long t;
  asm volatile("mov.u64 %0, %%clock64;" : "=l"(t) : : "memory");
  return t;
}
// Same traversal and deterministic minimum as search_right_persistent.
// Full-cache only. All threads reach every stamp, including inactive tails.
__global__ void search_right_profile(
    Curve c, Point target, U count, const DeviceRow *left, U left_size,
    unsigned long long *result, const U *offsets, unsigned shift,
    RightPartials partials, QueryProfile *profiles) {
  __shared__ unsigned long long bound;
  cuda::atomic_ref<unsigned long long, cuda::thread_scope_device> best(result[0]);
  QueryProfile stats{};
  const U stride = U(gridDim.x) * BLOCK;
  for (U start = U(blockIdx.x) * BLOCK; start < count;) {
    if (!threadIdx.x)
      bound = best.load(cuda::memory_order_relaxed);
    __syncthreads();
    if (start > bound)
      break;
    U count_here = count - start < BLOCK ? count - start : BLOCK;
    bool active = threadIdx.x < count_here;
    U id = active ? start + threadIdx.x : 0;
    auto t0 = profile_stamp();
    Jac p{}, q{};
    if (active) {
      p = c.jac(c.neg(partials.full[id].point()));
      q = c.jac(target);
    }
    profile_fence(p); profile_fence(q);
    auto t1 = profile_stamp();
    profile_fence(p); profile_fence(q);
    p = c.affine_sub_converted(p, q);
    profile_fence(p);
    auto t2 = profile_stamp();
    profile_fence(p);
    U iz = inverse_block(c, p);
    profile_fence(iz);
    auto t3 = profile_stamp();
    profile_fence(iz); profile_fence(p);
    Point residual = c.affine(p, iz);
    profile_fence(residual);
    auto t4 = profile_stamp();
    profile_fence(residual);
    U found = UINT64_MAX;
    if (active && id <= bound)
      found = relation_lookup(left, left_size, residual, offsets, shift, partials);
    profile_fence(found);
    auto t5 = profile_stamp();
    profile_fence(found);
    auto t6 = profile_stamp(); // Empty stamp/barrier control, never subtracted.
    if (active && id <= bound && found != UINT64_MAX)
      atomicMin(result, static_cast<unsigned long long>(id));
    if (!threadIdx.x) {
      stats.cycles[0] += t1 - t0;
      stats.cycles[1] += t2 - t1;
      stats.cycles[2] += t3 - t2;
      stats.cycles[3] += t4 - t3;
      stats.cycles[4] += t5 - t4;
      stats.cycles[5] += t6 - t5;
      ++stats.tiles;
      stats.rows += count_here;
    }
    __syncthreads();
    if (count - start <= stride)
      break;
    start += stride;
  }
  if (!threadIdx.x) {
    profiles[blockIdx.x] = stats;
    atomicAdd(result + 2, stats.rows);
  }
}
