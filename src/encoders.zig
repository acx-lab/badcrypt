const std = @import("std");
const testing = std.testing;

pub fn base64(src: []const u8, dest: []u8) void {
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

test "encode_b64 min" {
    var buffer: [256]u8 = undefined;
    base64(&[3]u8{ 0, 0, 0 }, &buffer);
    try testing.expectEqualStrings("AAAA", buffer[0..4]);
}

test "encode_b64 max" {
    var buffer: [256]u8 = undefined;
    base64(&[3]u8{ 255, 255, 255 }, &buffer);
    try testing.expectEqualStrings("////", buffer[0..4]);
}
