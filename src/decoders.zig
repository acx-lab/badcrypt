const std = @import("std");
const testing = std.testing;

pub fn single_byte_xor(decrypted: []u8, b: []const u8) void {
    var key: ?usize = undefined;
    var max_score: f32 = 1.0;
    for (0..255) |lower| {
        @memcpy(decrypted, b);
        const l: u8 = @intCast(lower);
        for (decrypted, 0..decrypted.len) |d, i| {
            decrypted[i] = d ^ @as(u8, l);
        }
        const sc_lower = score(decrypted);
        if (sc_lower > max_score) {
            max_score = sc_lower;
            key = lower;
        }
    }

    if (key) |k| {
        for (0..decrypted.len) |i| {
            decrypted[i] = b[i] ^ @as(u8, @intCast(k));
        }
    }
}

pub fn score(attempt: []const u8) f32 {
    var s: f32 = 1;
    for (attempt) |c| {
        switch (c) {
            'a' | 'A' => s *= 8.12,
            'b' | 'B' => s *= 1.49,
            'c' | 'C' => s *= 2.71,
            'd' | 'D' => s *= 4.32,
            'e' | 'E' => s *= 12.02,
            'f' | 'F' => s *= 2.30,
            'g' | 'G' => s *= 2.03,
            'h' | 'H' => s *= 5.92,
            'i' | 'I' => s *= 7.31,
            'j' | 'J' => s *= 0.10,
            'k' | 'K' => s *= 0.69,
            'l' | 'L' => s *= 3.98,
            'm' | 'M' => s *= 2.61,
            'n' | 'N' => s *= 6.95,
            'o' | 'O' => s *= 7.68,
            'p' | 'P' => s *= 1.82,
            'q' | 'Q' => s *= 0.11,
            'r' | 'R' => s *= 6.02,
            's' | 'S' => s *= 6.28,
            't' | 'T' => s *= 9.10,
            'u' | 'U' => s *= 2.88,
            'v' | 'V' => s *= 1.11,
            'w' | 'W' => s *= 2.09,
            'x' | 'X' => s *= 0.17,
            'y' | 'Y' => s *= 2.11,
            'z' | 'Z' => s *= 0.07,
            ' ' => s *= @as(f32, @floatCast(' ')),
            else => {},
        }
    }

    return s;
}

/// Decodes a hex string into a given byte slice. The byte slice must be at least
/// half the length of the hex string.
pub fn hex(src: []const u8, dest: []u8) !void {
    std.debug.assert(dest.len * 2 >= src.len);

    for (src, 0..) |c, i| {
        const destByte = i / 2;
        const shifted = i % 2 != 0;

        const target = &dest[destByte];
        switch (c) {
            '0'...'9' => if (shifted) {
                target.* |= c - '0';
            } else {
                target.* = ((c - '0') << 4) | 0;
            },
            'A'...'F' => if (shifted) {
                target.* |= (c - 'A' + 10);
            } else {
                target.* = ((c - 'A' + 10) << 4) | 0;
            },
            'a'...'f' => if (shifted) {
                target.* |= (c - 'a' + 10);
            } else {
                target.* = ((c - 'a' + 10) << 4) | 0;
            },
            else => return error.DecodeError,
        }
    }
}

test "decode_hex min value" {
    var buffer: [256]u8 = undefined;
    try hex("00", &buffer);

    try testing.expectEqualSlices(u8, buffer[0..1], &[1]u8{0});
}

test "decode_hex max value" {
    var buffer: [256]u8 = undefined;
    try hex("FF", &buffer);

    try testing.expectEqualSlices(u8, buffer[0..1], &[1]u8{255});
}

test "decode_hex multiple bytes" {
    var buffer: [256]u8 = undefined;
    try hex("B2380490527A2136", &buffer);

    try testing.expectEqualSlices(u8, buffer[0..8], &[8]u8{
        178, 56, 4, 144, 82, 122, 33, 54,
    });
}

test "decode_hex invalid input" {
    var buffer: [256]u8 = undefined;

    try testing.expectError(error.DecodeError, hex("ZZ", &buffer));
}
