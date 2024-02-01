const std = @import("std");

const MOST_COMMON_CHAR = "e";

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

fn score(attempt: []const u8) f32 {
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
