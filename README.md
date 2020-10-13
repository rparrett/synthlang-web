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

- Some sort of UI for generating more text with the current language
- Some sort of UI for tweaking language parameters
- Some sort of (optional? parametric?) filtering of output for pronounceability
- Syllable parts should be associated with sounds and there should be tooltips explaining how to pronounce a word
- It should be possible for a word to begin with e.g. `Ng`, but every language with `Ng` doesn't need to have this feature
  Maybe a "language" should also include probabilities of certain syllable part pairings, (including the null syllable part)
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
