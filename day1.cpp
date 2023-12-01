#include <fstream>
#include <vector>
#include <iostream>
#include <string>
#include <array>

int main() {
    int sum = 0;

    const std::array<std::string, 9> word_digits {
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
    };

    std::ifstream input_file("day1-input.txt");

    while (true) {
        std::string line;
        std::getline(input_file, line);

        if (input_file.eof()) {
            break;
        }

        std::vector<int> digits;

        std::cout << "line --- " << line << '\n';

        for (unsigned int i = 0; i < line.size(); i++) {
            const auto& c = line[i];

            if (std::isdigit(c)) {
                digits.push_back(c - '0');
                std::cout << "\tfound " << digits.back() << '\n';
                continue;
            }

            for (unsigned int j = 0; j < word_digits.size(); j++) {
                const auto& digit = word_digits[j];

                if (digit.size() + i <= line.size()) {
                    const auto substr = line.substr(i, digit.size());
                    if (substr == digit) {
                        digits.push_back(j + 1);
                        std::cout << "\tfound " << digits.back() << '\n';
                    }
                }
            }
        }

        int result = digits.front() * 10 + digits.back();
        sum += result;
    }

    std::cout << sum << '\n';
}
