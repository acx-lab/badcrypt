const std = @import("std");
const testing = std.testing;

/// Decodes a hex string into a given byte slice. The byte slice must be at least
/// half the length of the hex string.
pub fn decode(hex: []const u8, dest: []u8) !void {
    std.debug.assert(dest.len * 2 >= hex.len);

    for (hex, 0..) |c, i| {
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
    try decode("00", &buffer);

    try testing.expectEqualSlices(u8, buffer[0..1], &[1]u8{0});
}

test "decode_hex max value" {
    var buffer: [256]u8 = undefined;
    try decode("FF", &buffer);

    try testing.expectEqualSlices(u8, buffer[0..1], &[1]u8{255});
}

test "decode_hex multiple bytes" {
    var buffer: [256]u8 = undefined;
    try decode("B2380490527A2136", &buffer);

    try testing.expectEqualSlices(u8, buffer[0..8], &[8]u8{
        178, 56, 4, 144, 82, 122, 33, 54,
    });
}

test "decode_hex invalid input" {
    var buffer: [256]u8 = undefined;

    try testing.expectError(error.DecodeError, decode("ZZ", &buffer));
}
