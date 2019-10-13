# text-steg

> A command line program that hides arbitrary binary data in a text file generated automatically using an n-gram linguistic model

# Why?

The point of steganography is to hide secret data in an inocuous-looking carrier. Historically, steganography in the digital age has been mostly done via images and sound because they are large files, and thus can hold a lot of hidden data. Additionally, by simply looking at an image file it can be near impossible to tell if the image contains hidden data because our eyes can't make out such small differences. While a human probably can't tell if a given file has hidden data, a computer sometimes can. Thus with the rise of image steganography, steganalysis has become more an increasingly used tool in internet security. Steganalysis software tries to perform various statistical tests to tell if an image contains hidden data. Because of this, I was interested in trying to make software that can generate steganographically encoded text (from here on out referred to as stegotext) that might be better suited at avoiding automatic detection.

**It is my assumption that it is difficult for a computer to evaluate the legitimacy of a text file** because the quality of text varies greatly based on the person who wrote it.

There are two main challenges with creating a text steganography program:

1. Text is dense, and does not have a lot of room to hide data in. Therefore, your ratio of hidden data to overall stegotext size is going to be small.
2. Text can be more easily assesed by an individual than an image. A poorly encoded stegotext will stand out to a human's eyes more than a poorly encoded stego-image.

I believe the first issue is addressed fairly well from this project, but the second issue is more or less subjective. 

# How?

The general process of how this program works is as follows:

You provide a file that you wish to hide. You provide some source text that the program tries to mimic. The program attempts to pick words from the source text in an order that not only appears like real text, but also in a way that it can be decoded back into the original data from the file that you wish to hide.


I came up with two different algorithms for encoding arbitrary binary data into stegotext, which I have named:

- 'Char-bit'
- 'Char-value'


## Char-bit

The 'Char-bit' algorithm assigns every bit position to a character. For example, consider this encoding table:


| 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 
| - | - | - | - | - | - | - | - | - |
| e | a | o | l | i | h | t | u | s |


and consider a sequence of values to be encoded:

```
255, 0, 3
```

255 in binary is **`11111111`**, so it contains every bit, and therefore to encode a value of 255, we must find a word that contains the characters: **`aolihtus`** in any order. Next, we have to encode a value of 0. since 0 in binary is **`00000000`**, there are no bits that are set, and that is why we have a value for explicitly 0, in this case **`e`**. So we must find a word that contains **`e`**, but does not contain any other character from the table. Lastly, we must encode a value of 3. In binary, 3 is **`00000011`**, so we must find a word that contains **`su`** but does not contain any other characters from the table.

Decoding works backwards. Given a word such as **`howl`**, we see that it contains an **`h`**, **`o`**, and **`l`** from the table, therefore to decode the value that `howl` represents, we set the 5th, 2nd, and 3rd bit: **`01101000`**.

You might be wondering, why even assign a character to represent 0? If our table was:

| 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 
| - | - | - | - | - | - | - | - |
| a | o | l | i | h | t | u | s |

We could say that a word like **`my`** has a value of 0 because it does not contain any character from the table. However, we do not want this behavior because then any word that doesn't contain any character from the table will be considered a 0 value, and this greatly restricts the set of words that can be chosen for any given value. Instead, we want to have an explicit character for 0, because that allows us to have words like **`my`** show up in the stegotext that do not correspond to any particular value. This introduces noise to the stegotext, meaning not every single word corresponds to a value, and allows for a more natural-looking stegotext.

It is important to mention that **in this example we considered values of 8 bits, however, the algorithm allows you to choose a number of bits from 1 to 8 inclusively, and it defaults to 4.**

So if we were using a bitsize of 4, and we had to encode a byte: 254, then we would split it in half, and encode 2 words: **`1111`** and **`1110`**, and then our table would only contain 5 keys.

In the char-bit algorithm, the table map always contains **N + 1** keys, where **N** is the number of bits that you wish to encode/decode with. Choosing a larger **N** reduces the size of the output stegotext, but reduces the quality of the sentences that can be formed.

## Char-value