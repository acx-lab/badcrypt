# Badcrypt

Bad implementations of [crytopals](https://cryptopals.com/) challenges.

I'm by no means a rust expert, but I attempt to be idiomatic where possible. My
focus is on being a practical as I can, choosing to duplicate code or write one-off
hacks in the interest of focusing on the challenges themselves.

# Set 1 - Basics

## 1.1 - Convert hex to base64

This was an interesting experiment in implementing the From trait on a custom
type. Getting a working solution was less complicated than I expected but designing
a reasonably _rusty_ interface was good practice.

I peeked into base64 encoders once I completed this challenge and found that
it was most common to operate over larger word sizes than a single byte. One
example handled 64 bits per iteration to complete the work in 1/4 the cycles.

## 1.2 - XOR Buffers

I kept this simple by iterating through the buffer bytes and executing the xor operation. Encoding each byte to hex is a simple
mapping but I chose to use the **hex** crate to reencode the result.

I may revisit this encoding later.

## 1.3 - Single Byte XOR Cipher

This was a fun challenge. I repeatedly was confused by base64 and the hex strings
but overall this challange was relatively simple. The challenge suggests using
character frequency as signal for scoring each resulting string.

I found a table online showing the frequency of each letter in a 40,000 word corpus
so I naively just applied those frequencies as a score. The highest score wins
and fortunately this was enough to get the answer.

## 1.4 - Detect single-character XOR

This was a progression of the previous problem with alot more noise than signal.
In hindsight, I could have done this all with the previous solution and a bit of
bash.

My previous scoring algorithm was enough to succeed here, returning the highest
score for each key, sorting the result, and chosing the top.

The one annoying bit is that rust tries to interpret every string as UTF-8, but
not every decrypted winner was valid. I made the (correct in this case) assumption
that the message would be a UTF-8 valid message.

```sh
cargo run --bin 1-4-cipher -- data/1-4-cipher.txt | sort --reverse | head -n 1
```

## 1.5 - Implement repeating-key XOR

This was straight forward, XOR each byte with a cycling character in the key _ICE_.
The code on this one is concise thanks to rust iterators, specifically the cycling
iterator to hash the source material against.

## 1.6 - Break repeating-key XOR

This challenge exposed several weaknesses in the scoring algorithm developed in
the previous challenge. Specifically, I assigned _points_ to a sequence of characters that scaled based on the frequency of that letter. For example, the
letter "e" would earn a phrase 12 points, the letter "t" would earn 8 points, and
so on and so forth.

This approach worked okay where frequencies were below or equal to expected frequencies but broke down when frequencies were greater than expected.

To account for this I adjusted the scoring algorithm to calculate the _real_
frequency of each letter and measured the distance from the expected frequency
to the observed frequency.

For example, given the phrase "hello", the frequency of the letter "e" would be
one out of five, or 20%. Given an expected frequency of 12% this means the phrase
would score 12 - 20, so -8. Finally, I normalized the distances by taking the absolute value of the of the distance.

After doing this for each letter in the alphabet I have a score which reflects
the distance from expected letter frequency. My assumption is that more _English-y_
phrases would score lower (i.e. closer to the expected distribution.)

This implementation was better but still had problems. Most significantly, uppercase
versions of lowercase keys would often produce identical character distributions
of all uppercase keys. For the sake of time, I settled for coding in heuristics
to bias against sets of characters with more than half uppercase characters.

Another heuristic in the algorithm biases against character sets with a large number of non-alphabet characters. By setting the expected frequency of non-letter characters, the the more symbol or number type characters that appeared the worse
the score.

With a couple heuristics and a distribution based scoring algorithm I was able
to decrypt the cipher.

This problem set highlighted the value of selecting a representative test set for
algorithms. Creating a test set around scoring allowed for incremental improvements
that eventually lead to a "good enough" result.

## 1.7 - AES in ECB mode
This challenge was simple thanks to the rust bindings for openssl.
```rust
    let message = openssl::symm::decrypt(
        // Specified encryption scheme.
        openssl::symm::Cipher::aes_128_ecb(),
        // The decryption key, provided.
        KEY.as_bytes(),
        // Initialization vector, not used in this case.
        None,
        // The cipher buffer.
        &buf,
    )
```

## 1.8 - Detect AES in ECB mode

This problem highlights a key vulnerability of ECB mode encryption. Ciphers created
in ECB mode, short for "electronic cookbook", expose patterns in the encrypted
material.

My first thought was to break each hex-encoded buffer into blocks of 16 bytes,
align the bytes by index, and then score the buffers with our scoring algorithm.
AES is more complex than our simple XOR encryption and does a much better job of
creating distance from the encrypted byte.

I still felt I was on to something. Aligning each byte corresponding to the encryption key value must manifest some pattern. I would expect text to contain
repeated letters, therefore, I expect the cipher text of that source material to
contain repeated bytes.

My next approach was to align the bytes again but this time just count the number of
duplicate bytes per chunk, sum that number, and expect the buffer that contains the
most repeated bytes to be the AES encrypted buffer.

This approach worked and led to an observation. The more assumptions I can make about the exncrypted data, the easier it is to exploit it's security. Exceptions
and special cases are generally avoided in application design, in this case, it
can provide bounds on the chaos of bytes.

We'll see how well this holds up in future challenges, and notably, designing
better test cases was more productive than tuning magic numbers.

# Set 2  - Block Crypto

## 2.1 - Implement PKCS#7 Padding

This one was too easy for comfort. It's a little unclear without a given cipher to break what The expected outcome should be. I implemented a function that pads buffers to a uniform size by appending as many `\x04`
characters as needed.

Not much to say on this one.

```sh
cargo run --bin 2-1-solution -- 'YELLOW SUBMARINE'
```

## 2.2 - Implement CBC mode

I learned my lesson about shooting from the hip in my implementation and started with some basic tests
for encrypting and decrypting my own sources. This was very helpful and this challenge
overall was straight forward.

The major hang ups here came from misunderstanding implementation details in the `openssl`
crate.

The first peculiar thing is that although you are encrypting a block at time,
you __must__ pass a buffer that is the length of content being encrypted _plus_ the block
size (in this case 128 bits). It's not documented why the api requires this, I'm willing
to bet it's an artifact of the underlying C implementation — namely that C requires
an extra byte at the end for null byte termination of arrays. This resulted in writing some
clumsy code that overallocates a mutable slice and then truncates it, seemingly
for no reason.

The second confusing part was entirely the result of not reading documentation carefully.
By default, the symmetric encryption options in the rust crate pad the content
being encrypted/decrypted. This led to unexpected cipher lengths and just a mess
overall.

Working with the non-idiomatic openssl library became easier when I made liberal
use of `assert!` macros to sanity check results.

The CBC implementation itself is simple and improves security on encrypted text
that is longer than one block. CBC adds no additional security for single block
ciphers because the encryption process at one block is identical.

```sh
cargo run --bin 2-2-solution -- data/2-2-b64-cipher.txt
```