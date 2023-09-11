const std = @import("std");
const assert = std.debug.assert;
const testing = std.testing;

/// Decodes a hex string into a given byte slice. The byte slice must be at least
/// half the length of the hex string.
pub fn decode_hex(hex: []const u8, dest: []u8) !void {
    std.debug.assert(dest.len * 2 >= hex.len);

    for (hex, 0..) |c, i| {
        const destByte = i / 2;
        const shifted = i % 2 != 0;

        var target = &dest[destByte];
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
                target.* |= (c - 'A' + 10);
            } else {
                target.* = ((c - 'A' + 10) << 4) | 0;
            },
            else => return error.DecodeError,
        }
    }
}

pub fn encode_b64(src: []const u8, dest: []u8) void {
    // TODO(allancalix): Handling padding and non-multiples of 3 for input slices.
    const padding = src.len % 3;
    _ = padding;

    var i: usize = 0;
    var j: usize = 0;
    while (i < src.len) : (i += 3) {
        var value: u24 = 0;
        value |= (@as(u24, 0) | src[i]) << 16;
        value |= (@as(u24, 0) | src[i + 1]) << 8;
        value |= src[i + 2];

        dest[j] = base64_std(@truncate(value >> 18));
        dest[j + 1] = base64_std(@truncate(value >> 12));
        dest[j + 2] = base64_std(@truncate(value >> 6));
        dest[j + 3] = base64_std(@truncate(value));
        j += 4;
    }
}

fn base64_std(b: u6) u8 {
    switch (b) {
        0...25 => return @as(u8, b) + 'A',
        26...51 => return @as(u8, (b - 26)) + 'a',
        52...61 => return @as(u8, (b - 52)) + '0',
        62 => return '+',
        63 => return '/',
    }
}

test "decode_hex min value" {
    var buffer: [256]u8 = undefined;
    try decode_hex("00", &buffer);

    try testing.expectEqualSlices(u8, buffer[0..1], &[1]u8{0});
}

test "decode_hex max value" {
    var buffer: [256]u8 = undefined;
    try decode_hex("FF", &buffer);

    try testing.expectEqualSlices(u8, buffer[0..1], &[1]u8{255});
}

test "decode_hex multiple bytes" {
    var buffer: [256]u8 = undefined;
    try decode_hex("B2380490527A2136", &buffer);

    try testing.expectEqualSlices(u8, buffer[0..8], &[8]u8{
        178, 56, 4, 144, 82, 122, 33, 54,
    });
}

test "decode_hex invalid input" {
    var buffer: [256]u8 = undefined;

    try testing.expectError(error.DecodeError, decode_hex("ZZ", &buffer));
}

test "encode_b64 min" {
    var buffer: [256]u8 = undefined;
    encode_b64(&[3]u8{ 0, 0, 0 }, &buffer);
    try testing.expectEqualStrings("AAAA", buffer[0..4]);
}

test "encode_b64 max" {
    var buffer: [256]u8 = undefined;
    encode_b64(&[3]u8{ 255, 255, 255 }, &buffer);
    try testing.expectEqualStrings("////", buffer[0..4]);
}

test "badcrypt test case #1" {
    const hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    const base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    var buffer: [256]u8 = undefined;
    var encoded_result: [256]u8 = undefined;
    try decode_hex(hex, &buffer);
    encode_b64(buffer[0..48], &encoded_result);

    try testing.expectEqualStrings(base64, encoded_result[0..64]);
}
