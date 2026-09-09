# G2P

A tiny streaming grapheme-to-phoneme CLI built on
[espeak-ng](https://github.com/espeak-ng/espeak-ng). Text goes in,
espeak-ng's IPA phonemes come out — one JSON object per line over
stdin/stdout, so any program in any language can use espeak-ng G2P
through a subprocess pipe.

## Usage

```bash
$ g2p
{"text":"Bonjour tout le monde.","lang":"fr"}
{"phonemes":"bɔ̃ʒˈuʁ tˈu lə mˈɔ̃d"}
{"text":"नमस्ते, आप कैसे हैं?","lang":"hi"}
{"phonemes":"nˌʌmʌstˈeː ˈaːp kˈɛːseː hˈɛ̃ː"}
```

- **Request:** `{"text": "...", "lang": "<espeak voice id>"}` — e.g.
  `en-us`, `es`, `fr`, `it`, `pt-br`, `hi`.
- **Response:** `{"phonemes": "..."}` (words joined with single
  spaces) or `{"error": "..."}`.
- The process is long-running: initialize once, send one request per
  line, read one reply per line, close stdin to exit. espeak-ng keeps
  global state — send one request at a time per process.

## espeak-ng data

espeak-ng needs its data files at runtime. Point
`PIPER_ESPEAKNG_DATA_DIRECTORY` at the directory **containing**
`espeak-ng-data` (a pruned per-language subset works). Keep the path
short — espeak-ng has an internal path buffer of roughly 160 bytes.

## Building

```bash
cargo build --release   # needs cmake (espeak-ng is compiled statically)
```

The espeak-ng sources are vendored inside the `espeak-rs-sys` crate
package and pinned by `Cargo.lock`; this repository plus those pinned
crates form the complete corresponding source of the built binary.

## License

GPL-3.0-or-later — this program statically links espeak-ng, which is
GPLv3+. See [LICENSE](LICENSE).
