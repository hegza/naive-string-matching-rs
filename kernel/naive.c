__kernel void naive_sm(
		__private int const m,
		__global char const* const text,
		__global char const* const pattern,
		__global int* const result)
{
	uint const idx = get_global_id(0);

	for (int i = idx; i != idx + m; ++i) {
		result[idx] += (text[i] != pattern[i-idx]);
	}
}
