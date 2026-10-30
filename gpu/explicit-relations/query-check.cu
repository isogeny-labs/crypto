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
                            const Point *queries, U count, U *answers, unsigned shift = 6) {
  U i = U(blockIdx.x) * BLOCK + threadIdx.x;
  if (i < count)
    answers[i] = indexed_lookup(rows, size, queries[i], prefixes, shift);
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
template <class TableRow, class Less>
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
        for (U id : {U(9), U(2), U(UINT32_MAX)})
          full.push_back({point, id});
      else if (x % 17 == 0)
        full.push_back({point, U(UINT32_MAX)});
    }
  }
  std::sort(full.begin(), full.end(), RowLess{});
  std::vector<TableRow> rows;
  for (const auto &row : full)
    rows.push_back({point_key(row.point), row.id});
  std::sort(rows.begin(), rows.end(), Less{});
  thrust::device_vector<TableRow> device_rows(rows.begin(), rows.end());
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
  require(point_key(Point{(U(1) << 63) - 2, 1, false}) == UINT64_MAX);
  // The old 62-bit prefix endpoint is representable (not a curve point).
  require(point_key(Point{U(1) << 62, 0, false}) == (U(1) << 63) + 2);
}
// Exercise the largest new offset array with sparse exact keys, long empty
// ranges, endpoint buckets and duplicates. Compare every offset on CPU;
// query both row layouts independently against the original full lookup.
static void wide_prefix_boundaries(unsigned bits) {
  const auto layout = prefix_layout((U(1) << bits) - 1, U(1) << 31);
#ifdef ER_WIDE_INDEX
  require(layout.bins == (U(1) << 24) && layout.shift == bits - 24);
#else
  require(layout.bins == (U(1) << 16) && layout.shift == bits - 16);
#endif
  require(prefix_layout(65521, 4095).bins == 0);
  require(prefix_layout(65521, 4096).bins == 128);
  const U bins = U(1) << 24;
  const unsigned shift = bits - 24; // terminal offset does not encode the endpoint
  std::vector<Row> full{{Point{}, 7}, {Point{}, 3}};
  std::vector<Point> queries{Point{}};
  for (U bucket : {U(0), U(1), U(65535), U(65536), bins / 2, bins - 1}) {
    for (U delta : {U(0), U(1), (U(1) << shift) - 1}) {
      U x = (bucket << shift) + delta;
      if (x >= (U(1) << 63) - 1) continue; // outside every supported prime field
      for (U parity : {U(0), U(1)}) {
        Point q{x, parity, false};
        queries.push_back(q);
        if (delta != 1)
          for (U id : {U(9), U(2)})
            full.push_back({q, id});
      }
    }
  }
  // Synthetic comparator fixtures only; compact/full parity keys agree by
  // construction. Actual on-curve compact identities are exhausted above.
  std::sort(full.begin(), full.end(), RowLess{});
  std::vector<DeviceRow> rows;
  for (const auto &row : full)
    rows.push_back(device_row(row.point, row.id));
  std::sort(rows.begin(), rows.end(), DeviceRowLess{});
  thrust::device_vector<DeviceRow> device_rows(rows.begin(), rows.end());
  thrust::device_vector<Point> device_queries(queries.begin(), queries.end());
  thrust::device_vector<U> prefixes(bins + 1), answers(queries.size());
  build_prefix<<<(bins + 1 + BLOCK - 1) / BLOCK, BLOCK>>>(
      data(device_rows), rows.size(), bins, shift, data(prefixes));
  cuda_check(cudaGetLastError());
  std::vector<U> offsets(bins + 1);
  cuda_check(cudaMemcpy(offsets.data(), data(prefixes), offsets.size() * sizeof(U),
                        cudaMemcpyDeviceToHost));
  U position = 0;
  for (U bucket = 0; bucket <= bins; ++bucket) {
    Point key{bucket << shift, 0, false};
    while (position < full.size() && row_less_key(full[position], key))
      ++position;
    require(offsets[bucket] == position);
  }
  check_index<<<1, BLOCK>>>(data(device_rows), rows.size(), data(prefixes),
      data(device_queries), queries.size(), data(answers), shift);
  cuda_check(cudaGetLastError());
  std::vector<U> got(queries.size());
  cuda_check(cudaMemcpy(got.data(), data(answers), got.size() * sizeof(U),
                        cudaMemcpyDeviceToHost));
  for (size_t i = 0; i < queries.size(); ++i)
    require(got[i] == lookup(full.data(), full.size(), queries[i]));
}
__global__ void field_boundary_kernel(Fp64Ctx f, const U *pairs, U *out, U n) {
  U i = U(blockIdx.x) * blockDim.x + threadIdx.x;
  if (i >= n) return;
  U a = fp64_to_mont(f, pairs[2*i]), b = fp64_to_mont(f, pairs[2*i+1]);
  out[3*i] = fp64_from_mont(f, fp64_mul(f,a,b));
  out[3*i+1] = fp64_from_mont(f, fp64_add(f,a,b));
  out[3*i+2] = fp64_from_mont(f, fp64_sub(f,a,b));
}
static void field_boundary_checks() {
  U seed = 0x123456789abcdefULL;
  auto next = [&]() { seed ^= seed << 13; seed ^= seed >> 7; seed ^= seed << 17; return seed; };
  for (U p : {(U(1)<<62)-57, (U(1)<<63)-25}) {
    while (!prime(p)) p-=2;
    Curve c = curve(p,p-3,7);
    std::vector<U> pairs;
    for (U a : {U(0),U(1),p/2,p-2,p-1})
      for (U b : {U(0),U(1),p/2,p-2,p-1}) { pairs.push_back(a); pairs.push_back(b); }
    for (unsigned i=0;i<4096;++i) { pairs.push_back(next()%p); pairs.push_back(next()%p); }
    U n=pairs.size()/2;
    thrust::device_vector<U> input(pairs.begin(),pairs.end()), output(3*n);
    field_boundary_kernel<<<(n+BLOCK-1)/BLOCK,BLOCK>>>(c.f,data(input),data(output),n);
    cuda_check(cudaGetLastError());
    std::vector<U> got(3*n);
    cuda_check(cudaMemcpy(got.data(),data(output),got.size()*sizeof(U),cudaMemcpyDeviceToHost));
    for(U i=0;i<n;++i) {
      U a=pairs[2*i],b=pairs[2*i+1];
      require(got[3*i]==modmul(a,b,p));
      require(got[3*i+1]==U(((unsigned __int128)a+b)%p));
      require(got[3*i+2]==U(((unsigned __int128)a+p-b)%p));
    }
  }
}
#ifdef ER_NEGATIVE_FILTER
__global__ void check_filtered(const DeviceRow *rows, U size,
                                const Point *queries, U count, U *answers,
                                NegativeFilter filter) {
  U i = U(blockIdx.x) * BLOCK + threadIdx.x;
  RightPartials partials;
  partials.filter = filter;
  if (i < count)
    answers[i] = relation_lookup(rows, size, queries[i], nullptr, 0, partials);
}
static void negative_filter_boundaries() {
  Curve c = curve(65521, 0, 7);
  std::vector<Row> full{{Point{}, 9}, {Point{}, 2}};
  std::vector<Point> queries{Point{}};
  for (U x = 0; x < c.f.p; ++x) {
    Point q;
    if (!c.lift(x, 0, q)) continue;
    for (Point point : {q, c.neg(q)}) {
      queries.push_back(point);
      if (x % 13 == 0)
        for (U id : {U(9), U(2), U(UINT32_MAX)}) full.push_back({point, id});
    }
  }
  std::sort(full.begin(), full.end(), RowLess{});
  std::vector<DeviceRow> rows;
  for (const auto &r : full) rows.push_back(device_row(r.point, r.id));
  std::sort(rows.begin(), rows.end(), DeviceRowLess{});
  thrust::device_vector<DeviceRow> drows(rows.begin(), rows.end());
  thrust::device_vector<Point> dqueries(queries.begin(), queries.end());
  thrust::device_vector<U> answers(queries.size());
  const U words = filter_word_count(rows.size());
  thrust::device_vector<unsigned long long> bits(words, 0);
  build_negative_filter<<<(rows.size() + BLOCK - 1) / BLOCK, BLOCK>>>(
      data(drows), rows.size(), data(bits), words - 1);
  cuda_check(cudaGetLastError());
  for (unsigned mode = 0; mode < 3; ++mode) {
    if (mode == 1)
      cuda_check(cudaMemsetAsync(data(bits), 0xff, words * sizeof(U)));
    NegativeFilter filter{mode == 2 ? nullptr : data(bits), words - 1};
    check_filtered<<<(queries.size() + BLOCK - 1) / BLOCK, BLOCK>>>(
        data(drows), rows.size(), data(dqueries), queries.size(), data(answers), filter);
    cuda_check(cudaGetLastError());
    std::vector<U> got(queries.size());
    cuda_check(cudaMemcpy(got.data(), data(answers), got.size() * sizeof(U),
                          cudaMemcpyDeviceToHost));
    U misses = 0;
    for (size_t i = 0; i < queries.size(); ++i) {
      U expected = lookup(full.data(), full.size(), queries[i]);
      require(got[i] == expected);
      misses += expected == UINT64_MAX;
    }
    require(misses > 0); // Saturation forces false positives through exact lookup.
  }
}
#endif
int main() {
  try {
    field_boundary_checks();
#ifdef ER_CUB_SORT
    require(merge_sort_bytes(0) == 0);
    for (U count : {(U(1) << 31) - 1, U(1) << 31,
                    (U(1) << 31) + 1, U(1) << 32})
      require(merge_sort_bytes(count) >= count * sizeof(DeviceRow));
#endif
    require(right_cache_fits(40, 128, 1024, 640));
    require(!right_cache_fits(41, 128, 1024, UINT64_MAX));
    require(!right_cache_fits(40, 128, 1024, 639));
    require(!right_cache_fits(1, 769, 1024, UINT64_MAX));
    require(!right_cache_fits(UINT64_MAX, 0, UINT64_MAX, UINT64_MAX));
    require(!right_cache_fits(0, 0, 1024, UINT64_MAX));
    for (Point p : {Point{}, Point{0, 0, false},
                    Point{(U(1) << 62) - 2, (U(1) << 62) - 3, false}})
      require(equal(CachedPoint::from(p).point(), p));
    prefix_boundaries();
    compact_boundaries<CompactRow, CompactRowLess>();
    compact_boundaries<DenseRow, DenseRowLess>();
    require(DenseRow((U(1) << 63) + 1, UINT32_MAX).key() == (U(1) << 63) + 1);
    require(U(DenseRow(0, UINT32_MAX).id) != UINT64_MAX);
    wide_prefix_boundaries(62);
    wide_prefix_boundaries(63);
#ifdef ER_NEGATIVE_FILTER
    negative_filter_boundaries();
#endif
    require(!packed_winner_fits(0, 1));
    require(packed_winner_fits(1, UINT64_MAX));
    require(packed_winner_fits(UINT64_MAX, 1));
    require(!packed_winner_fits(UINT64_MAX, 2));
    require(packed_winner_fits(U(1) << 32, (U(1) << 32) - 1));
    require(!packed_winner_fits(U(1) << 32, U(1) << 32));
    Curve c = curve(97, 0, 7);
#ifdef ER_DENSE_ROWS
    DeviceBackend guarded(c, 1, 1);
    Plan oversized;
    oversized.left = (U(1) << 32) + 1;
    bool rejected = false;
    try { guarded.prepare(oversized); }
    catch (const std::runtime_error &e) {
      rejected = std::string(e.what()) == "LIMIT dense tuple ID capacity";
    }
    require(rejected);
    guarded.prepare(Plan{});
    require(!guarded.find(Point{}, 0).found);
#endif
    std::vector<Point> ps;
    for (U x = 0; x < 97; ++x) {
      Point q;
      if (c.lift(x, 0, q))
        ps.push_back(q);
    }
    // Exercise every enlarged slot boundary with exact CPU witnesses, tails,
    // empty reprepare and the same device storage used by each build variant.
    for (unsigned slots : {9u, 10u, MAX_SLOTS}) {
      std::vector<std::vector<Point>> bases(slots, {ps[0], ps[1]});
      Plan p = plan(c, bases, 4096);
      require(p.slots == slots && p.offsets[slots] == 2 * slots);
      DeviceBackend device(c, 4096, 17);
      HostBackend host(c, 4096, 17);
      device.prepare(p); host.prepare(p);
      for (Point target : {Point{}, ps[0], ps[1], ps.back()}) {
        auto got = device.find(target, 8192), expected = host.find(target, 8192);
        require(got.found == expected.found && got.indices == expected.indices);
        if (got.found) require(got.indices.size() == slots);
      }
      bases.back().clear();
      device.prepare(plan(c, bases, 4096));
      require(!device.find(Point{}, 0).found);
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
#ifdef ER_QUERY_PERSISTENT
    // Force multiple grid strides, a partial final tile, late minima, complete
    // misses and reset ordering. The check target caps the grid at two blocks.
    {
      std::vector<std::vector<Point>> bases(3);
      bases[0] = {base.front()};
      bases[1].assign(base.begin(), base.begin() + 33);
      bases[2].assign(base.begin() + 32, base.end());
      Plan p = plan(wide, bases, 65);
      // A valid deliberately unbalanced split exercises long right traversals.
      p.split = 1; p.left = 1; p.right = 33 * 33;
      require(p.left == 1 && p.right == 1089 && p.split == 1);
      DeviceBackend device(wide, 65, 1);
      HostBackend host(wide, 65, 1);
      device.prepare(p);
      host.prepare(p);
#ifdef ER_RIGHT_FULL_CACHE
      require(device.cached_right_rows() == p.right);
      DeviceBackend fallback(wide, 65, 1, 0);
      fallback.prepare(p);
      require(fallback.cached_right_rows() == 0);
#endif
      Point late, absent;
      bool have_late = false, have_absent = false;
      for (U i = p.right; i-- > 512;) {
        Point target = wide.affine(tuple_sum(wide, p.view(p.points.data()),
                                            0, p.slots, i));
        auto expected = host.find(target, p.left + p.right);
        require(expected.found);
        U right = expected.indices[1] * 33 + expected.indices[2];
        if (right > 512) { late = target; have_late = true; break; }
      }
      for (Point target : base) {
        if (!host.find(target, p.left + p.right).found) {
          absent = target; have_absent = true; break;
        }
      }
      require(have_late && have_absent);
      for (unsigned repeat = 0; repeat < 8; ++repeat) {
        for (Point target : {late, absent, late}) {
          auto got = device.find(target, p.left + p.right);
          auto expected = host.find(target, p.left + p.right);
          require(got.found == expected.found && got.indices == expected.indices);
#ifdef ER_RIGHT_FULL_CACHE
          auto uncached = fallback.find(target, p.left + p.right);
          require(uncached.found == got.found && uncached.indices == got.indices);
          if (!uncached.found)
            require(uncached.visited == p.right);
#endif
          if (got.found) {
            U right = expected.indices[1] * 33 + expected.indices[2];
            require(right > 512 && got.visited >= right + 1 && got.visited <= p.right);
          } else {
            require(got.visited == p.right);
          }
        }
      }
      device.prepare(Plan{});
      require(device.cached_right_rows() == 0);
      require(!device.find(Point{}, 0).found);
      device.prepare(p);
#ifdef ER_RIGHT_FULL_CACHE
      require(device.cached_right_rows() == p.right);
#endif
      require(device.find(late, p.left + p.right).indices ==
              host.find(late, p.left + p.right).indices);
    }
#endif
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
#ifdef ER_NEGATIVE_FILTER
        require(device.negative_filter_bytes() == filter_word_count(p.left) * sizeof(U));
#endif
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
#ifdef ER_QUERY_PERSISTENT
            require(got.visited >= right + 1 && got.visited <= p.right);
            visited = got.visited; // Exact work varies with block scheduling.
#else
            visited = std::min(p.right, (right / chunk + 1) * chunk);
#endif
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
#ifdef ER_QUERY_PERSISTENT
            require(got.visited >= right + 1 && got.visited <= p.right);
            visited = got.visited; // Exact work varies with block scheduling.
#else
            visited = std::min(p.right, (right / chunk + 1) * chunk);
#endif
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
