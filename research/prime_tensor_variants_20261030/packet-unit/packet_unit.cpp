// Fixed exact finite-field development checks for a supplied ternary packet gadget.
// No contraction evaluator, precision theorem, performance result or attack.
#include <algorithm>
#include <array>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <map>
#include <set>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

using Label = std::vector<int>;
using Pair = std::pair<int, int>;
using Count = std::int64_t;

int integer_power(int base, int exponent) {
    int result = 1;
    for (int i = 0; i < exponent; ++i) result *= base;
    return result;
}
int normalized(int x, int modulus) {
    const int r = x % modulus; return r < 0 ? r + modulus : r;
}
Pair raw_pair(Pair p, int modulus, bool inverse) {
    return inverse ? Pair{p.second, normalized(-p.first - p.second, modulus)}
                   : Pair{normalized(-p.first - p.second, modulus), p.first};
}
struct Field {
    int r;
    explicit Field(int characteristic) : r(characteristic) {
        if (r < 2 || r == 3) throw std::runtime_error("unsupported characteristic");
        for (int d = 2; d * d <= r; ++d)
            if (r % d == 0) throw std::runtime_error("unit supports prime fields only");
    }
    Pair action(Pair p, bool inverse) const { return raw_pair(p, r, inverse); }
};
struct Counts {
    Count assertions = 0, full_classes = 0, orbits = 0, slot_pairs = 0;
    Count locality_checks = 0, tile_checks = 0, two_target_tiles = 0;
    Count graph_edges = 0, graph_packets = 0, margin_rows = 0;
    Count high_field_pairs = 0, high_field_locality = 0, negatives = 0;
    std::array<Count, 3> high_target_sizes{};
    void require(bool condition, const std::string& message) {
        ++assertions; if (!condition) throw std::runtime_error(message);
    }
};
int encode(const Label& label, int r) {
    int value = 0, place = 1;
    for (const int x : label) { value += x * place; place *= r; }
    return value;
}
Label decode(int value, int r, int a) {
    Label label(static_cast<std::size_t>(4 * a));
    for (auto& x : label) { x = value % r; value /= r; }
    return label;
}
std::string text(const Label& label) {
    std::string result;
    for (const int x : label) {
        if (!result.empty()) result += ',';
        result += std::to_string(x);
    }
    return result;
}
std::string text(const std::set<int>& values) {
    if (values.empty()) return "empty";
    std::string result;
    for (const int x : values) {
        if (!result.empty()) result += ',';
        result += std::to_string(x);
    }
    return result;
}
Label apply_level(Label label, const Field& field, int i, bool inverse = false,
                  bool wrong_second_direction = false) {
    const std::size_t k = static_cast<std::size_t>(4 * i);
    const Pair one = field.action({label[k], label[k + 2]}, inverse);
    const Pair two = field.action({label[k + 1], label[k + 3]},
                                  wrong_second_direction ? inverse : !inverse);
    label[k] = one.first; label[k + 2] = one.second;
    label[k + 1] = two.first; label[k + 3] = two.second;
    return label;
}
std::vector<Label> packet(const Label& gamma, const Field& field, int a,
                          bool wrong_second_direction = false) {
    const int L = integer_power(3, a);
    std::vector<Label> result; result.reserve(static_cast<std::size_t>(L));
    for (int x = 0; x < L; ++x) {
        Label label = gamma; int digits = x;
        for (int i = 0; i < a; ++i) {
            const int digit = digits % 3; digits /= 3;
            for (int j = 0; j < digit; ++j)
                label = apply_level(label, field, i, false, wrong_second_direction);
        }
        result.push_back(label);
    }
    return result;
}
int active_levels(const Label& label, int a) {
    int k = 0;
    for (int i = 0; i < a; ++i) {
        bool nonzero = false;
        for (int j = 0; j < 4; ++j)
            nonzero = nonzero || label[static_cast<std::size_t>(4 * i + j)] != 0;
        if (nonzero) ++k;
    }
    return k;
}
std::pair<int, int> difference_level(int difference) {
    if (difference == 0) throw std::runtime_error("diagonal has no finite 3-adic level");
    int i = 0;
    while (difference % 3 == 0) { difference /= 3; ++i; }
    return {i, normalized(difference, 3)};
}
bool local(const std::vector<Label>& p, int x, int y) {
    const auto level = difference_level(y - x);
    const std::size_t row = static_cast<std::size_t>(4 * level.first + level.second - 1);
    return p[static_cast<std::size_t>(x)][row]
        == p[static_cast<std::size_t>(y)][row + 2];
}
bool valid_padding(Count H, int L) { return H > 0 && H % L == 0; }

struct Outputs {
    std::ofstream classes, orbits, margins, graphs, tiles, high, negative;
    explicit Outputs(const std::filesystem::path& path)
        : classes(path / "classes.tsv"), orbits(path / "orbits.tsv"),
          margins(path / "marginals.tsv"), graphs(path / "graphs.tsv"),
          tiles(path / "tiles.tsv"), high(path / "field67.tsv"),
          negative(path / "negative.tsv") {
        if (!classes || !orbits || !margins || !graphs || !tiles || !high || !negative)
            throw std::runtime_error("cannot open output records");
        classes << "panel\tr\ta\tclass\tlabel\tactive_levels\torbit_size\tslot_pairs\tlocality_checks\ttiles\tassigned_edges\tstatus\n";
        orbits << "panel\tr\ta\torbit\trepresentative\tactive_levels\torbit_size\tslots_per_class\tpacket_copies\tassigned_edges\tstatus\n";
        margins << "panel\tr\ta\tside\tlevel\tcomponent\tsymbol\tactual\texpected\tstatus\n";
        graphs << "panel\tr\ta\tDarr\tH\tM\tDelta\tpackets\tmax_row_degree\tmax_column_degree\tunused_slots\tcovered_edges\tstatus\n";
        tiles << "panel\tr\ta\tclass\tlevel\tcomponent\ty0\tsource_group\tcross_targets\ttarget_count\tstatus\n";
        high << "a\tb\torbit_size\tsource_group\tcross_targets\ttarget_count\tlocality_checks\tstatus\n";
        negative << "control\twitness\tobservation\tstatus\n";
    }
    void finish() {
        for (auto* stream : {&classes, &orbits, &margins, &graphs, &tiles, &high, &negative}) {
            stream->flush(); if (!*stream) throw std::runtime_error("record write failed");
            stream->close(); if (stream->fail()) throw std::runtime_error("record close failed");
        }
    }
};

void full_panel(int panel, int r, int a, int expected_orbits, Count expected_H,
                Count expected_M, Count expected_packets, Outputs& out, Counts& count) {
    const Field field(r);
    const int L = integer_power(3, a), Darr = integer_power(r, 4 * a);
    const Count H = ((2LL * Darr + L - 1) / L) * L;
    const Count M = H * Darr;
    count.require(H == expected_H && M == expected_M, "fixed matching graph size mismatch");
    count.require(valid_padding(H, L), "invalid H padding");
    count.require(M >= 2LL * Darr * Darr && M >= static_cast<Count>(L) * Darr,
                  "matching graph load inequalities failed");
    std::vector<std::vector<int>> orbit_classes;
    std::vector<int> orbit_of(static_cast<std::size_t>(Darr), -1);
    for (int id = 0; id < Darr; ++id) {
        if (orbit_of[static_cast<std::size_t>(id)] != -1) continue;
        const Label gamma = decode(id, r, a);
        const auto p = packet(gamma, field, a);
        std::map<int, int> frequencies;
        for (const auto& label : p) ++frequencies[encode(label, r)];
        const int expected_size = integer_power(3, active_levels(gamma, a));
        count.require(static_cast<int>(frequencies.size()) == expected_size,
                      "orbit size differs from 3^active");
        std::vector<int> members;
        const int orbit_id = static_cast<int>(orbit_classes.size());
        for (const auto& item : frequencies) {
            count.require(item.second == L / expected_size, "packet orbit padding not uniform");
            count.require(orbit_of[static_cast<std::size_t>(item.first)] == -1,
                          "action orbits overlap");
            orbit_of[static_cast<std::size_t>(item.first)] = orbit_id;
            members.push_back(item.first);
            const Label label = decode(item.first, r, a);
            for (int i = 0; i < a; ++i) {
                const int next = encode(apply_level(label, field, i), r);
                count.require(frequencies.count(next) == 1, "orbit not action closed");
            }
        }
        count.require(members.front() == id, "orbit representative not minimal");
        orbit_classes.push_back(members);
    }
    count.require(static_cast<int>(orbit_classes.size()) == expected_orbits,
                  "fixed orbit count mismatch");
    count.orbits += expected_orbits;

    std::vector<Count> assigned(static_cast<std::size_t>(Darr), 0);
    std::vector<std::uint8_t> seen(static_cast<std::size_t>(M), 0);
    std::vector<std::uint8_t> row_degree(static_cast<std::size_t>(M), 0),
                              column_degree(static_cast<std::size_t>(M), 0);
    std::vector<Count> row_margins(static_cast<std::size_t>(2 * a * r), 0),
                       column_margins(static_cast<std::size_t>(2 * a * r), 0);
    Count packet_count = 0, coverage = 0;
    for (std::size_t oi = 0; oi < orbit_classes.size(); ++oi) {
        const auto& members = orbit_classes[oi];
        const int size = static_cast<int>(members.size());
        const Label gamma = decode(members.front(), r, a);
        const auto p = packet(gamma, field, a);
        count.require((H * size) % L == 0, "nonintegral packet copy count");
        const Count copies = H * size / L;
        for (Count copy = 0; copy < copies; ++copy) {
            ++packet_count;
            for (const auto& label : p) {
                const int cls = encode(label, r);
                const Count edge_copy = assigned[static_cast<std::size_t>(cls)]++;
                count.require(edge_copy < H, "class assigned too many edges");
                const Count edge_id = static_cast<Count>(cls) * H + edge_copy;
                count.require(edge_id >= 0 && edge_id < M, "edge ID out of range");
                const auto index = static_cast<std::size_t>(edge_id);
                count.require(seen[index] == 0, "duplicate padded edge");
                seen[index] = 1; ++coverage;
                ++row_degree[index]; ++column_degree[index];
                for (int i = 0; i < a; ++i) for (int c = 0; c < 2; ++c) {
                    const std::size_t k = static_cast<std::size_t>(4 * i + c);
                    ++row_margins[static_cast<std::size_t>((2 * i + c) * r + label[k])];
                    ++column_margins[static_cast<std::size_t>((2 * i + c) * r + label[k + 2])];
                }
            }
        }
        out.orbits << panel << '\t' << r << '\t' << a << '\t' << oi << '\t'
                   << members.front() << '\t' << active_levels(gamma, a) << '\t'
                   << size << '\t' << L / size << '\t' << copies << '\t'
                   << copies * L << "\tPASS\n";
    }
    for (Count edge = 0; edge < M; ++edge) {
        const auto k = static_cast<std::size_t>(edge);
        count.require(seen[k] == 1 && row_degree[k] == 1 && column_degree[k] == 1,
                      "matching port coverage/degree failed");
    }
    count.require(coverage == M && packet_count * L == M && packet_count == expected_packets,
                  "packet/edge exact coverage mismatch");
    count.graph_edges += coverage; count.graph_packets += packet_count;
    for (int i = 0; i < a; ++i) for (int c = 0; c < 2; ++c) for (int value = 0; value < r; ++value) {
        const std::size_t index = static_cast<std::size_t>((2 * i + c) * r + value);
        for (const int side : {0, 1}) {
            const Count actual = side == 0 ? row_margins[index] : column_margins[index];
            count.require(actual == M / r && M % r == 0, "nonuniform actual port marginal");
            ++count.margin_rows;
            out.margins << panel << '\t' << r << '\t' << a << '\t'
                        << (side == 0 ? "row" : "column") << '\t' << i << '\t'
                        << c + 1 << '\t' << value << '\t' << actual << '\t' << M / r
                        << "\tPASS\n";
        }
    }
    out.graphs << panel << '\t' << r << '\t' << a << '\t' << Darr << '\t'
               << H << '\t' << M << "\t1\t" << packet_count << "\t1\t1\t0\t"
               << coverage << "\tPASS\n";

    for (int id = 0; id < Darr; ++id) {
        ++count.full_classes;
        const Label gamma = decode(id, r, a);
        const auto p = packet(gamma, field, a);
        for (int i = 0; i < a; ++i) {
            Label triple = gamma;
            for (int j = 0; j < 3; ++j) triple = apply_level(triple, field, i);
            count.require(triple == gamma, "U_i cubed not identity");
            count.require(apply_level(apply_level(gamma, field, i), field, i, true) == gamma,
                          "U_i inverse failed");
            for (int j = i + 1; j < a; ++j)
                count.require(apply_level(apply_level(gamma, field, i), field, j)
                              == apply_level(apply_level(gamma, field, j), field, i),
                              "different-level U actions do not commute");
        }
        Count pair_count = 0, local_count = 0, tiles = 0;
        for (int x = 0; x < L; ++x) for (int y = 0; y < L; ++y) {
            ++pair_count; ++count.slot_pairs;
            if (x == y) continue;
            count.require(local(p, x, y), "ternary first-difference locality failed");
            ++local_count; ++count.locality_checks;
        }
        for (int i = 0; i < a; ++i) for (int c = 0; c < 2; ++c) {
            const int step = integer_power(3, i + 1);
            const std::size_t coordinate = static_cast<std::size_t>(4 * i + c);
            for (int y0 = 0; y0 < step; ++y0) {
                const int source = p[static_cast<std::size_t>(y0)][coordinate + 2];
                for (int y = y0; y < L; y += step)
                    count.require(p[static_cast<std::size_t>(y)][coordinate + 2] == source,
                                  "source tile group not constant");
                std::set<int> targets;
                for (int x = 0; x < L; ++x) {
                    const int target = p[static_cast<std::size_t>(x)][coordinate];
                    if (target != source) targets.insert(target);
                }
                count.require(targets.size() <= 2, "more than two crossing target groups");
                if (targets.size() == 2) ++count.two_target_tiles;
                ++tiles; ++count.tile_checks;
                out.tiles << panel << '\t' << r << '\t' << a << '\t' << id << '\t'
                          << i << '\t' << c + 1 << '\t' << y0 << '\t' << source << '\t'
                          << text(targets) << '\t' << targets.size() << "\tPASS\n";
            }
        }
        count.require(assigned[static_cast<std::size_t>(id)] == H, "wrong class edge load");
        const auto orbit_index = static_cast<std::size_t>(orbit_of[static_cast<std::size_t>(id)]);
        out.classes << panel << '\t' << r << '\t' << a << '\t' << id << '\t'
                    << text(gamma) << '\t' << active_levels(gamma, a) << '\t'
                    << orbit_classes[orbit_index].size() << '\t' << pair_count << '\t'
                    << local_count << '\t' << tiles << '\t'
                    << assigned[static_cast<std::size_t>(id)] << "\tPASS\n";
    }
}

Pair pair_power(Pair pair, const Field& field, int exponent, bool inverse) {
    for (int j = 0; j < exponent; ++j) pair = field.action(pair, inverse);
    return pair;
}
void high_field(Outputs& out, Counts& count) {
    const Field field(67);
    count.require(field.r > 48, "proof label constant not met by unit field");
    for (int a = 0; a < field.r; ++a) for (int b = 0; b < field.r; ++b) {
        const Pair original{a, b}; ++count.high_field_pairs;
        count.require(pair_power(original, field, 3, false) == original,
                      "S cubed failed in F67");
        count.require(pair_power(original, field, 3, true) == original,
                      "inverse S cubed failed in F67");
        count.require(field.action(field.action(original, false), true) == original,
                      "S inverse failed in F67");
        std::set<Pair> orbit;
        std::set<int> row_values;
        for (int j = 0; j < 3; ++j) {
            const Pair p = pair_power(original, field, j, false);
            orbit.insert(p); row_values.insert(p.first);
        }
        count.require(orbit.size() == static_cast<std::size_t>(a == 0 && b == 0 ? 1 : 3),
                      "nonzero pair orbit not size3 in F67");
        int local_count = 0;
        for (int x = 0; x < 3; ++x) for (int y = 0; y < 3; ++y) if (x != y) {
            const int c = normalized(y - x, 3);
            const bool inverse = c == 2;
            count.require(pair_power(original, field, x, inverse).first
                          == pair_power(original, field, y, inverse).second,
                          "selected-direction locality failed in F67");
            ++local_count; ++count.high_field_locality;
        }
        row_values.erase(b);
        count.require(row_values.size() <= 2, "F67 crossing target set too large");
        ++count.high_target_sizes[row_values.size()];
        out.high << a << '\t' << b << '\t' << orbit.size() << '\t' << b << '\t'
                 << text(row_values) << '\t' << row_values.size() << '\t' << local_count
                 << "\tPASS\n";
    }
    count.require(count.high_target_sizes == std::array<Count, 3>{1, 198, 4290},
                  "F67 distinct-triple counts mismatch");
}

void negatives(Outputs& out, Counts& count) {
    const Field f5(5);
    const Label wrong_witness{0, 1, 0, 2};
    const auto correct = packet(wrong_witness, f5, 1);
    const auto wrong = packet(wrong_witness, f5, 1, true);
    count.require(local(correct, 0, 2) && !local(wrong, 0, 2),
                  "wrong-second-direction control not detected");
    count.require(correct[0][1] == 1 && wrong[2][3] == 2,
                  "wrong direction witness values mismatch");
    ++count.negatives;
    out.negative << "wrong_second_direction\tr5,a1,gamma=0,1,0,2,x0,y2\tcorrect=1,wrong_column=2;rejected\tPASS\n";
    count.require(!valid_padding(32, 3), "nonmultiple H padding accepted");
    ++count.negatives;
    out.negative << "nonmultiple_padding\tr2,a1,H32,L3\trejected\tPASS\n";
    bool rejected = false;
    try { const Field unsupported(3); (void)unsupported; }
    catch (const std::runtime_error&) { rejected = true; }
    count.require(rejected && raw_pair({1, 1}, 3, false) == Pair{1, 1},
                  "char3 guard/fixed-point regression failed");
    ++count.negatives;
    out.negative << "unsupported_characteristic3\tS(1,1)=(1,1)mod3\tnonzero_fixed_pair;rejected\tPASS\n";
    const auto p = packet(Label{0, 0, 1, 0}, f5, 1);
    const int source = p[0][2];
    std::set<int> targets;
    for (const auto& label : p) if (label[0] != source) targets.insert(label[0]);
    count.require(source == 1 && targets == std::set<int>{0, 4},
                  "explicit two-target packet not obtained");
    count.require(!(targets.size() <= 1), "binary one-target guard unexpectedly passed");
    ++count.negatives;
    out.negative << "binary_one_target_assumption\tr5,a1,gamma=0,0,1,0,source1\tcross_targets=0,4;rejected\tPASS\n";
}

int main(int argc, char** argv) {
    try {
        if (argc != 2) throw std::runtime_error("usage: packet_unit OUTPUT_DIRECTORY");
        const std::filesystem::path path(argv[1]); std::filesystem::create_directories(path);
        Outputs out(path); Counts count;
        full_panel(1, 2, 1, 6, 33, 528, 176, out, count);
        full_panel(2, 2, 2, 36, 513, 131328, 14592, out, count);
        full_panel(3, 5, 1, 209, 1251, 781875, 260625, out, count);
        high_field(out, count); negatives(out, count);
        count.require(count.full_classes == 897 && count.orbits == 251
                      && count.slot_pairs == 26505 && count.locality_checks == 22278
                      && count.tile_checks == 9990 && count.graph_edges == 913731
                      && count.graph_packets == 275393 && count.high_field_pairs == 4489
                      && count.high_field_locality == 26934 && count.negatives == 4,
                      "frozen panel totals mismatch");
        out.finish();
        std::ofstream summary(path / "summary.json");
        summary << "{\n  \"schema\": \"ternary-packet-development/v1\",\n"
                << "  \"full_classes\": " << count.full_classes
                << ",\n  \"orbits\": " << count.orbits
                << ",\n  \"slot_pairs\": " << count.slot_pairs
                << ",\n  \"nonzero_difference_locality_checks\": " << count.locality_checks
                << ",\n  \"source_tiles\": " << count.tile_checks
                << ",\n  \"two_cross_target_tiles\": " << count.two_target_tiles
                << ",\n  \"matching_edges\": " << count.graph_edges
                << ",\n  \"matching_packets\": " << count.graph_packets
                << ",\n  \"marginal_rows\": " << count.margin_rows
                << ",\n  \"field67_pairs\": " << count.high_field_pairs
                << ",\n  \"field67_locality_checks\": " << count.high_field_locality
                << ",\n  \"field67_cross_target_sizes\": [1,198,4290],\n"
                << "  \"negative_controls\": " << count.negatives
                << ",\n  \"assertions\": " << count.assertions
                << ",\n  \"timing_or_scaling_result\": false,\n  \"status\": \"PASS\"\n}\n";
        summary.flush(); if (!summary) throw std::runtime_error("summary write failed");
        summary.close(); if (summary.fail()) throw std::runtime_error("summary close failed");
        std::cout << "PASS ternary full_classes=" << count.full_classes << " orbits=" << count.orbits
                  << " slot_pairs=" << count.slot_pairs << " locality=" << count.locality_checks
                  << " tiles=" << count.tile_checks << " graph_edges=" << count.graph_edges
                  << " graph_packets=" << count.graph_packets << '\n';
        std::cout << "PASS field67 pairs=4489 locality=26934 negative_controls=4; two targets retained; no timing/theorem claim\n";
        return 0;
    } catch (const std::exception& error) {
        std::cerr << "FAIL " << error.what() << '\n'; return 1;
    }
}
