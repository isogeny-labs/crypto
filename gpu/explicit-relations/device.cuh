#pragma once
#include "host.hpp"
#include <cuda_runtime.h>
#include <thrust/device_vector.h>
#include <thrust/remove.h>
#include <thrust/sort.h>

namespace explicit_relations {
inline void cuda_check(cudaError_t e) {
  if (e != cudaSuccess)
    throw std::runtime_error(cudaGetErrorString(e));
}
constexpr unsigned BLOCK = 256;
// One inversion per block, including partially active blocks and infinity.
// Every thread participates in both scans and barriers. Infinity contributes 1.
__device__ Point normalize_block(Curve c, Jac p) {
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
}
__global__ void build_left(Curve c, Bases b, unsigned split, U count,
                           Row *rows) {
  U id = U(blockIdx.x) * BLOCK + threadIdx.x;
  Jac sum = id < count ? tuple_sum(c, b, 0, split, id) : Jac{};
  Point point = normalize_block(c, sum);
  if (id < count)
    rows[id] = {point, id};
}
__global__ void search_right(Curve c, Bases b, unsigned split, Point target,
                             U start, U count, const Row *left, U left_size,
                             U *matches, unsigned long long *winner) {
  U local = U(blockIdx.x) * BLOCK + threadIdx.x;
  // Form target - sum(right) in Jacobian coordinates, then normalize once.
  Jac sum =
      local < count ? tuple_sum(c, b, split, b.slots, start + local) : Jac{};
  if (sum.y)
    sum.y = c.f.p - sum.y;
  if (local < count)
    sum = c.mixed(sum, target);
  Point residual = normalize_block(c, sum);
  if (local < count) {
    U l = lookup(left, left_size, residual);
    matches[local] = l;
    if (l != UINT64_MAX)
      atomicMin(winner, (unsigned long long)local);
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
  thrust::device_vector<Point> points;
  thrust::device_vector<Row> rows;
  thrust::device_vector<U> matches;
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
    p = next;
    // 3 row buffers cover the retained table and conservative sort scratch.
    U bytes = product(p.left, 3 * sizeof(Row)) +
              product(p.points.size(), sizeof(Point)) +
              product(chunk, sizeof(U)) + sizeof(U);
    admission(bytes);
    points.assign(p.points.begin(), p.points.end());
    rows.resize(p.left);
    matches.resize(chunk);
    winner.resize(1);
    if (p.left) {
      build_left<<<blocks(p.left), BLOCK>>>(c, p.view(data(points)), p.split,
                                            p.left, data(rows));
      cuda_check(cudaGetLastError());
      thrust::sort(rows.begin(), rows.end(), RowLess{});
      cuda_check(cudaDeviceSynchronize());
    }
  }
  Result find(Point target, U max_work) {
    if (!c.on_curve(target))
      throw std::runtime_error("invalid target");
    if (p.left + p.right > max_work)
      throw std::runtime_error("LIMIT decomposition work; not nonexistence");
    Result r;
    for (U start = 0; start < p.right;) {
      U n = std::min(chunk, p.right - start);
      unsigned long long best = UINT64_MAX;
      cuda_check(cudaMemcpy(data(winner), &best, sizeof(best),
                            cudaMemcpyHostToDevice));
      search_right<<<blocks(n), BLOCK>>>(c, p.view(data(points)), p.split,
                                         target, start, n, data(rows), p.left,
                                         data(matches), data(winner));
      cuda_check(cudaGetLastError());
      cuda_check(cudaMemcpy(&best, data(winner), sizeof(best),
                            cudaMemcpyDeviceToHost));
      r.visited += n;
      if (best != UINT64_MAX) {
        U l;
        cuda_check(cudaMemcpy(&l, data(matches) + best, sizeof(l),
                              cudaMemcpyDeviceToHost));
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
