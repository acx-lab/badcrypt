const std = @import("std");
const assert = std.debug.assert;
const testing = std.testing;

// Computes the Hamming distance between two byte slices.
//
// Bytes passed to this slice can only be compared if they are
// the same length in bytes, otherwise an assertion error will
// be raised.
pub fn hamming_distance(a: []const u8, b: []const u8) usize {
    assert(a.len == b.len);

    var distance: usize = 0;
    for (a, b) |a_byte, b_byte| {
        var x = a_byte ^ b_byte;

        while (x != 0) {
            distance += x & 1;
            x >>= 1;
        }
    }

    return distance;
}

test "hamming_distance" {
    const cases = .{
        .{ &.{0}, &.{255}, 8 },
        .{ &.{0}, &.{0}, 0 },
        .{ &.{0, 255}, &.{0, 0}, 8 },
        .{ &.{1, 1}, &.{3, 3}, 2 },
        .{ "this is a test", "wokka wokka!!!", 37 },
    };

    inline for (cases) |c| {
        const a, const b, const expected = c;
        try testing.expectEqual(expected, hamming_distance(a, b));
    }
}
