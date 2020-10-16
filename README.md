# SynthLang-Web

Generates random fantasy / constructed languages

## Inspiration

SynthLang is heavily inspired by [this reddit post](https://www.reddit.com/r/proceduralgeneration/comments/49x6ry/procedural_language_generator/) by "Snarfilingus."

SynthLang doesn't do a lot of the fancy things that Language Generator does (yet?).

## See also

[SynthLang library](https://github.com/rparrett/synthlang)

## Demo

There may be a demo running [here](https://synthlang.robparrett.com).

## TODO

- Select english sample vocabulary randomly from a larger pool of words
- Some sort of UI for generating more text with the current language
- Some sort of UI for tweaking language parameters
- Improved (optional? parametric?) filtering of output for pronounceability
- Syllable parts should be associated with sounds and there should be tooltips explaining how to pronounce a word
- When combining syllables to make words, should we obey syllable part weights? Probably.
- Words should be particular parts of speach and we should be able to conjugate them
- Generate some prepositions and Lordly titles and such
- Languages should occasionally be strewn with apostrophes and perhaps hyphens

## Build

```
git clone https://github.com/rparrett/synthlang
git clone https://github.com/rparrett/synthlang-web
cd synthlang-web
cargo make build
cargo make serve
```

## See also

An excellent blog post about generating words. I wish I would have read this before starting on this project.
http://mewo2.com/notes/naming-language/

The inspiration for this project
http://lang-gen.appspot.com/
