// Exact rank test for a selector-only consequence violated at a given sum.
#include <algorithm>
#include <bit>
#include <chrono>
#include <cstdint>
#include <iostream>
#include <queue>
#include <string>
#include <string_view>
#include <tuple>
#include <unordered_map>
#include <vector>

using Word = std::uint64_t;
using Row = std::vector<Word>;
using Monomial = unsigned __int128;

unsigned degree_of(Monomial m) {
    return std::popcount(static_cast<Word>(m)) + std::popcount(static_cast<Word>(m >> 64));
}

struct Hash {
    std::size_t operator()(Monomial m) const {
        return std::hash<Word>{}(static_cast<Word>(m)) ^
               std::rotl(std::hash<Word>{}(static_cast<Word>(m >> 64)), 21);
    }
};
using Clock = std::chrono::steady_clock;

void subsets(std::vector<Monomial>& result, unsigned variables, unsigned degree, unsigned start = 0,
             Monomial prefix = 0) {
    if (!degree) {
        result.push_back(prefix);
        return;
    }
    for (unsigned i = start; i + degree <= variables; ++i) {
        subsets(result, variables, degree - 1, i + 1, prefix | (Monomial{1} << i));
    }
}

int main(int argc, char** argv) {
    const bool omit_repeated_products = argc == 1;
    if (!omit_repeated_products && (argc != 2 || std::string_view(argv[1]) != "--repeat-products")) return 1;
    unsigned k, right, degree, closure, neq, time_limit, check_count;
    Word witness;
    if (!(std::cin >> k >> right >> degree >> closure >> neq >> witness >> time_limit >> check_count) ||
        !k || k >= 128 || right >= 64 || right >= 128 - k || degree < 2 || degree > 4 || closure > 1) {
        return 1;
    }
    auto start = Clock::now();
    const unsigned variables = k + right;
    Monomial xmask = (Monomial{1} << k) - 1;
    std::vector<Monomial> monomials;
    for (unsigned d = 0; d <= degree; ++d) {
        subsets(monomials, variables, d);
    }
    std::stable_sort(monomials.begin(), monomials.end(), [&](Monomial a, Monomial b) {
        return std::make_pair(degree_of(a), degree_of(a & xmask)) <
               std::make_pair(degree_of(b), degree_of(b & xmask));
    });
    std::unordered_map<Monomial, unsigned, Hash> index;
    unsigned columns = monomials.size();
    const unsigned row_width = (columns + 63) / 64;
    Row positive(row_width), evaluation(row_width);
    unsigned lower = 0;
    for (unsigned i = 0; i < columns; ++i) {
        Monomial m = monomials[i];
        index[m] = i;
        lower += static_cast<unsigned>(degree_of(m)) < degree;
        if (m & xmask) {
            positive[i / 64] |= Word{1} << (i % 64);
        } else if ((m >> k & witness) == (m >> k)) {
            evaluation[i / 64] |= Word{1} << (i % 64);
        }
    }
    std::vector<Monomial> multipliers;
    for (unsigned d = 0; d <= degree - 2; ++d) {
        subsets(multipliers, variables, d);
    }
    auto read_equation = [&](std::vector<Monomial>& eq) {
        unsigned terms;
        if (!(std::cin >> terms)) return false;
        eq.resize(terms);
        for (Monomial& m : eq) {
            std::string digits;
            std::cin >> digits;
            if (digits.empty()) return false;
            for (char ch : digits) {
                if (ch < '0' || ch > '9') return false;
                m = m * 10 + ch - '0';
            }
            if (!index.contains(m)) return false;
        }
        return true;
    };
    std::vector<std::vector<Monomial>> equations(neq), checks(check_count);
    for (auto& equation : equations) {
        if (!read_equation(equation) || std::any_of(equation.begin(), equation.end(), [](Monomial m) { return degree_of(m) > 2; })) return 1;
    }
    for (auto& check : checks) if (!read_equation(check)) return 1;
    auto has_polynomial = [&](const std::vector<Monomial>& terms, const std::vector<Row>& pivots) {
        Row row(row_width);
        for (Monomial monomial : terms) {
            unsigned index_value = index.at(monomial);
            row[index_value / 64] ^= Word{1} << (index_value % 64);
        }
        for (int top = row_width - 1; top >= 0; --top) {
            while (row[top]) {
                unsigned lead = 64 * top + std::bit_width(row[top]) - 1;
                const auto& pivot = pivots[lead];
                if (pivot.empty()) return false;
                for (int w = 0; w <= top; ++w) row[w] ^= pivot[w];
            }
        }
        return true;
    };
    std::uint64_t xors=0, rank=0, attempted=0, words=0, iterations=0;
    bool rejected=false, timeout=false;
    std::uint64_t repeated_products_skipped = 0;
    std::vector<Row> processed(closure && omit_repeated_products ? lower : 0);
    auto remember_products = [&](Row& value) {
        for (int top = static_cast<int>(value.size()) - 1; top >= 0; --top) {
            while (value[top]) {
                unsigned leading = 64 * top + std::bit_width(value[top]) - 1;
                auto& previous = processed[leading];
                if (previous.empty()) {
                    previous.assign(value.begin(), value.begin() + top + 1);
                    value.resize(top + 1);
                    return true;
                }
                for (int j = 0; j <= top; ++j) value[j] ^= previous[j];
            }
        }
        return false;
    };
    std::vector<Row> pivots(columns);
    Row row(row_width);
    using Job = std::tuple<unsigned, unsigned, unsigned>;
    std::priority_queue<Job, std::vector<Job>, std::greater<Job>> queue;
    auto insert = [&](int top) {
        ++attempted;
        for (; top >= 0; --top) {
            while (row[top]) {
                unsigned lead = 64*top + std::bit_width(row[top]) - 1;
                auto& pivot = pivots[lead];
                if (pivot.empty()) {
                    pivot.assign(row.begin(), row.begin()+top+1);
                    ++rank;
                    if (closure && lead < lower) {
                        unsigned weight = 0;
                        for (Word w : pivot) weight += std::popcount(w);
                        queue.emplace(degree_of(monomials[lead]), weight, lead);
                    }
                    return lead == 0;
                }
                for (int w = 0; w <= top; ++w) {
                    row[w] ^= pivot[w];
                }
                ++xors;
                words += top+1;
            }
        }
        return false;
    };
    auto elapsed = [&]{ return std::chrono::duration<double>(Clock::now()-start).count(); };
    for (Monomial m : multipliers) {
        if (elapsed() > time_limit) { timeout = true; break; }
        for (const auto& eq : equations) {
            std::fill(row.begin(), row.end(), 0);
            int top = 0;
            for (Monomial term : eq) {
                unsigned i = index.at(m | term);
                row[i/64] ^= Word{1} << (i%64);
                top = std::max(top, static_cast<int>(i / 64));
            }
            if (!processed.empty() && degree_of(m) + 2 < degree) {
                // Every variable multiple already occurs in the raw matrix.
                Row lower_source(row.begin(), row.begin() + top + 1);
                remember_products(lower_source);
            }
            if ((rejected = insert(top))) break;
        }
        if (rejected) break;
    }
    const auto raw_count = attempted;
    const auto raw_rank = rank;
    std::vector<unsigned> products(closure ? variables*lower : 0);
    for (unsigned v = 0; closure && v < variables; ++v) {
        for (unsigned j = 0; j < lower; ++j) {
            products[v*lower+j] = index.at(monomials[j] | (Monomial{1}<<v));
        }
    }
    std::vector<unsigned> support;
    while (!queue.empty() && !rejected && !timeout) {
        if (!checks.empty() && iterations % 128 == 0 &&
            std::all_of(checks.begin(), checks.end(), [&](const auto& poly){ return has_polynomial(poly, pivots); })) break;
        if (elapsed() > time_limit) { timeout = true; break; }
        ++iterations;
        auto i=std::get<2>(queue.top());
        queue.pop();
        Row source = pivots[i];
        if (!processed.empty() && !remember_products(source)) {
            ++repeated_products_skipped;
            continue;
        }
        support.clear();
        for (unsigned j = 0; j < source.size(); ++j) {
            for (Word bits = source[j]; bits; bits &= bits-1) {
                support.push_back(j*64+std::countr_zero(bits));
            }
        }
        for (unsigned v=0; v<variables; ++v) {
            std::fill(row.begin(),row.end(),0);
            int top=0;
            for (unsigned m : support) {
                unsigned j=products[v*lower+m];
                row[j/64]^=Word{1}<<(j%64);
                top=std::max(top,static_cast<int>(j/64));
            }
            if ((rejected=insert(top))) break;
        }
    }
    // Mixed leading pivots stay independent under projection. Only pure
    // leading pivots can create a dependency after dropping pure columns.
    std::unordered_map<unsigned, Row> projected;
    unsigned pure = 0, selector_rank = 0;
    std::vector<std::vector<Word>> selector_rows;
    for (unsigned i=0; i<columns && !rejected && checks.empty(); ++i) {
        if ((monomials[i] & xmask) || pivots[i].empty()) continue;
        ++pure;
        row=pivots[i];
        int top=row.size()-1;
        bool has_mixed=false;
        for (; top>=0 && !has_mixed; --top) {
            while (row[top]&positive[top]) {
                unsigned lead=64*top+std::bit_width(row[top]&positive[top])-1;
                const Row* pivot=&pivots[lead];
                if (pivot->empty()) {
                    auto hit=projected.find(lead);
                    if (hit==projected.end()) {
                        projected.emplace(lead,row);
                        has_mixed=true;
                        break;
                    }
                    pivot=&hit->second;
                }
                for (unsigned w=0; w<pivot->size(); ++w) row[w]^=(*pivot)[w];
            }
        }
        if (!has_mixed) {
            ++selector_rank;
            if (degree == 3) {
                auto& terms = selector_rows.emplace_back();
                for (unsigned w = 0; w < row.size(); ++w) {
                    for (Word bits = row[w]; bits; bits &= bits - 1) {
                        terms.push_back(static_cast<Word>(monomials[64 * w + std::countr_zero(bits)] >> k));
                    }
                }
            }
            unsigned parity=0;
            for (unsigned w=0; w<row.size(); ++w) parity^=std::popcount(row[w]&evaluation[w]);
            rejected=(parity&1)!=0;
        }
    }
    std::cout << "{\"rejected\":" << (rejected ? "true" : "false")
              << ",\"timeout\":" << (timeout ? "true" : "false") << ",\"columns\":" << columns
              << ",\"rank\":" << rank << ",\"raw_rank\":" << raw_rank << ",\"raw_rows\":" << raw_count
              << ",\"pure_pivots\":" << pure << ",\"selector_rank\":" << selector_rank
              << ",\"iterations\":" << iterations << ",\"skipped\":" << repeated_products_skipped
              << ",\"attempts\":" << attempted << ",\"row_xors\":" << xors
              << ",\"word_xors\":" << words << ",\"seconds\":" << std::chrono::duration<double>(Clock::now()-start).count()
              << ",\"selector_rows\":[";
    for (unsigned i = 0; i < selector_rows.size(); ++i) {
        if (i) std::cout << ',';
        std::cout << '[';
        for (unsigned j = 0; j < selector_rows[i].size(); ++j) {
            if (j) std::cout << ',';
            std::cout << selector_rows[i][j];
        }
        std::cout << ']';
    }
    std::cout << "],\"members\":[";
    for (unsigned i = 0; i < checks.size(); ++i) {
        if (i) std::cout << ',';
        std::cout << (has_polynomial(checks[i], pivots) ? "true" : "false");
    }
    std::cout << "]}\n";
}
