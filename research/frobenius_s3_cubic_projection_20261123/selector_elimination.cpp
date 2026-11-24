// Exact rank test for a selector-only consequence violated at a given sum.
#include <algorithm>
#include <bit>
#include <chrono>
#include <cstdint>
#include <iostream>
#include <queue>
#include <tuple>
#include <unordered_map>
#include <vector>

using Word = std::uint64_t;
using Row = std::vector<Word>;
using Clock = std::chrono::steady_clock;

void subsets(std::vector<Word>& result, unsigned variables, unsigned degree, unsigned start = 0,
             Word prefix = 0) {
    if (!degree) {
        result.push_back(prefix);
        return;
    }
    for (unsigned i = start; i + degree <= variables; ++i) {
        subsets(result, variables, degree - 1, i + 1, prefix | (Word{1} << i));
    }
}

int main() {
    unsigned k, degree, closure, neq, time_limit;
    Word witness;
    if (!(std::cin >> k >> degree >> closure >> neq >> witness >> time_limit) ||
        !k || k >= 32 || degree < 2 || degree > 4 || closure > 1) {
        return 1;
    }
    auto start = Clock::now();
    Word xmask = (Word{1} << k) - 1;
    std::vector<Word> monomials;
    for (unsigned d = 0; d <= degree; ++d) {
        subsets(monomials, 2*k, d);
    }
    std::stable_sort(monomials.begin(), monomials.end(), [&](Word a, Word b) {
        return std::make_pair(std::popcount(a), std::popcount(a & xmask)) <
               std::make_pair(std::popcount(b), std::popcount(b & xmask));
    });
    std::unordered_map<Word, unsigned> index;
    unsigned columns = monomials.size();
    const unsigned row_width = (columns + 63) / 64;
    Row positive(row_width), evaluation(row_width);
    unsigned lower = 0;
    for (unsigned i = 0; i < columns; ++i) {
        Word m = monomials[i];
        index[m] = i;
        lower += static_cast<unsigned>(std::popcount(m)) < degree;
        if (m & xmask) {
            positive[i / 64] |= Word{1} << (i % 64);
        } else if ((m >> k & witness) == (m >> k)) {
            evaluation[i / 64] |= Word{1} << (i % 64);
        }
    }
    std::vector<Word> multipliers;
    for (unsigned d = 0; d <= degree - 2; ++d) {
        subsets(multipliers, 2*k, d);
    }
    std::vector<std::vector<Word>> equations(neq);
    for (auto& eq : equations) {
        unsigned terms;
        std::cin >> terms;
        eq.resize(terms);
        for (Word& m : eq) {
            std::cin >> m;
        }
    }
    std::uint64_t xors=0, rank=0, attempted=0, words=0, iterations=0;
    bool rejected=false, timeout=false;
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
                        queue.emplace(std::popcount(monomials[lead]), weight, lead);
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
    for (Word m : multipliers) {
        if (elapsed() > time_limit) { timeout = true; break; }
        for (const auto& eq : equations) {
            std::fill(row.begin(), row.end(), 0);
            int top = 0;
            for (Word term : eq) {
                unsigned i = index.at(m | term);
                row[i/64] ^= Word{1} << (i%64);
                top = std::max(top, static_cast<int>(i / 64));
            }
            if ((rejected = insert(top))) break;
        }
        if (rejected) break;
    }
    const auto raw_count = attempted;
    const auto raw_rank = rank;
    std::vector<unsigned> products(closure ? 2*k*lower : 0);
    for (unsigned v = 0; closure && v < 2*k; ++v) {
        for (unsigned j = 0; j < lower; ++j) {
            products[v*lower+j] = index.at(monomials[j] | (Word{1}<<v));
        }
    }
    std::vector<unsigned> support;
    while (!queue.empty() && !rejected && !timeout) {
        if (elapsed() > time_limit) { timeout = true; break; }
        ++iterations;
        auto i=std::get<2>(queue.top());
        queue.pop();
        support.clear();
        for (unsigned j=0; j<pivots[i].size(); ++j) {
            for (Word bits=pivots[i][j]; bits; bits&=bits-1) {
                support.push_back(j*64+std::countr_zero(bits));
            }
        }
        for (unsigned v=0; v<2*k; ++v) {
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
    for (unsigned i=0; i<columns && !rejected; ++i) {
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
                        terms.push_back(monomials[64 * w + std::countr_zero(bits)] >> k);
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
              << ",\"iterations\":" << iterations << ",\"attempts\":" << attempted << ",\"row_xors\":" << xors
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
    std::cout << "]}\n";
}
