// Included inside namespace explicit_relations only with ER_QUERY_FULL_CACHE.
// Caller guarantees a complete ordinary-affine right cache. Keep the original
// persistent traversal and minimum-ID reduction; omit cache-mode dispatch and
// reuse the converted target. No timer, profiling fence or extra barrier.
__global__ void search_right_fullcache(
    Curve c, Point target, U count, const DeviceRow *left, U left_size,
    unsigned long long *result, const U *offsets, unsigned shift,
    RightPartials partials) {
  __shared__ unsigned long long bound;
  cuda::atomic_ref<unsigned long long, cuda::thread_scope_device> best(result[0]);
  U visited = 0;
  const U stride = U(gridDim.x) * BLOCK;
  const Jac converted_target = c.jac(target);
  for (U start = U(blockIdx.x) * BLOCK; start < count;) {
    if (!threadIdx.x)
      bound = best.load(cuda::memory_order_relaxed);
    __syncthreads();
    if (start > bound)
      break;
    const U count_here = count - start < BLOCK ? count - start : BLOCK;
    const bool active = threadIdx.x < count_here;
    U id = active ? start + threadIdx.x : 0;
    Jac sum;
    if (active)
      sum = c.affine_sub_converted(c.jac(c.neg(partials.full[id].point())),
                                   converted_target);
    Point residual = normalize_block(c, sum);
    if (active && id <= bound &&
        relation_lookup(left, left_size, residual, offsets, shift, partials) != UINT64_MAX)
      atomicMin(result, static_cast<unsigned long long>(id));
    if (!threadIdx.x)
      visited += count_here;
    // Protect reusable normalization storage and the next shared bound.
    __syncthreads();
    if (count - start <= stride)
      break;
    start += stride;
  }
  if (!threadIdx.x)
    atomicAdd(result + 2, static_cast<unsigned long long>(visited));
}
