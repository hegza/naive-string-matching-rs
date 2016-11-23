__kernel void test(
			__private int const coeff,
			__global int const* const input,
			__global int* const result)
{
	uint const idx = get_global_id(0);
	result[idx] = input[idx] * coeff;
}

