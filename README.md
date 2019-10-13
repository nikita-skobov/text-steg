# text-steg

> A command line program that hides arbitrary binary data in a text file generated automatically using an n-gram linguistic model

# Table of Contents

* [Getting Started](#getting-started)
    * [Installing binary](#move-binary-to-path)
    * [Adding directory to path](#add-current-directory-to-path)
    * [Adding an alias](#add-an-alias)
* [Usage](#usage)
    * [Encoding](#encoding)
    * [Decoding](#decoding)
* [Examples](#examples)
* [What does this do?](#why)
* [How does it work?](#how)


# Getting Started

You must have rust installed: https://www.rust-lang.org/learn/get-started

Then you can:

```sh
git clone https://github.com/nikita-skobov/text-steg
cd text-steg/
cargo build --release
./target/release/text-steg --help # to verify it works
```

You can either move the compiled binary into a directory already in your path, add the current directory of the binary to your path, or make an alias to use the program without specifying the full path of the program every time.

## Move binary to path:

First find out which directories are in your path:

```sh
echo $PATH
```

It is recommended to use `/usr/local/bin` for programs that you compiled yourself, so use that if you have it. Otherwise, replace the following ocurrences of `/usr/local/bin` with whichever other folder you wish to use:

```sh
sudo cp target/release/text-steg /usr/local/bin/
```

Now you should be able to use the program in your terminal without having to type the whole path. Try running:

```sh
text-steg --help
```

To see if it works.

## Add current directory to path:

First find out which directories are in your path:

```sh
echo $PATH
```

Then simply do:

```sh
PATH=$PATH:/path/to/this/repo/target/release/
```

And then try to run 

```sh
text-steg --help
```

to see if it works.

## Add an alias:

You can list your current aliases with:

```sh
alias
```

Then add your alias by specifying the full path to the compiled binary:

```sh
alias text-steg="/path/to/this/repo/target/release/text-steg"
```

And then try to run 

```sh
text-steg --help
```

to see if it works.

# Usage

The program can either encode or decode stegotext.

Use

```sh
text-steg encode --help
# or
text-steg decode --help
```

to see the full options for encoding and decoding.

## Encoding

### Basic usage:

```sh
text-steg encode --file <file_to_encode> --output <name_of_output_file> --words <file_to_mimic_from>
```

This will take any arbitrary `<file_to_encode>` and generate stegotext that mimics the words and style of `<file_to_mimic_from>` and saves the output in `<name_of_output_file>`

### Advanced usage:

You can sepcify which algorithm you wish to encode with:

```sh
--algo <name_of_algo>
```

Where the `<name_of_algo>` can be one of:

- char-bit-shuffle (default)
- char-value-shuffle
- char-bit
- char-value

See [This explanation](#how) for how the different algorithms work

You can specify the number of bits you wish to encode with:

```sh
--bits <bits>
```

where `<bits>` must be a number between 1 to 8, defaults to 4.

You can specify the maximum n-gram generation of the mimic text:

```sh
-n <n>
```

where n is any positive integer. defaults to 2. *Note: the higher n, the longer, and more memory this program will use. Also with the char-bit algorithm, an n larger than 5 will rarely provide any better results*

You can specify a seed to be used to see the random shuffling. This seed must then be provided when decoding the stegotext:

```sh
--seed <seed>
```

The default seed is `abcd`, so if you encoded without specifying a seed, you can decode also by not specifying a seed, or by explicitly specifying `abcd`.

Alternatively to specifying a seed in the command, you can specify a password **which works exactly the same as a seed**, but the difference is that the program asks you for the password so that it does not show up in your command history. To use a password:

```sh
--pass true
```

The default is that pass is false, so you must explicitly state that it is true if you want to use a password.

## Decoding

### Basic usage

```sh
text-steg decode --file <file_that_was_encoded> --output <name_of_output_file>
```

This will take a file `<file_that_was_encoded>` that was previously encoded by this program, and decode it, and output the decoded secret data into `<name_of_output_file>`.

### Advanced usage

If you encoded with an algorithm other than the default, you must specify this algorithm when decoding:

```sh
--algo <name_of_algo>
```

Where the `<name_of_algo>` can be one of:

- char-bit-shuffle (default)
- char-value-shuffle
- char-bit
- char-value

See [This explanation](#how) for how the different algorithms work

If you encoded with a bitsize different than the default, you must specify this when decoding:

```sh
--bits <bits>
```

If you encoded with a password/seed you must specify this when decoding:

```sh
--pass true
```

and then input the same password that was used to encode the stegotext, or:

```sh
-seed <seed>
```

where `<seed>` is the seed/password that was used to encode the stegotext.

# Examples

The following examples show the type of output one can expect to see using various encoding options. See the [Last example](#char-value-with-n7-and-bitsize-of-1-best-results) to see the best possible results.

For simplicity, all of the following examples will use the same mimic file: varney.txt which is a slightly stripped down version of the plain-text version of the novel [Varney The Vampire](https://www.gutenberg.org/ebooks/14833) provided by Project Gutenberg. The secret file will also be the same for every example, and it will be called `secret.txt`:

```
email: someusername1212@gmail.com
pass: my_PASSworD98#
```


### basic char-bit

Command: 

```sh
text-steg encode --file secret.txt --output encoded.txt --words varney.txt
# note that this is the same as:
# text-steg encode --algo char-bit-shuffle --file secret.txt --output encoded.txt --words varney.txt
# because char-bit-shuffle is the default algorithm
```

Encoded.txt:

```
vampyre drinks . hush ! excuses . seize the town , consequently , saltzburgh , though , like asking ql by my own overjoyed qo certainly tvzd amazement thankful iii differing in great yzb jack , zealously from despair . hush ! murder qualified than by kxb chillingworth , froze with expectation , fighting , and perhaps , liquor , jack for it might carry their victim of his heart , saying , adjournment gain complaint . something previous to say , took to the unequally borne perquisite , from behind him partially in early childhood familiarized i zeal with charles from likely fixed his foot from the flames , lizzy , unequivocal and which being undertaken , but i lix . cruizing fqw just as josiah whoever charley zesr explain , and now she suffered . magistrate , replied the mob , quickening vxz indeed , might expect , if i require , anxiously . [illustration] the doctor felt that the most remarkable xk
```

Command to decode:

```sh
text-steg decode --file encoded.txt --output decoded.txt
```

### char-bit with seed

Command:

```sh
text-steg encode --file secret.txt --output encoded.txt --words varney.txt --seed mysecretseed
```

Encoded.txt:

```
object , charles informed of course during the joke . chapter of violence , perhaps gathering fresh joke . come to wnx zounds , exhausting mwkb john , wished to doubt , joy of lightning , jumped down quietly broken through a love would not , marmaduke were sufficiently to my dear friend , conjure sentinels adored esteemed unfathomable drug vault . upon their own eloquent dumb ascendancy , quick , under such xxxv . lizzy , but pray , johnsons definition architects exquisitely sublime explaining all events , explain , ready to enjoy qk upon the waving his pursuers for twelve , perfectly hideous mdz chillingworth sprang to him all of my beautiful , under any human life , froze with one glance upon such a mistake . darkness , whispered the mischief dejectedly . overlooked . you for blood , jacobs , square , because i walked backwards , come and eukj varney of amazement , consequently , chivalry . if anybody would tell you dog , and there
```

Command to decode:

```sh
text-steg decode --file encoded.txt --output decoded.txt --seed mysecretseed
```

### char-value with n5 and bitsize of 2

Command:

```sh
text-steg encode --file secret.txt --output encoded.txt --algo char-value-shuffle --words varney.txt --bits 2 -n 5
```

Encoded.txt:

```
said flora which prompts safeguards uniform of the most painful fact and the crime with the same man as ever drew marshal deeds ambuscade crowbar another time to see him before he did succeed firm as he spoke there was little inducement vampire in the town to see you more good god knows what we had done so much towards him a world to get up some evidence that some preternatural intensity to get into and obtain possession the house is a vampyre has got a shot as that which you are happily as a matter for the present to you that communication to the other as if my fate for him the whole affair is quite a mistake in all its parts of their own house as long time of it is a matter which they have killed a strange reason for withholding indiscretely shant motionless as if to recover my dear sir nay more of mystery in such cries come from your fears for some minutes no longer to be your second to sir a little time to indulge for a few weeks or two into a cry which was not likely they should ever again be it so happened that at one time looked like one of those hideous an end and i hope that not many
```

Command to decode:

```sh
text-steg decode --file encoded.txt --output decoded.txt --algo char-value-shuffle --bits 2
```

### char-value with n7 and bitsize of 1 (best results)

Command:

```sh
text-steg encode --file secret.txt --output encoded.txt --algo char-value-shuffle --words varney.txt --bits 1 -n 7
```

Encoded.txt:

```
i will go on to commit all the world like the portrait hangs in that way of a contradiction as well as some very strong light of truth and reflection of the rapidly consuming building is two young men set of circumstances arose and expressed it would seem to lead the same route to that which i have still in a few moments the medical friend the doctor should be able for a few days of his own objects in the dimness and repose of the house and all about it plain as an anchor with a deep sigh he continued to converse without being aware of the facts deserved any evil coming from it some wax lights to do it likewise to believe the marvellous to do something which was the old house is to get into the apartment seemed like a piece of sheer neglect and contemptuous takes the admiral looked like so much of their anger of a most fearful character of it to the other to conclude what he said by varney the supposed vampyre from the mischief that threatened him to be something more than earthly dwelling upon it and to the constancy of an hour longer to defend that he was not a man who would have been glad to get into the house of a man who had made up his mind what to do in the way you would be awakened some one was at once the miserable dungeon in which the man could have committed a great shout from the first to have an impression which i then placed my dear sir francis varney to the shout rent assisted in a few moments they must find the old fashioned knocker dusky looking eyes of the figure of a nature which is a fight for it yet he could not quite make a good thing of utter impossibility is a vampyre a blood sucker a human blood sucker receive all that was about to make a man sick with apprehension on floras account which he was not a man to be very busy in the world to be done now to be done with such an air of desertion jocose bits workmen were the only one of the set among whom they thought of that which i so much admire what you are about to leave the room was very dark now that he was no one to be done with such fearful rapidity that i have made it a matter which he was not quite so bad as many persons would be to tell my secret of what you are saying
```

Command to decode:

```sh
text-steg decode --file encoded.txt --output decoded.txt --algo char-value-shuffle --bits 1
```

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

The 'Char-value' algorithm is slightly simpler than the 'Char-bit' algorithm. It assigns every character in the alhabet a value ranging from 0 to (2^N - 1) in increments of powers of 2. where once again N is the number of bits chosen to be encoded/decoded with.

The value is decoded by adding the value of each character present in the word. In the Char-bit algorithm, duplicate characters were ignored since a bit can either be set, or not set; you cannot set a bit twice. In this algorithm, however, duplicate characters are counted, so you simply add the value for that character twice. If you reach the maximum value, 2^N, it overflows the value back to 0 and starts again.

The values that correspond to a character are determined based on the number of bits. The simplest case is when the number of bits is 1. In this case, every single word corresponds to a bit either being 0 or 1. Therefore we divide all of our characters in the alphabet into two sets, and one set gets every character assigned a value of 0, and the other set gets every character assigned a value of 1.

Consider an example:

| i | t | a | o | e | n | s | h | ... (more letters) |
| - | - | - | - | - | - | - | - | ------------------ |
| 0 | 1 | 0 | 1 | 0 | 1 | 0 | 1 | ... (more values)  |

So if we were to encode a byte of 254, we would split it up into single bit pieces: `1`, `1`, `1`, `1`, `1`, `1`, `1`, `0`

and then for each piece, we need to find a word whose value is 1. Consider: **`note`**. every character maps to a 1 except for **`e`**. Therefore, 1 + 1 + 1 + 0 = 3. But then we perform modulo arithmetic to get the actual value: `3 % 2^N = 3 % 2^1 = 3 % 2 = 1`, so our value is 1. It is clear to see that with this encoding method, there are many more words that can give us a specific value, however this is only true with small sizes of N. If we used an N of 8, meaning we must be able to encode any value from 0 to 255, our choices of words for any given value is rather small. 

Let's say we provide a file to mimic that contains 1024 unique words. If we use a bitsize of 1, that means approximately 512 of those words have a value of 0, and approximately 512 have a value of 1. If, however, we use a bitsize of 2, we now have 4 sets of words: each set contains roughly 256 words, and the 4 sets map to values of 00, 01, 10, and 11.

Because of this, the Char-value algorithm is not well suited for large bitsizes. However, the Char-value algorithm produces much better stegotext at a low bitsize compared to the Char-bit algorithm at the same low bitsize.


### Important note about the encoding/decoding tables:

In the above examples, we considered static tables that map a certain character to a certain bit position/value. In the actual program, this table is shuffled for every new value that needs to be encoded/decoded. This accomplishes two things:

1. It allows for a user to input a password that seeds a random number generator such that the stegotext can only be decoded with the same password that it was encoded with.
2. It prevents patterns from showing up in the stegotext that might reveal the pattern of the original secret data.

the former is not that important since the ideal use of this program is to encrypt the secret data prior to generating the stegotext, however the latter is much more important. Many binary files have long sequences of zeros in a row. If we used a static mapping table, we would see a high chance of a single word be outputted several times in a row.

It is because of these 2 reasons that the actual program implements shuffling of the table via a passwords/seed by default. This behavior can be disabled by the user if they so wish, but it is mostly only provided for example purposes/debugging.