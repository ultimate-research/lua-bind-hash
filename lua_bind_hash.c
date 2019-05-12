#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include <inttypes.h>

#define SHUFIV 0x27d4eb2f165667c4
#define HASHIV 0x60ea27eeadc0b5d5

#define DELTA1 -0x61c8864e7a143579
#define DELTA2 -0x3d4d51c2d82b14b1
#define DELTA3 0x85ebca77c2b2ae63
#define DELTA4 0x165667b19e3779f9
#define DELTA5 -0x784349ab80000000
#define DELTA6 -0x210ca4fef0869357
#define DELTA7 0xc2b2ae3d27d4eb4e
#define DELTA8 0x61c8864e7a143578
#define DELTA9 0x1bbcd8c2f5e54380
#define DELTA10 0x3c6ef3630bd7950e
#define DELTA11 0x779b185ebca87000
#define DELTA12 -0x1939e850d5e40000
#define DELTA13 -0x7a1435883d4d519d
#define DELTA14 0x6c158a5880000000

#define SHUFFLE1(a, b, c, d) (a * b | c >> d)
#define SHUFFLE2(a, b, c) a * b | a * c >> 0x21
#define SHUFFLE3(a) DELTA1 + DELTA3 ^ (SHUFFLE2(a, DELTA5, DELTA6)) * DELTA1

#define PTRTO(x) (int64_t *)((int64_t)x)

uint64_t lua_bind_hash(void *data_, uint64_t len) {
	int64_t *data = data_;	
	int64_t *dataEnd = PTRTO(data + len);
	
	uint64_t hash;

	int64_t shuffled = SHUFIV;
	
	if (len > 0x1f)
	{
		uint64_t temp[7];

		temp[4] = DELTA7;
		temp[1] = DELTA8;
		temp[2] = -1;

		hash = HASHIV;

		do 
		{
			hash = hash + data[0] * DELTA2;
			temp[0] = hash >> 0x21 | hash << 0x1F;
			temp[4] = temp[4] + data[1] * DELTA2;
			temp[2] = temp[2] + data[2] * DELTA2;
			temp[1] = temp[1] + data[3] * DELTA2;
			temp[5] = temp[4] >> 0x21 | temp[4] << 0x1F;
			temp[4] = temp[5] * DELTA1;
			temp[6] = temp[2] >> 0x21 | temp[2] << 0x1F;
			temp[3] = temp[1] >> 0x21 | temp[1] << 0x1F;
			hash = temp[0] * DELTA1;
			temp[2] = temp[6] * DELTA1;
			temp[1] = temp[3] * DELTA1;
			data += 4;
		}
		while (data <= dataEnd - 4);
		
		shuffled = 
		((((SHUFFLE1(temp[5], DELTA9, temp[4], 0x39) + SHUFFLE1(temp[0], DELTA10, hash, 0x3f) + 
			SHUFFLE1(temp[6], DELTA11, temp[2], 0x34) + SHUFFLE1(temp[3], DELTA12, temp[1], 0x2e) ^ 
			(SHUFFLE2(temp[0], DELTA5, DELTA6)) * DELTA1) * SHUFFLE3(temp[5])) * 
			SHUFFLE3(temp[6])) * SHUFFLE3(temp[3])) * DELTA1 + DELTA13;
	}
	
	int64_t *dataPtr = data + 1;
	
	len += shuffled;
	
	while (dataPtr <= dataEnd)
	{
		dataPtr = data + 2;
		len = (SHUFFLE2(*data, DELTA14, DELTA2)) * DELTA1 ^ len;
		len = (len >> 0x25 | len << 0x1b) * DELTA1 + DELTA3;
		data++;
	}
	
	if (PTRTO(data + 4) <= dataEnd)
	{
		len = *(uint32_t *)data * DELTA1 ^ len;
		len = (len >> 0x29 | len << 0x17) * DELTA2 + DELTA4;
		data = PTRTO(data + 4);
	}
	
	if (data < dataEnd)
	{
		do
		{
			dataPtr = PTRTO(data + 1);
			len = *(uint8_t *)data * (SHUFIV + 1) ^ len;
			len = (len >> 0x35 | len << 0xb) * DELTA1;
			data = dataPtr;
		}
		while (dataEnd != dataPtr);
	}
	
	hash = (len ^ len >> 0x21) * DELTA2;
	hash = (hash ^ hash >> 0x1d) * DELTA4;

	return hash ^ hash >> 0x20;
}

uint64_t lua_bind_hash_str(char *str)
{
	return lua_bind_hash(str, strlen(str));
}

int main()
{
	printf("Hash of 'password' = %" PRIx64 "\n", lua_bind_hash_str("password"));
	return 0;
}
