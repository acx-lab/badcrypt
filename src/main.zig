const std = @import("std");
const assert = std.debug.assert;
const testing = std.testing;

const decoders = @import("./decoders.zig");
const encoders = @import("./encoders.zig");

pub fn xor_slices(a: []const u8, b: []const u8, dest: []u8) []const u8 {
    assert(a.len == b.len);
    assert(a.len <= dest.len);

    for (a, b, 0..) |an, bn, i| {
        dest[i] = an ^ bn;
    }

    return dest[0..a.len];
}

test "badcrypt test case #1" {
    const input_hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    const base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    var buffer: [256]u8 = undefined;
    var encoded_result: [256]u8 = undefined;
    try decoders.hex(input_hex, &buffer);
    encoders.base64(buffer[0..48], &encoded_result);

    try testing.expectEqualStrings(base64, encoded_result[0..64]);
}

test "badcrypt test case #2" {
    var first_decode: [256]u8 = undefined;
    var second_decode: [256]u8 = undefined;
    try decoders.hex("1c0111001f010100061a024b53535009181c", &first_decode);
    try decoders.hex("686974207468652062756c6c277320657965", &second_decode);

    var result: [256]u8 = undefined;
    const result_slice = xor_slices(first_decode[0..18], second_decode[0..18], &result);

    var result_decode: [256]u8 = undefined;
    try decoders.hex("746865206b696420646f6e277420706c6179", &result_decode);
    try testing.expectEqualSlices(u8, result_decode[0..18], result_slice);
}

test "badcrypt test case #3" {
    const input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    const expected = "Cooking MC's like a pound of bacon";
    const input_bytes = input.len / 2;

    var buf: [input_bytes]u8 = undefined;
    try decoders.hex(input, &buf);

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
        try decoders.hex(line, &buf);

        const decrypted = try testing.allocator.dupe(u8, buf[0 .. line.len / 2]);
        defer testing.allocator.free(decrypted);
        decoders.single_byte_xor(decrypted, buf[0 .. line.len / 2]);

        const line_score = decoders.score(decrypted);
        if (decoders.score(decrypted) > score) {
            score = line_score;
            @memcpy(most_likely, decrypted);
        }
    }

    try testing.expectEqualStrings("Now that the party is jumping\n", most_likely);
}

pub fn repeated_key_xor(key: []const u8, data: []u8) !void {
    for (0..data.len) |i| {
        data[i] = data[i] ^ key[i % key.len];
    }
}

test "badcrypt test case #5" {
    const input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    const key = "ICE";
    const expected =
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272" ++ "a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

    var encrypted_input: [input.len]u8 = undefined;
    @memcpy(&encrypted_input, input);

    try repeated_key_xor(key, &encrypted_input);
    const hex_encoded_result = std.fmt.bytesToHex(&encrypted_input, .lower);

    try testing.expectEqualStrings(expected, &hex_encoded_result);
}
