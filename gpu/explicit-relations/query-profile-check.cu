#include "device.cuh"
#include <iostream>
using namespace explicit_relations;
int main() {
  unsigned cases = 0, hits = 0, misses = 0;
  for (U prime : {U(97), U(65521), (U(1) << 61) - 1}) {
    Curve c = curve(prime, prime == 97 ? 2 : 0, prime == 97 ? 3 : 7);
    std::vector<Point> ps{Point{}};
    for (U x = 0; ps.size() < 20; ++x) {
      Point p;
      if (c.lift(x, 0, p)) {
        ps.push_back(p);
        if (p.y) ps.push_back(c.neg(p));
      }
    }
    std::vector<DeviceRow> left{device_row(ps[0], 0), device_row(ps[1], 1), device_row(ps[2], 2)};
    std::sort(left.begin(), left.end(), DeviceRowLess{});
    thrust::device_vector<DeviceRow> dl(left.begin(), left.end());
    for (U count : {U(1), U(255), U(256), U(257), U(1025)}) {
      std::vector<CachedPoint> right;
      for (U i = 0; i < count; ++i)
        right.push_back(CachedPoint::from(ps[i % ps.size()]));
      thrust::device_vector<CachedPoint> dr(right.begin(), right.end());
      thrust::device_vector<unsigned long long> result(3);
      thrust::device_vector<QueryProfile> profiles(2);
      RightPartials partials{};
      partials.full = data(dr);
      for (Point target : ps) {
        U expected = UINT64_MAX;
        for (U i = 0; i < count; ++i) {
          Point residual = c.affine(c.affine_sub(target, right[i].point()));
          if (lookup(left.data(), left.size(), residual) != UINT64_MAX) {
            expected = i;
            break;
          }
        }
        for (bool staged : {false, true}) {
          cuda_check(cudaMemset(data(result), 0xff, 2 * sizeof(U)));
          cuda_check(cudaMemset(data(result) + 2, 0, sizeof(U)));
          if (staged)
            search_right_profile<<<2, BLOCK>>>(c, target, count, data(dl), left.size(),
                data(result), nullptr, 0, partials, data(profiles));
          else
            search_right_persistent<<<2, BLOCK>>>(c, Bases{}, 0, target, count,
                data(dl), left.size(), data(result), nullptr, 0, partials);
          cuda_check(cudaGetLastError());
          unsigned long long got[3];
          cuda_check(cudaMemcpy(got, data(result), sizeof(got), cudaMemcpyDeviceToHost));
          if (got[0] != expected || got[2] > count ||
              (expected == UINT64_MAX && got[2] != count))
            throw std::runtime_error("profile witness/coverage mismatch");
          if (staged) {
            QueryProfile host[2];
            cuda_check(cudaMemcpy(host, data(profiles), sizeof(host), cudaMemcpyDeviceToHost));
            if (host[0].rows + host[1].rows != got[2])
              throw std::runtime_error("profile row accounting mismatch");
          }
        }
        ++cases;
        expected == UINT64_MAX ? ++misses : ++hits;
      }
    }
  }
  if (!hits || !misses) throw std::runtime_error("missing positive/negative fixtures");
  std::cout << "PASS profile cases=" << cases << " hits=" << hits << " misses=" << misses
            << " tails/infinity/duplicates/minimum/coverage\n";
}
