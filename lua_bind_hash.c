#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include <inttypes.h>

uint64_t lua_bind_hash(void* data_, size_t len) {
    int64_t *data = data_;
    int64_t *plVar1;
    int64_t *plVar2;
    uint64_t uVar3;
    uint64_t uVar4;
    int64_t lVar5;
    uint64_t uVar6;
    uint64_t uVar7;
    uint64_t uVar8;
    uint64_t uVar9;
    uint64_t uVar10;
    uint64_t uVar11;
    
    lVar5 = 0x27d4eb2f165667c4;
    plVar1 = (int64_t *)((int64_t)data + len);
    if (0x1f < len) {
        uVar9 = 0xc2b2ae3d27d4eb4e;
        uVar6 = 0x61c8864e7a143578;
        uVar7 = 0xffffffffffffffff;
        uVar3 = 0x60ea27eeadc0b5d5;
        do {
            uVar3 = uVar3 + *data * -0x3d4d51c2d82b14b1;
            uVar4 = uVar3 >> 0x21 | uVar3 * 0x80000000;
            uVar9 = uVar9 + data[1] * -0x3d4d51c2d82b14b1;
            uVar7 = uVar7 + data[2] * -0x3d4d51c2d82b14b1;
            uVar6 = uVar6 + data[3] * -0x3d4d51c2d82b14b1;
            uVar10 = uVar9 >> 0x21 | uVar9 * 0x80000000;
            uVar9 = uVar10 * -0x61c8864e7a143579;
            uVar11 = uVar7 >> 0x21 | uVar7 * 0x80000000;
            uVar8 = uVar6 >> 0x21 | uVar6 * 0x80000000;
            uVar3 = uVar4 * -0x61c8864e7a143579;
            uVar7 = uVar11 * -0x61c8864e7a143579;
            uVar6 = uVar8 * -0x61c8864e7a143579;
            data = data + 4;
        } while (data <= plVar1 + -4);
        lVar5 = (((((uVar10 * 0x1bbcd8c2f5e54380 | uVar9 >> 0x39) +
                    (uVar4 * 0x3c6ef3630bd7950e | uVar3 >> 0x3f) +
                    (uVar11 * 0x779b185ebca87000 | uVar7 >> 0x34) +
                    (uVar8 * -0x1939e850d5e40000 | uVar6 >> 0x2e) ^
                    (uVar4 * -0x784349ab80000000 | uVar4 * -0x210ca4fef0869357 >> 0x21) *
                    -0x61c8864e7a143579) * -0x61c8864e7a143579 + 0x85ebca77c2b2ae63 ^
                    (uVar10 * -0x784349ab80000000 | uVar10 * -0x210ca4fef0869357 >> 0x21) *
                    -0x61c8864e7a143579) * -0x61c8864e7a143579 + 0x85ebca77c2b2ae63 ^
                    (uVar11 * -0x784349ab80000000 | uVar11 * -0x210ca4fef0869357 >> 0x21) *
                    -0x61c8864e7a143579) * -0x61c8864e7a143579 + 0x85ebca77c2b2ae63 ^
                    (uVar8 * -0x784349ab80000000 | uVar8 * -0x210ca4fef0869357 >> 0x21) *
                    -0x61c8864e7a143579) * -0x61c8864e7a143579 + -0x7a1435883d4d519d;
    }
    plVar2 = data + 1;
    len = lVar5 + len;
    while (plVar2 <= plVar1) {
        plVar2 = data + 2;
        len = (*data * -0x6c158a5880000000 | (uint64_t)(*data * -0x3d4d51c2d82b14b1) >> 0x21) *
                    -0x61c8864e7a143579 ^ len;
        len = (len >> 0x25 | len << 0x1b) * -0x61c8864e7a143579 + 0x85ebca77c2b2ae63;
        data = data + 1;
    }
    if ((int64_t *)((int64_t)data + 4U) <= plVar1) {
        len = (uint64_t)*(uint32_t *)data * -0x61c8864e7a143579 ^ len;
        len = (len >> 0x29 | len << 0x17) * -0x3d4d51c2d82b14b1 + 0x165667b19e3779f9;
        data = (int64_t *)((int64_t)data + 4U);
    }
    if (data < plVar1) {
        do {
            plVar2 = (int64_t *)((int64_t)data + 1);
            len = (uint64_t)*(uint8_t *)data * 0x27d4eb2f165667c5 ^ len;
            len = (len >> 0x35 | len << 0xb) * -0x61c8864e7a143579;
            data = plVar2;
        } while (plVar1 != plVar2);
    }
    uVar3 = (len ^ len >> 0x21) * -0x3d4d51c2d82b14b1;
    uVar3 = (uVar3 ^ uVar3 >> 0x1d) * 0x165667b19e3779f9;
    return uVar3 ^ uVar3 >> 0x20;
}

uint64_t lua_bind_hash_str(char* str) {
    return lua_bind_hash(str, strlen(str));
}

int main() {
    printf("Hash of 'password' = %" PRIx64 "\n", lua_bind_hash_str("password"));
    return 0;
}
