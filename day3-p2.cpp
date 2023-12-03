#include <iostream>
#include <array>
#include <fstream>
#include <vector>
#include <cctype>

struct coords_t {
    size_t x;
    size_t y;
    bool invalid = false;
};

int main() {
    std::ifstream input("day3-input.txt");

    std::vector<std::vector<char>> grid;

    while (true) {
        std::string line;
        std::getline(input, line);

        if (input.eof()) {
            break;
        }

        std::vector<char> grid_line;
        for (char c : line) {
            grid_line.push_back(c);
        }

        grid.push_back(std::move(grid_line));
    }

    std::vector<coords_t> symbol_coords;

    for (size_t y = 0; y < grid.size(); y++) {
        for (size_t x = 0; x < grid[y].size(); x++) {
            if (grid[y][x] == '*') {
                symbol_coords.push_back(coords_t { x, y });
            }
        }
    }

    int sum = 0;

    std::cout << "symbols: ";
    for (const auto& coord : symbol_coords) {
        std::cout << grid[coord.y][coord.x] << ", ";
    }
    std::cout << '\n';

    for (const auto& coord : symbol_coords) {
        std::vector<coords_t> numbers;
        std::array<int, 3> search_offsets = { -1, 0, 1 };

        std::cout << "coord : " << coord.x << ", " << coord.y << '\n';

        for (int x_offset : search_offsets) {
            for (int y_offset : search_offsets) {
                if (coord.x == 0 && x_offset == -1) {
                    continue;
                }

                if (coord.y == 0 && y_offset == -1) {
                    continue;
                }

                if (coord.x + x_offset >= grid[0].size()) {
                    continue;
                }

                if (coord.y + y_offset >= grid[0].size()) {
                    continue;
                }

                if (std::isdigit(grid.at(coord.y + y_offset).at(coord.x + x_offset))) {
                    numbers.push_back({ coord.x + x_offset, coord.y + y_offset });
                }
            }
        }

        std::vector<int> ratios;

        std::cout << "For symbol " << grid[coord.y][coord.x] << ", numbers\n";
        for (const auto& number_c : numbers) {
            std::cout << grid[number_c.y][number_c.x];

            if (number_c.invalid) {
                continue;
            }

            auto search_c = number_c;

            std::string digits;

            while (search_c.x < grid[0].size() && std::isdigit(grid[search_c.y][search_c.x])) {
                digits.push_back(grid[search_c.y][search_c.x]);
                search_c.x++;

                for (auto& num : numbers) {
                    if (num.x == search_c.x && num.y == search_c.y) {
                        num.invalid = true;
                    }
                }
            }

            search_c = number_c;
            search_c.x--;

            while (search_c.x < grid[0].size() && std::isdigit(grid[search_c.y][search_c.x])) {
                digits.insert(digits.cbegin(), grid[search_c.y][search_c.x]);
                if (search_c.x == 0) {
                    break;
                }
                search_c.x--;
                for (auto& num : numbers) {
                    if (num.x == search_c.x && num.y == search_c.y) {
                        num.invalid = true;
                    }
                }
            }

            std::cout << " (part of number " << digits << "), ";

            int parsed = std::stoi(digits);
            ratios.push_back(parsed);
        }

        if (ratios.size() == 2) {
            sum += (ratios[0] * ratios[1]);
        }
        std::cout << "\n\n";
    }

    std::cout << "SUM: " << sum << "\n";

    return 0;
}
