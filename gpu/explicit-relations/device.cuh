#pragma once
#include "host.hpp"
#include <cstdio>
#include <cuda_runtime.h>
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
__global__ void build_left(Curve c, Bases b, unsigned split, U count,
                           Row *rows) {
  U id = U(blockIdx.x) * BLOCK + threadIdx.x;
  Jac sum = id < count ? tuple_sum(c, b, 0, split, id) : Jac{};
  Point point = normalize_block(c, sum);
  if (id < count)
    rows[id] = {point, id};
}
__global__ void build_partial(Curve c, Bases b, unsigned first, unsigned end,
                              U count, Point *points) {
  U id = U(blockIdx.x) * BLOCK + threadIdx.x;
  Jac sum = id < count ? tuple_sum(c, b, first, end, id) : Jac{};
  Point point = normalize_block(c, sum);
  if (id < count)
    points[id] = point;
}
struct RightPartials {
  const Point *first = nullptr, *second = nullptr;
  U second_size = 0;
};
// A radix encoding preserves the existing right-then-left witness order.
// Reserve UINT64_MAX for no match; fall back to the separate array when the
// full batch cannot be encoded without overflow (including non-service users).
inline bool packed_winner_fits(U left, U count) {
  return left && count <= UINT64_MAX / left;
}
// Index the high x bits of the sorted full-point table. Each bucket retains
// the original row order (including duplicate-point ids). Infinity sorts
// before finite points and deliberately uses the unindexed lookup.
__global__ void build_prefix(const Row *rows, U size, U bins, unsigned shift,
                             U *offsets) {
  U bucket = U(blockIdx.x) * BLOCK + threadIdx.x;
  if (bucket > bins)
    return;
  Point key{bucket << shift, 0, false};
  U lo = 0, hi = size;
  while (lo < hi) {
    U mid = lo + (hi - lo) / 2;
    if (compare(rows[mid].point, key) < 0)
      lo = mid + 1;
    else
      hi = mid;
  }
  offsets[bucket] = lo;
}
RF_HD U indexed_lookup(const Row *rows, U size, Point key, const U *offsets,
                       unsigned shift) {
  if (!offsets || key.inf)
    return lookup(rows, size, key);
  U bucket = key.x >> shift;
  U first = offsets[bucket], last = offsets[bucket + 1];
  return lookup(rows + first, last - first, key);
}
template <bool Packed>
__global__ void search_right(Curve c, Bases b, unsigned split, Point target,
                             U start, U count, const Row *left, U left_size,
                             U *matches, unsigned long long *winner,
                             const U *offsets, unsigned shift,
                             RightPartials partials) {
  U local = U(blockIdx.x) * BLOCK + threadIdx.x;
  // Form target - sum(right) in Jacobian coordinates, then normalize once.
  Jac sum;
  if (local < count) {
    U id = start + local;
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
    U l = indexed_lookup(left, left_size, residual, offsets, shift);
    if constexpr (!Packed)
      matches[local] = l;
    if (l != UINT64_MAX) {
      U key = Packed ? local * left_size + l : local;
      atomicMin(winner, (unsigned long long)key);
    }
  }
}
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
  thrust::device_vector<Point> points;
  thrust::device_vector<Row> rows;
  thrust::device_vector<U> matches;
  thrust::device_vector<U> prefixes;
  thrust::device_vector<Point> right_first, right_second;
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
  DeviceBackend(Curve curve, U max_rows, U batch)
      : c(curve), cap(max_rows), chunk(batch) {
    if (!cap || !chunk || chunk > cap)
      throw std::runtime_error("invalid GPU row/chunk caps");
    blocks(cap);
    cuda_check(cudaFree(nullptr));
  }
  void prepare(const Plan &next) {
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
    p = next;
    packed = packed_winner_fits(p.left, std::min(chunk, p.right));
#ifdef ER_QUERY_REFERENCE
    packed = false;
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
    U bins = 0;
#ifndef ER_QUERY_BINARY_REFERENCE
    if (p.left >= 4096) {
      unsigned bits = 0;
      for (U v = c.f.p - 1; v; v >>= 1)
        ++bits;
      bins = 1;
      unsigned index_bits = 0;
      while (bins < 65536 && bins * 2 <= p.left / 32 && index_bits < bits) {
        bins *= 2;
        ++index_bits;
      }
      prefix_shift = bits - index_bits;
    }
#endif
    // 3 row buffers cover the retained table and conservative sort scratch.
    U bytes = product(p.left, 3 * sizeof(Row)) +
              product(p.points.size(), sizeof(Point)) +
              (packed ? 0 : product(chunk, sizeof(U))) + sizeof(U) +
              (bins ? product(bins + 1, sizeof(U)) : 0) +
              product(first_count + second_count, sizeof(Point));
#ifdef ER_TABLE_DIAGNOSTICS
    std::fprintf(stderr,
                 "TABLE left=%llu right=%llu bins=%llu admission_bytes=%llu "
                 "partial_first=%llu partial_second=%llu\n",
                 (unsigned long long)p.left, (unsigned long long)p.right,
                 (unsigned long long)bins, (unsigned long long)bytes,
                 (unsigned long long)first_count,
                 (unsigned long long)second_count);
#endif
    admission(bytes);
    points.assign(p.points.begin(), p.points.end());
    rows.resize(p.left);
    if (!packed)
      matches.resize(chunk);
    winner.resize(1);
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
      thrust::sort(rows.begin(), rows.end(), RowLess{});
      if (bins) {
        prefixes.resize(bins + 1);
        build_prefix<<<blocks(bins + 1), BLOCK>>>(data(rows), p.left, bins,
                                                  prefix_shift, data(prefixes));
        cuda_check(cudaGetLastError());
      }
      cuda_check(cudaDeviceSynchronize());
    }
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
    Fp64Ctx h = fp64_ctx_new(s);
    U inv = fp64_to_mont(h, modpow(alpha, s - 2, s));
    thrust::device_vector<Point> out(tau);
    hash_base<<<blocks(tau), BLOCK>>>(c, h, inv, beta, tau, data(out));
    cuda_check(cudaGetLastError());
    return compact(out);
  }
};
} // namespace explicit_relations
