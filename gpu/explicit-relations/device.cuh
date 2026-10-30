#pragma once
#include "host.hpp"
#include "negative-filter.hpp"
#include <cstdio>
#include <cuda_runtime.h>
#ifdef ER_QUERY_PERSISTENT
#include <cuda/atomic>
#endif
#ifdef ER_CUB_SORT
#include <cub/device/device_merge_sort.cuh>
#endif
#ifdef ER_NORMALIZE_CUB_REFERENCE
#include <cub/block/block_scan.cuh>
#endif
#include <thrust/device_vector.h>
#include <thrust/remove.h>
#include <thrust/sort.h>

namespace explicit_relations {
inline void cuda_check(cudaError_t e) {
  if (e != cudaSuccess)
    throw std::runtime_error(cudaGetErrorString(e));
}
constexpr unsigned BLOCK = 256;
static_assert(BLOCK >= 2 && (BLOCK & (BLOCK - 1)) == 0,
              "normalization requires a power-of-two block");
#ifdef ER_NORMALIZE_CUB_REFERENCE
struct FieldProduct {
  Curve c;
  __device__ U operator()(U a, U b) const { return c.mul(a, b); }
};
#endif
// One inversion per block, including partially active blocks and infinity.
// Infinity and inactive lanes contribute 1. No lane may skip a barrier.
__device__ Point normalize_block(Curve c, Jac p) {
#ifdef ER_NORMALIZE_SCAN_REFERENCE
  // Retained only for matched baseline builds and differential tests.
  __shared__ U prefix[BLOCK], suffix[BLOCK], inverse;
  unsigned t = threadIdx.x;
  U z = p.z ? p.z : c.f.one;
  prefix[t] = z;
  suffix[t] = z;
  __syncthreads();
  for (unsigned d = 1; d < BLOCK; d *= 2) {
    U a = t >= d ? prefix[t - d] : c.f.one;
    U b = t + d < BLOCK ? suffix[t + d] : c.f.one;
    __syncthreads();
    prefix[t] = c.mul(prefix[t], a);
    suffix[t] = c.mul(suffix[t], b);
    __syncthreads();
  }
  if (!t)
    inverse = fp64_inv(c.f, prefix[BLOCK - 1]);
  __syncthreads();
  U iz = c.mul(inverse, c.mul(t ? prefix[t - 1] : c.f.one,
                              t + 1 < BLOCK ? suffix[t + 1] : c.f.one));
  return c.affine(p, iz);
#elif defined(ER_NORMALIZE_CUB_REFERENCE)
  // External work-efficient scan reference, with the same inversion and
  // affine conversion as both native implementations.
  using Scan = cub::BlockScan<U, BLOCK>;
  __shared__ typename Scan::TempStorage storage;
  __shared__ U values[BLOCK], reverse[BLOCK], inverse;
  unsigned t = threadIdx.x;
  U z = p.z ? p.z : c.f.one, prefix, suffix, total;
  values[t] = z;
  __syncthreads();
  Scan(storage).ExclusiveScan(z, prefix, c.f.one, FieldProduct{c}, total);
  __syncthreads();
  if (!t)
    inverse = fp64_inv(c.f, total);
  Scan(storage).ExclusiveScan(values[BLOCK - 1 - t], suffix, c.f.one,
                              FieldProduct{c});
  reverse[BLOCK - 1 - t] = suffix;
  __syncthreads();
  return c.affine(p, c.mul(inverse, c.mul(prefix, reverse[t])));
#else
  // Product tree: BLOCK-1 multiplies up, 2*(BLOCK-1) down. The previous
  // two inclusive scans used 2*BLOCK*log2(BLOCK)+2*BLOCK multiplies.
  // Each parent inverse times its sibling product is the child's inverse.
  __shared__ U tree[2 * BLOCK];
  unsigned t = threadIdx.x;
  tree[BLOCK + t] = p.z ? p.z : c.f.one;
  __syncthreads();
  for (unsigned width = BLOCK / 2; width; width /= 2) {
    if (t < width) {
      unsigned node = width + t;
      tree[node] = c.mul(tree[2 * node], tree[2 * node + 1]);
    }
    __syncthreads();
  }
  if (!t)
    tree[1] = fp64_inv(c.f, tree[1]);
  __syncthreads();
  for (unsigned width = 1; width < BLOCK; width *= 2) {
    if (t < width) {
      unsigned node = width + t;
      U inverse = tree[node], left = tree[2 * node], right = tree[2 * node + 1];
      tree[2 * node] = c.mul(inverse, right);
      tree[2 * node + 1] = c.mul(inverse, left);
    }
    __syncthreads();
  }
  return c.affine(p, tree[BLOCK + t]);
#endif
}
#if defined(ER_DENSE_ROWS)
using DeviceRow = DenseRow;
using DeviceRowLess = DenseRowLess;
RF_HD DeviceRow device_row(Point point, U id) { return {point_key(point), id}; }
#elif defined(ER_COMPACT_ROWS)
using DeviceRow = CompactRow;
using DeviceRowLess = CompactRowLess;
RF_HD DeviceRow device_row(Point point, U id) { return {point_key(point), id}; }
#else
using DeviceRow = Row;
using DeviceRowLess = RowLess;
RF_HD DeviceRow device_row(Point point, U id) { return {point, id}; }
#endif
#ifdef ER_CUB_SORT
struct MergeScratch {
  void *ptr = nullptr;
  explicit MergeScratch(size_t bytes) {
    if (bytes)
      cuda_check(cudaMalloc(&ptr, bytes));
  }
  MergeScratch(const MergeScratch &) = delete;
  MergeScratch &operator=(const MergeScratch &) = delete;
  ~MergeScratch() { if (ptr) cudaFree(ptr); }
};
inline size_t merge_sort_bytes(U count) {
  size_t bytes = 0;
  // Explicit signed64-bit count: the admitted row cap can exceed INT_MAX.
  // A null scratch pointer requests sizing only, without touching keys.
  cuda_check(cub::DeviceMergeSort::SortKeys(
      nullptr, bytes, static_cast<DeviceRow *>(nullptr), int64_t(count),
      DeviceRowLess{}));
  return bytes;
}
#endif
RF_HD bool row_less_key(const Row &row, Point key) {
  return compare(row.point, key) < 0;
}
RF_HD bool row_less_key(const CompactRow &row, Point key) {
  return row.key < point_key(key);
}
RF_HD bool row_less_key(const DenseRow &row, Point key) {
  return row.key() < point_key(key);
}
__global__ void build_left(Curve c, Bases b, unsigned split, U count,
                           DeviceRow *rows) {
  U id = U(blockIdx.x) * BLOCK + threadIdx.x;
  Jac sum = id < count ? tuple_sum(c, b, 0, split, id) : Jac{};
  Point point = normalize_block(c, sum);
  if (id < count)
    rows[id] = device_row(point, id);
}
__global__ void build_partial(Curve c, Bases b, unsigned first, unsigned end,
                              U count, Point *points) {
  U id = U(blockIdx.x) * BLOCK + threadIdx.x;
  Jac sum = id < count ? tuple_sum(c, b, first, end, id) : Jac{};
  Point point = normalize_block(c, sum);
  if (id < count)
    points[id] = point;
}
#ifdef ER_RIGHT_FULL_CACHE
// With ER_QUERY_CACHED, the full cache takes precedence and the charged
// partial tables serve as the exact fallback when full-cache admission fails.
__global__ void build_right_cache(Curve c, Bases b, unsigned split, U count,
                                 CachedPoint *out) {
  U id = U(blockIdx.x) * BLOCK + threadIdx.x;
  Jac sum = id < count ? tuple_sum(c, b, split, b.slots, id) : Jac{};
  Point point = normalize_block(c, sum);
  if (id < count)
    out[id] = CachedPoint::from(point);
}
#endif
#ifdef ER_NEGATIVE_FILTER
RF_HD U filter_row_key(const Row &r) { return point_key(r.point); }
RF_HD U filter_row_key(const CompactRow &r) { return r.key; }
RF_HD U filter_row_key(const DenseRow &r) { return r.key(); }
template <class TableRow>
__global__ void build_negative_filter(const TableRow *rows, U count,
                                      unsigned long long *words, U word_mask) {
  U i = U(blockIdx.x) * BLOCK + threadIdx.x;
  if (i < count) {
    const U h = filter_hash(filter_row_key(rows[i]));
    atomicOr(words + ((h >> 32) & word_mask),
             static_cast<unsigned long long>(filter_pattern(h)));
  }
}
#endif
struct RightPartials {
  const Point *first = nullptr, *second = nullptr;
  U second_size = 0;
#ifdef ER_NEGATIVE_FILTER
  NegativeFilter filter;
#endif
#ifdef ER_RIGHT_FULL_CACHE
  const CachedPoint *full = nullptr;
#endif
};
// A radix encoding preserves the existing right-then-left witness order.
// Reserve UINT64_MAX for no match; fall back to the separate array when the
// full batch cannot be encoded without overflow (including non-service users).
inline bool packed_winner_fits(U left, U count) {
  return left && count <= UINT64_MAX / left;
}
struct PrefixLayout {
  U bins;
  unsigned shift;
};
inline PrefixLayout prefix_layout(U prime, U row_count) {
  U bins = 0;
  unsigned shift = 0;
#ifndef ER_QUERY_BINARY_REFERENCE
  if (row_count >= 4096) {
    unsigned bits = 0;
    for (U v = prime - 1; v; v >>= 1)
      ++bits;
    bins = 1;
    unsigned index_bits = 0;
    // Larger tables otherwise leave thousands of keys in every bucket.
    // Opt in to at most 2^24 buckets (128 MiB plus one offset); setup
    // and the actual offsets remain included in cold work and admission.
#ifdef ER_WIDE_INDEX
    constexpr U max_bins = U(1) << 24;
#else
    constexpr U max_bins = U(1) << 16;
#endif
    while (bins < max_bins && bins * 2 <= row_count / 32 && index_bits < bits) {
      bins *= 2;
      ++index_bits;
    }
    shift = bits - index_bits;
  }
#endif
  return {bins, shift};
}
// Index the high x bits of the sorted full-point table. Each bucket retains
// the original row order (including duplicate-point ids). Infinity sorts
// before finite points and deliberately uses the unindexed lookup.
template <class TableRow>
__global__ void build_prefix(const TableRow *rows, U size, U bins, unsigned shift,
                             U *offsets) {
  U bucket = U(blockIdx.x) * BLOCK + threadIdx.x;
  if (bucket > bins)
    return;
  if (bucket == bins) {
    offsets[bucket] = size;
    return; // The conceptual x=2^63 endpoint has no compact u64 key.
  }
  Point key{bucket << shift, 0, false};
  U lo = 0, hi = size;
  while (lo < hi) {
    U mid = lo + (hi - lo) / 2;
    if (row_less_key(rows[mid], key))
      lo = mid + 1;
    else
      hi = mid;
  }
  offsets[bucket] = lo;
}
template <class TableRow>
RF_HD U indexed_lookup(const TableRow *rows, U size, Point key, const U *offsets,
                       unsigned shift) {
  if (!offsets || key.inf)
    return lookup(rows, size, key);
  U bucket = key.x >> shift;
  U first = offsets[bucket], last = offsets[bucket + 1];
  return lookup(rows + first, last - first, key);
}
template <class TableRow>
RF_HD U relation_lookup(const TableRow *rows, U size, Point key,
                         const U *offsets, unsigned shift,
                         const RightPartials &partials) {
#ifdef ER_NEGATIVE_FILTER
  if (!partials.filter.maybe(point_key(key)))
    return UINT64_MAX;
#else
  (void)partials;
#endif
  return indexed_lookup(rows, size, key, offsets, shift);
}
template <bool Packed>
__global__ void search_right(Curve c, Bases b, unsigned split, Point target,
                             U start, U count, const DeviceRow *left, U left_size,
                             U *matches, unsigned long long *winner,
                             const U *offsets, unsigned shift,
                             RightPartials partials) {
  U local = U(blockIdx.x) * BLOCK + threadIdx.x;
  // Form target - sum(right) in Jacobian coordinates, then normalize once.
  Jac sum;
  if (local < count) {
    U id = start + local;
#ifdef ER_RIGHT_FULL_CACHE
    if (partials.full) {
      sum = c.jac(partials.full[id].point());
    } else
#endif
    if (partials.first) {
      sum = c.mixed(sum, partials.first[id / partials.second_size]);
      sum = c.mixed(sum, partials.second[id % partials.second_size]);
    } else {
      sum = tuple_sum(c, b, split, b.slots, id);
    }
  }
  if (sum.y)
    sum.y = c.f.p - sum.y;
  if (local < count)
    sum = c.mixed(sum, target);
  Point residual = normalize_block(c, sum);
  if (local < count) {
    U l = relation_lookup(left, left_size, residual, offsets, shift, partials);
    if constexpr (!Packed)
      matches[local] = l;
    if (l != UINT64_MAX) {
      U key = Packed ? local * left_size + l : local;
      atomicMin(winner, (unsigned long long)key);
    }
  }
}
#ifdef ER_QUERY_PERSISTENT
// A fixed grid visits disjoint BLOCK-sized tiles in increasing grid strides.
// A tile may stop only above an already found right ID, so every ID below the
// final minimum is still examined. No cross-block barrier or spin wait exists.
__global__ void search_right_persistent(
    Curve c, Bases b, unsigned split, Point target, U count,
    const DeviceRow *left, U left_size, unsigned long long *result,
    const U *offsets, unsigned shift, RightPartials partials) {
  __shared__ unsigned long long bound;
  cuda::atomic_ref<unsigned long long, cuda::thread_scope_device> best(result[0]);
  U visited = 0;
  const U stride = U(gridDim.x) * BLOCK;
  for (U start = U(blockIdx.x) * BLOCK; start < count;) {
    if (!threadIdx.x)
      bound = best.load(cuda::memory_order_relaxed);
    __syncthreads();
    if (start > bound)
      break; // Uniform for the entire block, including padded lanes.
    const U count_here = count - start < BLOCK ? count - start : BLOCK;
    const bool active = threadIdx.x < count_here;
    U id = active ? start + threadIdx.x : 0;
    Jac sum;
    if (active) {
#ifdef ER_RIGHT_FULL_CACHE
      if (partials.full) {
        sum = c.jac(partials.full[id].point());
      } else
#endif
      if (partials.first) {
        sum = c.mixed(sum, partials.first[id / partials.second_size]);
        sum = c.mixed(sum, partials.second[id % partials.second_size]);
      } else {
        sum = tuple_sum(c, b, split, b.slots, id);
      }
      if (sum.y)
        sum.y = c.f.p - sum.y;
      sum = c.mixed(sum, target);
    }
    Point residual = normalize_block(c, sum);
    if (active && id <= bound &&
        relation_lookup(left, left_size, residual, offsets, shift, partials) != UINT64_MAX)
      atomicMin(result, static_cast<unsigned long long>(id));
    if (!threadIdx.x)
      visited += count_here;
    // normalize_block owns reusable shared storage; all reads must finish
    // before any lane enters its next iteration or rewrites the shared bound.
    __syncthreads();
    if (count - start <= stride)
      break;
    start += stride;
  }
  if (!threadIdx.x)
    atomicAdd(result + 2, static_cast<unsigned long long>(visited));
}
// Resolve the minimum duplicate left ID only after the right minimum is final.
// Keeping two IDs separate avoids a potentially overflowing packed product.
__global__ void resolve_persistent_winner(
    Curve c, Bases b, unsigned split, Point target, const DeviceRow *left,
    U left_size, unsigned long long *result, const U *offsets, unsigned shift,
    RightPartials partials) {
  const U id = result[0];
  if (id == UINT64_MAX)
    return; // Uniform; previous kernel completed on the same stream.
  Jac sum;
  if (!threadIdx.x) {
#ifdef ER_RIGHT_FULL_CACHE
    if (partials.full) {
      sum = c.jac(partials.full[id].point());
    } else
#endif
    if (partials.first) {
      sum = c.mixed(sum, partials.first[id / partials.second_size]);
      sum = c.mixed(sum, partials.second[id % partials.second_size]);
    } else {
      sum = tuple_sum(c, b, split, b.slots, id);
    }
    if (sum.y)
      sum.y = c.f.p - sum.y;
    sum = c.mixed(sum, target);
  }
  Point residual = normalize_block(c, sum);
  if (!threadIdx.x)
    result[1] = relation_lookup(left, left_size, residual, offsets, shift, partials);
}
#endif
__global__ void lift_x(Curve c, const U *xs, U n, Point *out) {
  U i = U(blockIdx.x) * BLOCK + threadIdx.x;
  if (i >= n)
    return;
  Point p;
  c.lift(xs[i], 0, p);
  out[2 * i] = p;
  Point q = c.neg(p);
  out[2 * i + 1] = equal(p, q) ? Point{} : q;
}
__global__ void hash_base(Curve c, Fp64Ctx h, U inv, U beta, U tau,
                          Point *out) {
  U t = U(blockIdx.x) * BLOCK + threadIdx.x;
  if (t >= tau)
    return;
  U v = fp64_to_mont(h, t >= beta ? t - beta : t + h.p - beta);
  U z = fp64_from_mont(h, fp64_mul(h, inv, v));
  Point q;
  if (z < 2 * c.f.p)
    c.lift(z / 2, z & 1, q);
  out[t] = q;
}
__global__ void hash_base_wide(Curve c, U modulus, U inv, U beta, U tau,
                               Point *out) {
  U t = U(blockIdx.x) * BLOCK + threadIdx.x;
  if (t >= tau)
    return;
  U v = t >= beta ? t - beta : modulus - (beta - t);
  U z = modulus < (U(1) << 63) ? hash_mul63(inv, v, modulus)
                                     : hash_mul64(inv, v, modulus);
  Point q;
  if (z < 2 * c.f.p)
    c.lift(z / 2, z & 1, q);
  out[t] = q;
}
struct IsInfinity {
  RF_HD bool operator()(Point p) const { return p.inf; }
};
struct PointLess {
  RF_HD bool operator()(Point a, Point b) const { return compare(a, b) < 0; }
};
template <class T> T *data(thrust::device_vector<T> &v) {
  return thrust::raw_pointer_cast(v.data());
}
class DeviceBackend {
  Curve c;
  Plan p;
  U cap, chunk;
  bool packed = false;
#ifdef ER_QUERY_PERSISTENT
  unsigned persistent_grid = 1;
  static constexpr unsigned result_words = 3;
#else
  static constexpr unsigned result_words = 1;
#endif
  thrust::device_vector<Point> points;
  thrust::device_vector<DeviceRow> rows;
  thrust::device_vector<U> matches;
  thrust::device_vector<U> prefixes;
#ifdef ER_NEGATIVE_FILTER
  thrust::device_vector<unsigned long long> filter_words;
#endif
  thrust::device_vector<Point> right_first, right_second;
#ifdef ER_RIGHT_FULL_CACHE
  thrust::device_vector<CachedPoint> right_full;
  U right_cache_limit = UINT64_MAX;
#endif
  unsigned prefix_shift = 0;
  thrust::device_vector<unsigned long long> winner;
  void admission(U bytes) {
    size_t free = 0, total = 0;
    cuda_check(cudaMemGetInfo(&free, &total));
    if (bytes > free - free / 4)
      throw std::runtime_error(
          "LIMIT insufficient GPU memory including sort reserve");
  }
  static unsigned blocks(U n) {
    U b = n / BLOCK + (n % BLOCK != 0);
    if (b > 2147483647)
      throw std::runtime_error("LIMIT CUDA grid size");
    return (unsigned)b;
  }
  std::vector<Point> compact(thrust::device_vector<Point> &v) {
    auto end = thrust::remove_if(v.begin(), v.end(), IsInfinity{});
    v.erase(end, v.end());
    thrust::sort(v.begin(), v.end(), PointLess{});
    std::vector<Point> out(v.size());
    if (!out.empty())
      cuda_check(cudaMemcpy(out.data(), data(v), out.size() * sizeof(Point),
                            cudaMemcpyDeviceToHost));
    return out;
  }

public:
  U cached_partial_rows() const {
    return right_first.size() + right_second.size();
  }
  DeviceBackend(Curve curve, U max_rows, U batch, U full_cache_limit = UINT64_MAX)
      : c(curve), cap(max_rows), chunk(batch) {
#ifdef ER_RIGHT_FULL_CACHE
    right_cache_limit = full_cache_limit;
#else
    (void)full_cache_limit;
#endif
    if (!cap || !chunk || chunk > cap)
      throw std::runtime_error("invalid GPU row/chunk caps");
    blocks(cap);
    cuda_check(cudaFree(nullptr));
#ifdef ER_QUERY_PERSISTENT
    int device = 0, processors = 0;
    cuda_check(cudaGetDevice(&device));
    cuda_check(cudaDeviceGetAttribute(&processors, cudaDevAttrMultiProcessorCount,
                                      device));
    if (processors <= 0 || processors > 1048576)
      throw std::runtime_error("invalid CUDA processor count");
    persistent_grid = unsigned(processors) * 8;
#ifdef ER_PERSISTENT_BLOCK_LIMIT
    static_assert(ER_PERSISTENT_BLOCK_LIMIT > 0, "positive persistent grid limit");
    persistent_grid = std::min(persistent_grid, unsigned(ER_PERSISTENT_BLOCK_LIMIT));
#endif
#endif
  }
  U cached_right_rows() const {
#ifdef ER_RIGHT_FULL_CACHE
    return right_full.size();
#else
    return 0;
#endif
  }
  U negative_filter_bytes() const {
#ifdef ER_NEGATIVE_FILTER
    return filter_words.size() * sizeof(unsigned long long);
#else
    return 0;
#endif
  }
  void prepare(const Plan &next) {
#ifdef ER_NEGATIVE_FILTER
    filter_words.clear();
    filter_words.shrink_to_fit();
#endif
    rows.clear();
    rows.shrink_to_fit();
    points.clear();
    points.shrink_to_fit();
    matches.clear();
    matches.shrink_to_fit();
    winner.clear();
    winner.shrink_to_fit();
    prefixes.clear();
    prefixes.shrink_to_fit();
    right_first.clear();
    right_first.shrink_to_fit();
    right_second.clear();
    right_second.shrink_to_fit();
#ifdef ER_RIGHT_FULL_CACHE
    right_full.clear();
    right_full.shrink_to_fit();
#endif
    p = next;
#ifdef ER_DENSE_ROWS
    if (p.left > (U(1) << 32))
      throw std::runtime_error("LIMIT dense tuple ID capacity");
#endif
    packed = packed_winner_fits(p.left, std::min(chunk, p.right));
#ifdef ER_QUERY_REFERENCE
    packed = false;
#endif
#ifdef ER_QUERY_PERSISTENT
    // This path stores separate global IDs and needs no per-tuple match array.
    packed = true;
#endif
    U first_count = 0, second_count = 0;
    unsigned middle = 0;
#if defined(ER_QUERY_CACHED) && !defined(ER_QUERY_UNCACHED_REFERENCE)
    // Each partial table is capped by the explicit chunk bound. This reuses
    // short sums without retaining the exponentially larger full right table.
    if (p.right >= 4096 && p.slots - p.split >= 3) {
      for (unsigned cut = p.split + 1; cut < p.slots; ++cut) {
        U a = 1, b = 1;
        for (unsigned j = p.split; j < p.slots; ++j) {
          U n = p.offsets[j + 1] - p.offsets[j];
          if (j < cut)
            a = product(a, n);
          else
            b = product(b, n);
        }
        if (a <= chunk && b <= chunk &&
            (!middle || a + b < first_count + second_count)) {
          middle = cut;
          first_count = a;
          second_count = b;
        }
      }
    }
#endif
    const auto layout = prefix_layout(c.f.p, p.left);
    U bins = layout.bins;
    prefix_shift = layout.shift;
    const U row_bytes = product(p.left, sizeof(DeviceRow));
#ifdef ER_CUB_SORT
    const size_t sort_bytes = merge_sort_bytes(p.left);
    [[maybe_unused]] constexpr unsigned queried_sort = 1;
#else
    // Legacy Thrust admission reserves two additional complete row buffers.
    const size_t sort_bytes = product(row_bytes, 2);
    [[maybe_unused]] constexpr unsigned queried_sort = 0;
#endif
    if (sort_bytes > UINT64_MAX - row_bytes)
      throw std::runtime_error("LIMIT sort allocation overflow");
    U bytes = row_bytes + sort_bytes +
              product(p.points.size(), sizeof(Point)) +
              (packed ? 0 : product(chunk, sizeof(U))) + result_words * sizeof(U) +
              (bins ? product(bins + 1, sizeof(U)) : 0) +
              product(first_count + second_count, sizeof(Point));
    [[maybe_unused]] U filter_count = 0;
#ifdef ER_NEGATIVE_FILTER
    filter_count = filter_word_count(p.left);
#endif
    const U filter_bytes = product(filter_count, sizeof(unsigned long long));
    const U retained = bytes - sort_bytes + filter_bytes;
    // Filter construction follows sort scratch release, before cache allocation.
    bytes = std::max(bytes, retained);
    [[maybe_unused]] U full_count = 0;
#ifdef ER_RIGHT_FULL_CACHE
    size_t free_bytes = 0, total_bytes = 0;
    cuda_check(cudaMemGetInfo(&free_bytes, &total_bytes));
    if (right_cache_fits(p.right, retained, free_bytes, right_cache_limit)) {
      full_count = p.right;
      // Sort scratch is released before the full cache is allocated. Charge
      // the greater phase peak, rather than simultaneously-live storage.
      bytes = std::max(bytes, retained + product(full_count, sizeof(CachedPoint)));
    }
#endif
#ifdef ER_TABLE_DIAGNOSTICS
    std::fprintf(stderr,
                 "TABLE left=%llu right=%llu bins=%llu admission_bytes=%llu "
                 "partial_first=%llu partial_second=%llu row_width=%u "
                 "sort_bytes=%llu sort_queried=%u right_cache_rows=%llu filter_bytes=%llu\n",
                 (unsigned long long)p.left, (unsigned long long)p.right,
                 (unsigned long long)bins, (unsigned long long)bytes,
                 (unsigned long long)first_count,
                 (unsigned long long)second_count, unsigned(sizeof(DeviceRow)),
                 (unsigned long long)sort_bytes, queried_sort,
                 (unsigned long long)full_count, (unsigned long long)filter_bytes);
#endif
    admission(bytes);
    points.assign(p.points.begin(), p.points.end());
    rows.resize(p.left);
    if (!packed)
      matches.resize(chunk);
    winner.resize(result_words);
    if (middle) {
      right_first.resize(first_count);
      right_second.resize(second_count);
      build_partial<<<blocks(first_count), BLOCK>>>(
          c, p.view(data(points)), p.split, middle, first_count,
          data(right_first));
      cuda_check(cudaGetLastError());
      build_partial<<<blocks(second_count), BLOCK>>>(
          c, p.view(data(points)), middle, p.slots, second_count,
          data(right_second));
      cuda_check(cudaGetLastError());
    }
    if (p.left) {
      build_left<<<blocks(p.left), BLOCK>>>(c, p.view(data(points)), p.split,
                                            p.left, data(rows));
      cuda_check(cudaGetLastError());
#ifdef ER_CUB_SORT
      {
        MergeScratch scratch(sort_bytes);
        size_t available = sort_bytes;
        cuda_check(cub::DeviceMergeSort::SortKeys(
            scratch.ptr, available, data(rows), int64_t(p.left), DeviceRowLess{}));
        cuda_check(cudaDeviceSynchronize());
      } // Scratch is released before indexing and relation queries.
#else
      thrust::sort(rows.begin(), rows.end(), DeviceRowLess{});
#endif
      if (bins) {
        prefixes.resize(bins + 1);
        build_prefix<<<blocks(bins + 1), BLOCK>>>(data(rows), p.left, bins,
                                                  prefix_shift, data(prefixes));
        cuda_check(cudaGetLastError());
      }
      cuda_check(cudaDeviceSynchronize());
    }
#ifdef ER_NEGATIVE_FILTER
    if (filter_count) {
      filter_words.resize(filter_count);
      cuda_check(cudaMemsetAsync(data(filter_words), 0, filter_bytes));
      build_negative_filter<<<blocks(p.left), BLOCK>>>(
          data(rows), p.left, data(filter_words), filter_count - 1);
      cuda_check(cudaGetLastError());
      cuda_check(cudaDeviceSynchronize());
    }
#endif
#ifdef ER_RIGHT_FULL_CACHE
    if (full_count) {
      right_full.resize(full_count);
      build_right_cache<<<blocks(full_count), BLOCK>>>(
          c, p.view(data(points)), p.split, full_count, data(right_full));
      cuda_check(cudaGetLastError());
      cuda_check(cudaDeviceSynchronize());
    }
#endif
  }
  Result find(Point target, U max_work) {
    if (!c.on_curve(target))
      throw std::runtime_error("invalid target");
    if (p.left + p.right > max_work)
      throw std::runtime_error("LIMIT decomposition work; not nonexistence");
    Result r;
    RightPartials partials;
    if (!right_first.empty())
      partials = {data(right_first), data(right_second), right_second.size()};
#ifdef ER_RIGHT_FULL_CACHE
    if (!right_full.empty())
      partials.full = data(right_full);
#endif
#ifdef ER_NEGATIVE_FILTER
    if (!filter_words.empty())
      partials.filter = {data(filter_words), filter_words.size() - 1};
#endif
#ifdef ER_QUERY_PERSISTENT
    if (p.right) {
      cuda_check(cudaMemsetAsync(data(winner), 0xff, 2 * sizeof(U)));
      cuda_check(cudaMemsetAsync(data(winner) + 2, 0, sizeof(U)));
      search_right_persistent<<<std::min(persistent_grid, blocks(p.right)), BLOCK>>>(
          c, p.view(data(points)), p.split, target, p.right, data(rows), p.left,
          data(winner), prefixes.empty() ? nullptr : data(prefixes), prefix_shift,
          partials);
      cuda_check(cudaGetLastError());
      resolve_persistent_winner<<<1, BLOCK>>>(
          c, p.view(data(points)), p.split, target, data(rows), p.left,
          data(winner), prefixes.empty() ? nullptr : data(prefixes), prefix_shift,
          partials);
      cuda_check(cudaGetLastError());
      unsigned long long result[3];
      cuda_check(cudaMemcpy(result, data(winner), sizeof(result),
                            cudaMemcpyDeviceToHost));
      r.visited = result[2];
      if (r.visited > p.right)
        throw std::runtime_error("invalid persistent visit count");
      if (result[0] != UINT64_MAX) {
        if (result[0] >= p.right || result[1] >= p.left ||
            r.visited <= result[0])
          throw std::runtime_error("invalid persistent witness or coverage");
        r.found = true;
        r.indices = p.indices(result[1], result[0]);
      } else if (r.visited != p.right) {
        throw std::runtime_error("incomplete persistent miss");
      }
    }
#else
    for (U start = 0; start < p.right;) {
      U n = std::min(chunk, p.right - start);
      unsigned long long best = UINT64_MAX;
#if defined(ER_QUERY_REFERENCE) || defined(ER_QUERY_PACKED_ONLY)
      cuda_check(cudaMemcpy(data(winner), &best, sizeof(best),
                            cudaMemcpyHostToDevice));
#else
      // Same default stream as the search and blocking readback: no host
      // upload/staging is needed, and the reset completes before the search.
      cuda_check(cudaMemsetAsync(data(winner), 0xff, sizeof(best)));
#endif
      if (packed)
        search_right<true><<<blocks(n), BLOCK>>>(
            c, p.view(data(points)), p.split, target, start, n, data(rows),
            p.left, nullptr, data(winner),
            prefixes.empty() ? nullptr : data(prefixes), prefix_shift,
            partials);
      else
        search_right<false><<<blocks(n), BLOCK>>>(
            c, p.view(data(points)), p.split, target, start, n, data(rows),
            p.left, data(matches), data(winner),
            prefixes.empty() ? nullptr : data(prefixes), prefix_shift,
            partials);
      cuda_check(cudaGetLastError());
      cuda_check(cudaMemcpy(&best, data(winner), sizeof(best),
                            cudaMemcpyDeviceToHost));
      r.visited += n;
      if (best != UINT64_MAX) {
        U l;
        if (packed) {
          l = best % p.left;
          best /= p.left;
        } else {
          cuda_check(cudaMemcpy(&l, data(matches) + best, sizeof(l),
                                cudaMemcpyDeviceToHost));
        }
        r.found = true;
        r.indices = p.indices(l, start + best);
        break;
      }
      start += n;
    }
#endif
    verify(c, p, target, r);
    return r;
  }
  std::vector<Point> lift(const std::vector<U> &xs) {
    if (xs.size() > cap / 2)
      throw std::runtime_error("LIMIT explicit lifts exceed row cap");
    admission(product(xs.size(), sizeof(U) + 6 * sizeof(Point)));
    thrust::device_vector<U> input(xs.begin(), xs.end());
    thrust::device_vector<Point> out(xs.size() * 2);
    if (xs.empty())
      return {};
    lift_x<<<blocks(xs.size()), BLOCK>>>(c, data(input), xs.size(), data(out));
    cuda_check(cudaGetLastError());
    auto result = compact(out);
    for (U x : xs) {
      auto it = std::lower_bound(result.begin(), result.end(), x,
                                 [](Point p, U v) { return p.x < v; });
      if (it == result.end() || it->x != x)
        throw std::runtime_error("x does not lift");
    }
    return result;
  }
  std::vector<Point> hash(U s, U alpha, U beta, U tau) {
    admission(product(tau, 3 * sizeof(Point)));
    U canonical_inv = modpow(alpha, s - 2, s);
    thrust::device_vector<Point> out(tau);
    if (s < (U(1) << 62)) {
      Fp64Ctx h = fp64_ctx_new(s);
      U inv = fp64_to_mont(h, canonical_inv);
      hash_base<<<blocks(tau), BLOCK>>>(c, h, inv, beta, tau, data(out));
    } else {
      hash_base_wide<<<blocks(tau), BLOCK>>>(c, s, canonical_inv, beta, tau, data(out));
    }
    cuda_check(cudaGetLastError());
    return compact(out);
  }
};
} // namespace explicit_relations
