using System;
using System.Text;

class LuaBindHash {
    unsafe static ulong Hash(byte[] data_) {
        long *data;
        fixed (byte *data__ = data_)
        {
            data = (long*)data__;
        }
        ulong len = (ulong)data_.Length;
        long *plVar1;
        long *plVar2;
        ulong uVar3;
        ulong uVar4;
        long lVar5;
        ulong uVar6;
        ulong uVar7;
        ulong uVar8;
        ulong uVar9;
        ulong uVar10;
        ulong uVar11;
        
        lVar5 = 0x27d4eb2f165667c4;
        plVar1 = (long *)((ulong)data + len);
        if (0x1f < len) {
            uVar9 = 0xc2b2ae3d27d4eb4e;
            uVar6 = 0x61c8864e7a143578;
            uVar7 = 0xffffffffffffffff;
            uVar3 = 0x60ea27eeadc0b5d5;
            do {
                uVar3 = (ulong)((long)uVar3 + *data * -0x3d4d51c2d82b14b1);
                uVar4 = uVar3 >> 0x21 | uVar3 * 0x80000000;
                uVar9 += (ulong)(data[1] * -0x3d4d51c2d82b14b1);
                uVar7 += (ulong)(data[2] * -0x3d4d51c2d82b14b1);
                uVar6 += (ulong)(data[3] * -0x3d4d51c2d82b14b1);
                uVar10 = uVar9 >> 0x21 | uVar9 * 0x80000000;
                uVar9 = (ulong)((long)uVar10 * -0x61c8864e7a143579);
                uVar11 = uVar7 >> 0x21 | uVar7 * 0x80000000;
                uVar8 = uVar6 >> 0x21 | uVar6 * 0x80000000;
                uVar3 = (ulong)((long)uVar4 * -0x61c8864e7a143579);
                uVar7 = (ulong)((long)uVar11 * -0x61c8864e7a143579);
                uVar6 = (ulong)((long)uVar8 * -0x61c8864e7a143579);
                data = data + 4;
            } while (data <= plVar1 + -4);
            lVar5 = (long)((((((uVar10 * 0x1bbcd8c2f5e54380 | uVar9 >> 0x39) +
                        (uVar4 * 0x3c6ef3630bd7950e | uVar3 >> 0x3f) +
                        (uVar11 * 0x779b185ebca87000 | uVar7 >> 0x34) +
                        (uVar8 * 0xe6c617af2a1c0000 | uVar6 >> 0x2e) ^
                        (uVar4 * 0x87bcb65480000000 | uVar4 * 0xdef35b010f796ca9 >> 0x21) *
                        0x9e3779b185ebca87) * 0x9e3779b185ebca87 + 0x85ebca77c2b2ae63 ^
                        (uVar10 * 0x87bcb65480000000 | uVar10 * 0xdef35b010f796ca9 >> 0x21) *
                        0x9e3779b185ebca87) * 0x9e3779b185ebca87 + 0x85ebca77c2b2ae63 ^
                        (uVar11 * 0x87bcb65480000000 | uVar11 * 0xdef35b010f796ca9 >> 0x21) *
                        0x9e3779b185ebca87) * 0x9e3779b185ebca87 + 0x85ebca77c2b2ae63 ^
                        (uVar8 * 0x87bcb65480000000 | uVar8 * 0xdef35b010f796ca9 >> 0x21) *
                        0x9e3779b185ebca87) * 0x9e3779b185ebca87 + 0x85ebca77c2b2ae63);
        }
        plVar2 = data + 1;
        len = (ulong)lVar5 + len;
        while (plVar2 <= plVar1) {
            plVar2 = data + 2;
            len = (ulong)((long)((ulong)((long)*data * -0x6c158a5880000000) | (ulong)((long)*data * -0x3d4d51c2d82b14b1) >> 0x21) *
                        -0x61c8864e7a143579) ^ len;
            len = (ulong)((long)(len >> 0x25 | len << 0x1b) * -0x61c8864e7a143579) + 0x85ebca77c2b2ae63;
            data = data + 1;
        }
        if ((long *)((long)data + 4U) <= plVar1) {
            len = (ulong)((long)((ulong)*(ulong*)data) * -0x61c8864e7a143579) ^ len;
            len = (ulong)((long)(len >> 0x29 | len << 0x17) * -0x3d4d51c2d82b14b1) + 0x165667b19e3779f9;
            data = (long *)((long)data + 4U);
        }
        if (data < plVar1) {
            do {
                plVar2 = (long *)((long)data + 1);
                len = (ulong)*(byte *)data * 0x27d4eb2f165667c5 ^ len;
                len = (ulong)((long)(len >> 0x35 | len << 0xb) * -0x61c8864e7a143579);
                data = plVar2;
            } while (plVar1 != plVar2);
        }
        uVar3 = (ulong)((long)(len ^ len >> 0x21) * -0x3d4d51c2d82b14b1);
        uVar3 = (uVar3 ^ uVar3 >> 0x1d) * 0x165667b19e3779f9;
        return uVar3 ^ uVar3 >> 0x20;
    }

    static ulong HashString(string s) {
        return Hash(Encoding.UTF8.GetBytes(s));
    }

    static void Main() {
        // Should print A4D50A730E36970E
        Console.WriteLine(HashString("password1234").ToString("X"));
    }
}

