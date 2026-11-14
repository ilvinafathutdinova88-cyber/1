# CHIMERA

A physical-modeling voice synthesizer in the browser. The web app is branded **Chimera Arabic
Version** and can switch its interface and preferred speech examples between Arabic and English.
Every sound comes from a simulated airway (lungs, vocal folds or syrinx, a branched vocal tract,
lips, nose), not from samples.
The engine is Rust, compiled to WebAssembly for an AudioWorklet; the same code runs natively
for tests and the `chimera` CLI.

## Chimera Arabic Version

This repository is an Arabic-focused fork of [the original CHIMERA project](https://github.com/jamesondh/chimera-vocal-synth).
It extends the physical voice simulator with Arabic speech and singing support. Sound is generated
by a model of the lungs, vocal folds and vocal tract—not by prerecorded audio or an AI voice model.

### Arabic features in this fork

- **Arabic/English interface:** Switch languages from the top bar. Arabic mode sets the page to
  right-to-left layout and provides Arabic speech and song examples.
- **Direct Arabic text routing:** Arabic script is sent to the Arabic front end; it is not
  transliterated into English spelling or passed through English G2P. Arabic and English can be
  mixed in one input, with routing selected by script.
- **Tashkeel and pronunciation varieties:** Short vowels, sukūn, shadda, tanwīn and long-vowel
  letters inform pronunciation. Users can choose Modern Standard Arabic or common Egyptian
  pronunciations. Unvowelled words use a conservative fallback, so adding tashkeel usually gives
  the engine better pronunciation information.
- **Arabic syllables for singing:** Vowel marks and long-vowel letters help determine Arabic
  syllable boundaries while keeping the written playback order. Boundaries in some unvowelled
  words remain approximate.
- **Initial Arabic articulation behavior:** Emphatic consonants have distinct tongue targets. An
  experimental rule also applies part of the emphatic target to an adjacent vowel. This 35% blend
  is an engineering starting point, not a trajectory measured from Arabic speakers; acoustic
  evaluation and native-speaker review are needed before treating it as accurate.
- **Breath-driven physical voice:** This fork uses CHIMERA's existing physical airflow model to
  drive the vocal tract and voice instead of assembling prerecorded samples.

### Try Arabic

1. Start the app and open **Speak** for speech or **Sing** for singing.
2. Use **Arabic / English** to switch the interface language and examples.
3. In **Speak**, enter Arabic text and select **Standard Arabic** or **Egyptian**. In **Sing**, try
   a vocalized lyric such as `السَّلَامُ عَلَيْكُمْ`.

### Current limitations

This project is **not a Quran or Tajweed reciter**. It does not model recitation signs, connected
recitation rules or Tajweed, and should not be used to produce audio presented as verified
recitation. Unvowelled Arabic, context-sensitive pronunciation and the experimental articulation
rule also need further work and native-speaker review. The current goal is Arabic speech and
singing through the general physical voice model.

### How the Arabic path works

Arabic letters and diacritics → Arabic and dialect front end → phonemes and syllables → shared
articulatory planner → vocal-tract simulation and audio output. The same Rust engine runs natively
and as WebAssembly in the browser. Arabic-specific regression tests are in
`crates/chimera/tests/arabic.rs`. Run the project checks with:

```sh
bun run test
bun run typecheck
```

This Arabic edition builds on the original CHIMERA project. See the [license](LICENSE) and
project documentation before redistribution.

## Vision & scope

1. **One instrument**: a physically modeled voice (lungs, vocal folds, throat, mouth). No
   samples, no neural audio.
2. **The web app is the primary surface**: anyone should be able to make songs in it without
   an LLM's help. Mobile is the likely next surface. The plugin (CLAP/VST3/AU) still ships, but
   it's secondary.
3. **As realistic as we can make it**: the main push is to see how close the simulation gets to
   a real voice. The robotic, Vocaloid-like sound people like today stays playable as a preset
   ([#93](https://github.com/jamesondh/chimera-vocal-synth/issues/93)).
4. **Languages enter through one shared front-end design**, not one-off hacks. Each needs a
   native speaker's check and must leave English unchanged.
5. **The design language (the vector display) is settled**; the look within it can still evolve.
6. **Nothing copyrighted in git**; MIT; a deterministic, real-time-safe engine.
7. **Not now**: a DAW or sequencer, a light theme, MPE, multi-voice dialogue.

## What it does

- **Play**: drag the tongue, lips and jaw in a live cutaway of the head; vowel pad, encoders,
  computer keyboard and Web MIDI.
- **Speak**: English text (CMUdict with a letter-to-sound fallback), Arabic text (automatically
  routed from its script, with tashkeel and Egyptian or Modern Standard pronunciation), or IPA,
  turned into articulatory gestures and prosody, with the gestural score and a consonant chart on
  screen. Speech plans add an inhalation in pauses long enough for a breath. In IPA mode a notice
  under the box lists the symbols and marks the voice doesn't say (`chimera render speech` prints
  the same as warnings).
- **Sing**: type words and the **Sing** button sings them to a tune picked by its mood, each line
  fitted to a phrase of it (Tempo and Key on the Voice screen); **Record video** sings it again into a
  9:16 clip to share. The keys sing the same words a syllable at a time, legato or not, onto a
  pitch roll; consonants land before the beat. Lyrics can be English, Arabic (including tashkeel),
  Korean (Hangul: one block per note, with Korean sound rules) or Russian; each language is a front end
  ([docs/language-front-ends.md](docs/language-front-ends.md)).
- **Beatbox**: pads and a step sequencer, every drum an articulatory gesture.
- **Bestiary**: songbird (two-voice syrinx), seal, alligator and elephant.
- **Duet** (`?show=<id>`): two 3D heads replay a pre-rendered song.

Male, female and child bodies blend continuously. Everything on screen animates from live
engine state: pressure in the airway, airflow, the vocal folds, spectra and formants.

## Run it

Needs Rust with the `wasm32-unknown-unknown` target, and Bun.

```sh
bun install
bun run dev        # builds the wasm, then Vite on 127.0.0.1:5317
bun run test       # wasm build, cargo test --workspace, bun test tests
```

The AudioWorklet needs a secure context, so open it on localhost or over HTTPS (for example
`tailscale serve`). Vercel builds it with `vercel.json`.

Offline renders and measurement:

```sh
cargo run --release -p chimera-cli -- render speech|text|song|notes|beatbox|creature|vowels …
cargo run --release -p chimera-cli -- diagnose       # nasal leakage, formants, bandwidths, pitch
cargo run --release -p chimera-cli -- compare csd    # against a real singer (docs/csd-comparison.md)
cargo run --release -p chimera-cli -- compare tsp    # against 23 real speakers (docs/speech-comparison.md)
bun tools/spectrogram.ts in.wav out.png
bun tools/musicxml-to-notes.ts score.mxl P1 --out p1.json   # a score's part → `render notes` melody
```

## Docs

- [docs/architecture.md](docs/architecture.md): the acoustic model (FDTD airway, sources,
  anatomy, gestures), and its [references and credits](docs/architecture.md#references-and-credits).
- [docs/rust-engine.md](docs/rust-engine.md): the engine's real-time and determinism contract,
  and the browser boundary.
- [docs/ui-redesign.md](docs/ui-redesign.md) and [docs/mockups/](docs/mockups/): the vector
  display look.
- [docs/csd-comparison.md](docs/csd-comparison.md): scoring the voice against the Children's
  Song Dataset.
- [docs/speech-comparison.md](docs/speech-comparison.md): scoring the speech against real
  speakers, phone by phone (the TSP speech database).
- [docs/russian.md](docs/russian.md): how Russian text is read, and what is approximate.
- [docs/hill-climb/](docs/hill-climb/): the rules and log for tuning the voice one measured
  step at a time.

Contributing: see [CONTRIBUTING.md](CONTRIBUTING.md). Agents: see [AGENTS.md](AGENTS.md).

## License

[MIT](LICENSE). Bundled third-party files keep their own licenses: the CMU Pronouncing
Dictionary ([public/data/CMUDICT-LICENSE.txt](public/data/CMUDICT-LICENSE.txt)) and the fonts
([public/fonts/](public/fonts/)).
