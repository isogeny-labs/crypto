#include <cstdint>
#include <fstream>
#include <iostream>
#include <initializer_list>
#include <sstream>
#include <string>
#include <vector>

struct Bin { std::uint64_t score, count, tail; };

int main(int argc, char **argv) {
  if (argc != 3) {
    std::cerr << "usage: allocation_distribution INPUT.tsv OUTPUT.svg\n";
    return 2;
  }
  std::ifstream input(argv[1]);
  if (!input) { std::cerr << "cannot open input TSV\n"; return 1; }
  std::string line;
  if (!std::getline(input, line) || line != "score_twice\tallocations\ttail_at_least") {
    std::cerr << "unexpected TSV header\n"; return 1;
  }
  std::vector<Bin> bins;
  std::uint64_t total = 0;
  while (std::getline(input, line)) {
    const auto first = line.find('\t');
    const auto second = first == std::string::npos ? first : line.find('\t', first + 1);
    if (first == std::string::npos || second == std::string::npos || line.find('\t', second + 1) != std::string::npos) {
      std::cerr << "malformed TSV row\n"; return 1;
    }
    Bin b{};
    try {
      b.score = std::stoull(line.substr(0, first));
      b.count = std::stoull(line.substr(first + 1, second - first - 1));
      b.tail = std::stoull(line.substr(second + 1));
    } catch (...) { std::cerr << "nonnumeric TSV field\n"; return 1; }
    if (b.score != bins.size()) { std::cerr << "noncontiguous score bins\n"; return 1; }
    total += b.count;
    bins.push_back(b);
  }
  if (bins.size() != 205 || total != 20037600) {
    std::cerr << "unexpected bin count or allocation total\n"; return 1;
  }
  std::uint64_t suffix = 0;
  for (std::size_t i = bins.size(); i-- > 0;) {
    suffix += bins[i].count;
    if (bins[i].tail != suffix) { std::cerr << "tail column mismatch\n"; return 1; }
  }
  constexpr std::size_t threshold = 117;
  if (bins[threshold].tail != 5869345) {
    std::cerr << "observed threshold tail mismatch\n"; return 1;
  }

  constexpr double left = 88, top = 58, plot_w = 790, plot_h = 330;
  const double max_count = 320000.0;
  const double step = plot_w / bins.size();
  std::ofstream svg(argv[2]);
  if (!svg) { std::cerr << "cannot write SVG\n"; return 1; }
  svg << "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1000\" height=\"510\" viewBox=\"0 0 1000 510\">\n"
      << "<rect width=\"100%\" height=\"100%\" fill=\"white\"/>\n"
      << "<style>text{font-family:Arial,sans-serif;fill:#182230}.axis{stroke:#344054;stroke-width:1.5}.grid{stroke:#d0d5dd;stroke-width:1}.tick{font-size:13px}.label{font-size:16px}.title{font-size:21px;font-weight:bold}.note{font-size:14px}</style>\n"
      << "<text class=\"title\" x=\"500\" y=\"27\" text-anchor=\"middle\">Exact labelled allocation distribution</text>\n";
  for (int t = 0; t <= 3; ++t) {
    const double y = top + plot_h - (t * 100000.0 / max_count) * plot_h;
    svg << "<line class=\"grid\" x1=\"" << left << "\" y1=\"" << y << "\" x2=\"" << left + plot_w << "\" y2=\"" << y << "\"/>\n"
        << "<text class=\"tick\" x=\"" << left - 10 << "\" y=\"" << y + 4 << "\" text-anchor=\"end\">" << t * 100 << "k</text>\n";
  }
  for (const int score : {0, 50, 100, 150, 204}) {
    const double x = left + score * step + step / 2;
    svg << "<line class=\"grid\" x1=\"" << x << "\" y1=\"" << top << "\" x2=\"" << x << "\" y2=\"" << top + plot_h << "\"/>\n"
        << "<text class=\"tick\" x=\"" << x << "\" y=\"" << top + plot_h + 22 << "\" text-anchor=\"middle\">" << score << "</text>\n";
  }
  for (const auto &b : bins) {
    const double h = b.count / max_count * plot_h;
    const double x = left + b.score * step;
    svg << "<rect x=\"" << x << "\" y=\"" << top + plot_h - h << "\" width=\"" << step + .15 << "\" height=\"" << h << "\" fill=\"" << (b.score >= threshold ? "#93c5fd" : "#475467") << "\"/>\n";
  }
  const double tx = left + threshold * step + step / 2;
  svg << "<line x1=\"" << tx << "\" y1=\"" << top - 3 << "\" x2=\"" << tx << "\" y2=\"" << top + plot_h << "\" stroke=\"#b42318\" stroke-width=\"2.5\"/>\n"
      << "<text x=\"" << tx + 7 << "\" y=\"" << top + 18 << "\" font-size=\"14\" font-weight=\"bold\" fill=\"#b42318\">observed 2U = 117</text>\n"
      << "<line class=\"axis\" x1=\"" << left << "\" y1=\"" << top + plot_h << "\" x2=\"" << left + plot_w << "\" y2=\"" << top + plot_h << "\"/>\n"
      << "<line class=\"axis\" x1=\"" << left << "\" y1=\"" << top << "\" x2=\"" << left << "\" y2=\"" << top + plot_h << "\"/>\n"
      << "<text class=\"label\" x=\"" << left + plot_w/2 << "\" y=\"" << top + plot_h + 48 << "\" text-anchor=\"middle\">Doubled U score (2U)</text>\n"
      << "<text class=\"label\" transform=\"translate(23 " << top + plot_h/2 << ") rotate(-90)\" text-anchor=\"middle\">Labelled assignments per score</text>\n"
      << "<rect x=\"670\" y=\"435\" width=\"12\" height=\"12\" fill=\"#93c5fd\"/><text class=\"note\" x=\"688\" y=\"446\">2U ≥ 117: 5,869,345 / 20,037,600</text>\n"
      << "<text class=\"note\" x=\"88\" y=\"478\">Exact finite allocation counts; one bin per integer 2U score. Source: committed distribution.tsv.</text>\n</svg>\n";
  if (!svg) { std::cerr << "failed writing SVG\n"; return 1; }
  std::cout << "validated " << bins.size() << " bins, " << total << " assignments; threshold tail " << bins[threshold].tail << "\n";
}
