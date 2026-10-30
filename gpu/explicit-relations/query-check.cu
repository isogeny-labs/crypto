// Differential query tests: deliberately repeated targets and prepares exercise
// reset ordering, duplicate sums, chunk tails, empty plans and radix bounds.
#include "device.cuh"
#include <iostream>
using namespace explicit_relations;
static void require(bool ok) {
  if (!ok)
    throw std::runtime_error("query boundary check failed");
}
template <class TableRow>
__global__ void check_index(const TableRow *rows, U size, const U *prefixes,
                            const Point *queries, U count, U *answers) {
  U i = U(blockIdx.x) * BLOCK + threadIdx.x;
  if (i < count)
    answers[i] = indexed_lookup(rows, size, queries[i], prefixes, 6);
}
static void prefix_boundaries() {
  // Synthetic sorted full keys deliberately leave empty buckets, repeat
  // infinity and finite keys, and straddle bucket/field endpoints.
  std::vector<Row> rows{{Point{}, 7}, {Point{}, 3}};
  for (U x : {U(0), U(1), U(63), U(64), U(4095), U(4096), U(65520)})
    for (U y : {U(0), U(5)})
      for (U id : {U(9), U(2)})
        rows.push_back({Point{x, y, false}, id});
  std::sort(rows.begin(), rows.end(), RowLess{});
  std::vector<Point> queries{Point{}};
  for (U x = 0; x < 65521; ++x)
    for (U y : {U(0), U(1), U(5)})
      queries.push_back(Point{x, y, false});
  thrust::device_vector<Row> device_rows(rows.begin(), rows.end());
  thrust::device_vector<Point> device_queries(queries.begin(), queries.end());
  thrust::device_vector<U> prefixes(1025), answers(queries.size());
  build_prefix<<<5, BLOCK>>>(data(device_rows), rows.size(), 1024, 6,
                             data(prefixes));
  cuda_check(cudaGetLastError());
  check_index<<<(queries.size() + BLOCK - 1) / BLOCK, BLOCK>>>(
      data(device_rows), rows.size(), data(prefixes), data(device_queries),
      queries.size(), data(answers));
  cuda_check(cudaGetLastError());
  std::vector<U> got(queries.size());
  cuda_check(cudaMemcpy(got.data(), data(answers), got.size() * sizeof(U),
                        cudaMemcpyDeviceToHost));
  for (size_t i = 0; i < queries.size(); ++i)
    require(got[i] == lookup(rows.data(), rows.size(), queries[i]));
}
static void compact_boundaries() {
  Curve c = curve(65521, 0, 7);
  std::vector<Row> full{{Point{}, 7}, {Point{}, 3}};
  std::vector<Point> queries{Point{}};
  // Exhaust every valid point on this curve, including opposite signs and
  // any y=0 point. Keep sparse table coverage so misses/empty buckets occur.
  for (U x = 0; x < c.f.p; ++x) {
    Point q;
    if (!c.lift(x, 0, q))
      continue;
    for (Point point : {q, c.neg(q)}) {
      queries.push_back(point);
      if (x % 13 == 0 || x < 2 || x > c.f.p - 8)
        for (U id : {U(9), U(2)})
          full.push_back({point, id});
    }
  }
  std::sort(full.begin(), full.end(), RowLess{});
  std::vector<CompactRow> rows;
  for (const auto &row : full)
    rows.push_back({point_key(row.point), row.id});
  std::sort(rows.begin(), rows.end(), CompactRowLess{});
  thrust::device_vector<CompactRow> device_rows(rows.begin(), rows.end());
  thrust::device_vector<Point> device_queries(queries.begin(), queries.end());
  thrust::device_vector<U> prefixes(1025), answers(queries.size());
  build_prefix<<<5, BLOCK>>>(data(device_rows), rows.size(), 1024, 6,
                             data(prefixes));
  cuda_check(cudaGetLastError());
  check_index<<<(queries.size() + BLOCK - 1) / BLOCK, BLOCK>>>(
      data(device_rows), rows.size(), data(prefixes), data(device_queries),
      queries.size(), data(answers));
  cuda_check(cudaGetLastError());
  std::vector<U> got(queries.size());
  cuda_check(cudaMemcpy(got.data(), data(answers), got.size() * sizeof(U),
                        cudaMemcpyDeviceToHost));
  for (size_t i = 0; i < queries.size(); ++i) {
    U expected = lookup(full.data(), full.size(), queries[i]);
    require(got[i] == expected);
    require(lookup(rows.data(), rows.size(), queries[i]) == expected);
  }
  require(point_key(Point{}) == 0);
  require(point_key(Point{0, 0, false}) == 2);
  require(point_key(Point{0, 1, false}) == 3);
  require(point_key(Point{(U(1) << 62) - 1, 1, false}) ==
          (U(1) << 63) + 1);
  // The largest prefix endpoint is representable too (not a curve point).
  require(point_key(Point{U(1) << 62, 0, false}) == (U(1) << 63) + 2);
}
int main() {
  try {
    prefix_boundaries();
    compact_boundaries();
    require(!packed_winner_fits(0, 1));
    require(packed_winner_fits(1, UINT64_MAX));
    require(packed_winner_fits(UINT64_MAX, 1));
    require(!packed_winner_fits(UINT64_MAX, 2));
    require(packed_winner_fits(U(1) << 32, (U(1) << 32) - 1));
    require(!packed_winner_fits(U(1) << 32, U(1) << 32));
    Curve c = curve(97, 0, 7);
    std::vector<Point> ps;
    for (U x = 0; x < 97; ++x) {
      Point q;
      if (c.lift(x, 0, q))
        ps.push_back(q);
    }
    // Exercise automatic index admission and rebuilding through the public
    // backend, including a table with >=4096 rows and a partial query block.
    Curve wide = curve(65521, 0, 7);
    std::vector<Point> base;
    for (U x = 0; base.size() < 65; ++x) {
      Point q;
      if (wide.lift(x, 0, q))
        base.push_back(q);
    }
    DeviceBackend indexed(wide, 8192, 257);
    HostBackend reference(wide, 8192, 257);
    for (unsigned size : {65, 3, 64}) {
      std::vector<std::vector<Point>> bases(4);
      for (auto &slot : bases)
        slot.assign(base.begin(), base.begin() + size);
      Plan p = plan(wide, bases, 8192);
      indexed.prepare(p);
      reference.prepare(p);
      for (Point target : {base.front(), Point{}, base.back()}) {
        auto got = indexed.find(target, p.left + p.right);
        auto expected = reference.find(target, p.left + p.right);
        require(got.found == expected.found && got.indices == expected.indices);
      }
    }
    for (U chunk : {U(1), U(31), U(256), U(257), U(4096)}) {
      DeviceBackend device(c, 4096, chunk);
      HostBackend host(c, 4096, chunk);
      for (unsigned size : {3, 17, 0, 3}) {
        std::vector<std::vector<Point>> bases(4);
        for (auto &base : bases)
          base.assign(ps.begin(), ps.begin() + size);
        Plan p = plan(c, bases, 4096);
        device.prepare(p);
        host.prepare(p);
        for (Point target : {ps.front(), Point{}, ps.back(), ps.front()}) {
          Result got = device.find(target, p.left + p.right);
          Result expected = host.find(target, p.left + p.right);
          require(got.found == expected.found &&
                  got.indices == expected.indices);
          U visited = p.right;
          if (got.found) {
            U right = 0;
            for (unsigned j = p.split; j < p.slots; ++j)
              right = right * size + expected.indices[j];
            visited = std::min(p.right, (right / chunk + 1) * chunk);
          }
          require(got.visited == visited);
        }
      }
    }
    std::vector<Point> signed_base;
    for (unsigned i = 0; i < 6; ++i) {
      signed_base.push_back(base[i]);
      signed_base.push_back(wide.neg(base[i]));
    }
    std::sort(signed_base.begin(), signed_base.end(), PointLess{});
    signed_base.erase(
        std::unique(signed_base.begin(), signed_base.end(), equal),
        signed_base.end());
    require(signed_base.size() >= 11);
    for (U chunk : {U(31), U(257)}) {
      DeviceBackend cached(wide, 32768, chunk);
      HostBackend exact(wide, 32768, chunk);
      for (unsigned size : {11, 0, 3, 11}) {
        std::vector<std::vector<Point>> bases(8);
        for (auto &slot : bases)
          slot.assign(signed_base.begin(), signed_base.begin() + size);
        Plan p = plan(wide, bases, 32768);
        cached.prepare(p);
        exact.prepare(p);
        for (Point target :
             {signed_base.front(), Point{}, signed_base.back()}) {
          auto got = cached.find(target, p.left + p.right);
          auto expected = exact.find(target, p.left + p.right);
          require(got.found == expected.found &&
                  got.indices == expected.indices);
          U visited = p.right;
          if (got.found) {
            U right = 0;
            for (unsigned j = p.split; j < p.slots; ++j)
              right = right * size + expected.indices[j];
            visited = std::min(p.right, (right / chunk + 1) * chunk);
          }
          require(got.visited == visited);
        }
      }
    }
    std::cout
        << "PASS packed overflow bounds, deterministic witnesses, repeated "
           "queries/prepares, chunk tails, empty plans, visited counts and "
           "prefix bucket/duplicate/infinity boundaries, bounded partial-sum "
           "reuse, uncached fallback and exact compact point keys\n";
  } catch (const std::exception &e) {
    std::cerr << e.what() << '\n';
    return 1;
  }
}
