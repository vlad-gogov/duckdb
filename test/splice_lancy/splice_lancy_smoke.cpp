#include <cstdint>
#include <iostream>

struct SmokeResult {
	int32_t status;
	int32_t value;
};

extern "C" SmokeResult duckdb_splice_lancy_hello(int32_t bias, int32_t input);

int main() {
	const auto result = duckdb_splice_lancy_hello(40, 2);
	if (result.status != 0) {
		return 1;
	}
	if (result.value != 42) {
		std::cerr << "Expected 42 from Splice/Lancy JIT, got " << result.value << '\n';
		return 1;
	}
	std::cout << "Hello from Splice/Lancy JIT: 40 + 2 = " << result.value << '\n';
	return 0;
}
