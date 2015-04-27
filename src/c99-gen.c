#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>
#include <limits.h>


#define INT_TO_RUST(iu, T) \
	printf("pub type %s = %s%u;\n", #T, #iu, sizeof(T)*CHAR_BIT)

#define CONST_TO_RUST(C, T, pspec) \
	printf("pub const %s: %s = %"pspec";\n", #C, #T, C)


int main(int argc, char const *argv[])
{
	INT_TO_RUST(i, int_fast8_t);
	INT_TO_RUST(i, int_fast16_t);
	INT_TO_RUST(i, int_fast32_t);
	INT_TO_RUST(i, int_fast64_t);

	INT_TO_RUST(i, int_least8_t);
	INT_TO_RUST(i, int_least16_t);
	INT_TO_RUST(i, int_least32_t);
	INT_TO_RUST(i, int_least64_t);

	INT_TO_RUST(i, intmax_t);
	INT_TO_RUST(i, intptr_t);

	INT_TO_RUST(u, uint_fast8_t);
	INT_TO_RUST(u, uint_fast16_t);
	INT_TO_RUST(u, uint_fast32_t);
	INT_TO_RUST(u, uint_fast64_t);

	INT_TO_RUST(u, uint_least8_t);
	INT_TO_RUST(u, uint_least16_t);
	INT_TO_RUST(u, uint_least32_t);
	INT_TO_RUST(u, uint_least64_t);

	INT_TO_RUST(u, uintmax_t);
	INT_TO_RUST(u, uintptr_t);

	CONST_TO_RUST(INT8_MIN, int8_t, PRId8);
	CONST_TO_RUST(INT16_MIN, int16_t, PRId16);
	CONST_TO_RUST(INT32_MIN, int32_t, PRId32);
	CONST_TO_RUST(INT64_MIN, int64_t, PRId64);
	CONST_TO_RUST(INT_FAST8_MIN, int_fast8_t, PRIdFAST8);
	CONST_TO_RUST(INT_FAST16_MIN, int_fast16_t, PRIdFAST16);
	CONST_TO_RUST(INT_FAST32_MIN, int_fast32_t, PRIdFAST32);
	CONST_TO_RUST(INT_FAST64_MIN, int_fast64_t, PRIdFAST64);
	CONST_TO_RUST(INT_LEAST8_MIN, int_least8_t, PRIdLEAST8);
	CONST_TO_RUST(INT_LEAST16_MIN, int_least16_t, PRIdLEAST16);
	CONST_TO_RUST(INT_LEAST32_MIN, int_least32_t, PRIdLEAST32);
	CONST_TO_RUST(INT_LEAST64_MIN, int_least64_t, PRIdLEAST64);
	CONST_TO_RUST(INTPTR_MIN, intptr_t, PRIdPTR);
	CONST_TO_RUST(INTMAX_MIN, intmax_t, PRIdMAX);

	CONST_TO_RUST(INT8_MAX, int8_t, PRId8);
	CONST_TO_RUST(INT16_MAX, int16_t, PRId16);
	CONST_TO_RUST(INT32_MAX, int32_t, PRId32);
	CONST_TO_RUST(INT64_MAX, int64_t, PRId64);
	CONST_TO_RUST(INT_FAST8_MAX, int_fast8_t, PRIdFAST8);
	CONST_TO_RUST(INT_FAST16_MAX, int_fast16_t, PRIdFAST16);
	CONST_TO_RUST(INT_FAST32_MAX, int_fast32_t, PRIdFAST32);
	CONST_TO_RUST(INT_FAST64_MAX, int_fast64_t, PRIdFAST64);
	CONST_TO_RUST(INT_LEAST8_MAX, int_least8_t, PRIdLEAST8);
	CONST_TO_RUST(INT_LEAST16_MAX, int_least16_t, PRIdLEAST16);
	CONST_TO_RUST(INT_LEAST32_MAX, int_least32_t, PRIdLEAST32);
	CONST_TO_RUST(INT_LEAST64_MAX, int_least64_t, PRIdLEAST64);
	CONST_TO_RUST(INTPTR_MAX, intptr_t, PRIdPTR);
	CONST_TO_RUST(INTMAX_MAX, intmax_t, PRIdMAX);

	CONST_TO_RUST(UINT8_MAX, uint8_t, PRIu8);
	CONST_TO_RUST(UINT16_MAX, uint16_t, PRIu16);
	CONST_TO_RUST(UINT32_MAX, uint32_t, PRIu32);
	CONST_TO_RUST(UINT64_MAX, uint64_t, PRIu64);
	CONST_TO_RUST(UINT_FAST8_MAX, uint_fast8_t, PRIuFAST8);
	CONST_TO_RUST(UINT_FAST16_MAX, uint_fast16_t, PRIuFAST16);
	CONST_TO_RUST(UINT_FAST32_MAX, uint_fast32_t, PRIuFAST32);
	CONST_TO_RUST(UINT_FAST64_MAX, uint_fast64_t, PRIuFAST64);
	CONST_TO_RUST(UINT_LEAST8_MAX, uint_least8_t, PRIuLEAST8);
	CONST_TO_RUST(UINT_LEAST16_MAX, uint_least16_t, PRIuLEAST16);
	CONST_TO_RUST(UINT_LEAST32_MAX, uint_least32_t, PRIuLEAST32);
	CONST_TO_RUST(UINT_LEAST64_MAX, uint_least64_t, PRIuLEAST64);
	CONST_TO_RUST(UINTPTR_MAX, uintptr_t, PRIuPTR);
	CONST_TO_RUST(UINTMAX_MAX, uintmax_t, PRIuMAX);

	return 0;
}
