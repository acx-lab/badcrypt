# Badcrypt

Bad and unsafe implementations of crytopals challenges.

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
iterator to hash the source masterial against.

## 1.6 - Break repeating-key XOR
TODO(allancalix): Fill me in.

## 1.7 - AES in ECB mode
TODO(allancalix): Fill me in.

## 1.8 - Detect AES in ECB mode
TODO(allancalix): Fill me in.