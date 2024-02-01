const std = @import("std");
const assert = std.debug.assert;
const testing = std.testing;

const hex = @import("./hex.zig");
const decoders = @import("./decoders.zig");

pub fn xor_slices(a: []const u8, b: []const u8, dest: []u8) []const u8 {
    assert(a.len == b.len);
    assert(a.len <= dest.len);

    for (a, b, 0..) |an, bn, i| {
        dest[i] = an ^ bn;
    }

    return dest[0..a.len];
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
    const input_hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    const base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    var buffer: [256]u8 = undefined;
    var encoded_result: [256]u8 = undefined;
    try hex.decode(input_hex, &buffer);
    encode_b64(buffer[0..48], &encoded_result);

    try testing.expectEqualStrings(base64, encoded_result[0..64]);
}

test "badcrypt test case #2" {
    var first_decode: [256]u8 = undefined;
    var second_decode: [256]u8 = undefined;
    try hex.decode("1c0111001f010100061a024b53535009181c", &first_decode);
    try hex.decode("686974207468652062756c6c277320657965", &second_decode);

    var result: [256]u8 = undefined;
    const result_slice = xor_slices(first_decode[0..18], second_decode[0..18], &result);

    var result_decode: [256]u8 = undefined;
    try hex.decode("746865206b696420646f6e277420706c6179", &result_decode);
    try testing.expectEqualSlices(u8, result_decode[0..18], result_slice);
}

test "badcrypt test case #3" {
    const input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    const expected = "Cooking MC's like a pound of bacon";
    const input_bytes = input.len / 2;

    var buf: [input_bytes]u8 = undefined;
    try hex.decode(input, &buf);

    const decrypted = try testing.allocator.dupe(u8, &buf);
    defer testing.allocator.free(decrypted);
    decoders.single_byte_xor(decrypted, &buf);

    try testing.expectEqualStrings(expected, decrypted);
}

test "badcrypt test case #4" {
    const datafile = try std.fs.cwd().openFile("datasets/1_4.txt", .{});
    defer datafile.close();

    const file = try datafile.readToEndAlloc(testing.allocator, 36_000);
    defer testing.allocator.free(file);

    var lineIterator = std.mem.split(u8, file, "\n");
    var score: f32 = 1.0;
    const most_likely: []u8 = try testing.allocator.alloc(u8, 30);
    defer testing.allocator.free(most_likely);
    // Loops through each line in the file and decrypts it, scores the decrypted output,
    // and keeps track of the decrypted output with the highest score.
    while (lineIterator.next()) |line| {
        var buf: [256]u8 = undefined;
        try hex.decode(line, &buf);

        const decrypted = try testing.allocator.dupe(u8, buf[0 .. line.len / 2]);
        defer testing.allocator.free(decrypted);
        decoders.single_byte_xor(decrypted, buf[0 .. line.len / 2]);

        const line_score = decoders.score(decrypted);
        if (decoders.score(decrypted) > score) {
            score = line_score;
            @memcpy(most_likely, decrypted);
        }
    }

    std.debug.print("{s}\n", .{most_likely});
}
