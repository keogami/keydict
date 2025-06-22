# Keydict

A dictionary search algorithm designed to work with old keypad inputs.

Part of my research to test the hypothesis that typing on older phones was
faster than the digital qwerty keyboard on current smartphones.

## Usage

First, we need to form a tree out of our dictionary. Given a new line separated
word list `words.txt`, we can generate a tree with:

```bash
keydict generate-tree words.txt
```

This will generate `words.kdtree` file next to the wordlist. This can be used
for actual searches.

Then, to search for a word like `dog`, use the keys `364`. Look at the keypad on
your dialing app to find keys for your words.

```bash
keydict search --words words.kdtree 364
```

Finally, to find completions use `complete` subcommand. For example, to find the
words `poverty`, use the keys `768378`

```bash
keydict complete --words words.kdtree 768378
```

## Example

With the wordlist from
[https://github.com/dwyl/english-words](dwyl/english-words) I get the following
suggestion.

```bash
> keydict complete --words aux/words_alpha.kdtree 768378
loading file
took 298.380523ms
search took 27.157µs
found the following words by the prefix:
[
    "potestal",
    "potestas",
    "potestate",
    "potestative",
    "poverties",
    "poverty",
    "povertyweed",
    "soverty",
]
```

## Further Work

The suggestions returned can be ranked with some sort of commonality factor,
ranking more common words higher; but that is out of scope of this algo.

The algo itself does quite some unneccessary allocations that can be avoided,
but for now it's fast enough. Still, feel free to raise a PR.
