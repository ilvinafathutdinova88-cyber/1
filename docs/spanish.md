# Spanish

Sing accepts Spanish lyrics, as spoken in Spain. Spanish is written in Latin letters, so the
router can't tell it from English by script: choose **Español** in the menu on the Lyrics
screen (`lang` `"es"` in `render notes` and `chimera syllables --lang`). Words in Latin letters
are then read by `crates/chimera/src/phonetics/spanish.rs`; Hangul and Cyrillic still go to Korean
and Russian. The demo songs' own words are read as written, whatever the menu says.
There are no new sounds: Spanish is mapped onto the existing inventory.

Speak reads Spanish too: choose the language in the menu next to *Text is IPA* (the same choice as
Sing's lyrics), or `chimera render text <out> "<text>" [body] --lang es`. It is the same reading,
plus what speech needs:

- **Vowel lengths.** The planner's duration rules are English's (Klatt), which shorten unstressed
  vowels to ~40 ms; Spanish is syllable-timed and keeps them, so the front end times its vowels:
  75 ms unstressed, 105 ms stressed, the phrase's last half again as long, glides 50 ms (all
  scaled by the Rate).
- **Questions.** A yes/no question rises (*¿Me oyes?*); one that starts with a question word
  (*¿Dónde estás?*, *¿Por qué no?*) falls like a statement.
- **Across words** b, d, g soften as in singing (*la boca*), and a phrase starts afresh.

## Writing it

- Write it as it's spelt, with its accents: the spelling gives the sounds and the stress.
  Decomposed accents (a + U+0301) are read too. ¿ and ¡ are skipped; ? and ! end a phrase.
- One syllable is one note: `co-ra-zón`. A diphthong is one syllable (`bien`, `ai-re`, `ciu-dad`);
  a hyphen splits one where the tune wants two notes (`pi-a-no`), and keeps the word's stress.
- `~` holds the previous vowel onto the next note.
- Write numbers as words.

## What it does

| | |
|---|---|
| Vowels | a e i o u; i, u (and a final y) glide next to another vowel; an accent mark makes them a syllable of their own (dí-a, ba-úl) |
| Stress | the vowel with an accent mark; else the last syllable but one if the word ends in a vowel, n or s; else the last. Articles, prepositions and object pronouns of one syllable are unstressed |
| c, z | /θ/ before e, i and for z |
| g, j | j, and g before e, i: /x/; gue, gui: /g/ with a silent u; güe, güi keep /w/ |
| ll, y | /ʝ/ (yeísmo); y alone or after a vowel is the vowel i (y, hoy, muy) |
| r | one tap /ɾ/; rr, and r at the start of a word or after n, l, s: the trill /r/ |
| b, d, g | soften to /β ð ɣ/ after a vowel or a consonant, across words too (la boca); stay stops after a pause, a nasal, or (d) l |
| n | /m/ before p, b, m; /ŋ/ before k, g, x |
| h | silent; x is /ks/ (/s/ at the start of a word); qu is /k/ |
| p t k ch | unaspirated |

## What is approximate

- **t, d** are the alveolar ones, not dental.
- **The trill** is the inventory's /r/: two or three strikes of the tongue tip, about 20 a
  second, with the voice carrying on between them. Until 2026-11-16 the strikes ran together into
  one ~90 ms silence (a native listener heard *perro* as something strange; `tests/trill.rs`).
  Real trills run at about 25 Hz, but the model's tongue tip is too slow to strike and reopen
  that fast.
- **Sinalefa**: vowels don't merge across words ("mi amor" is three notes); write `mia-mor` to
  sing it in two.
- Not done: loanwords (*whisky*), x as /x/ (*México*), seseo and other regional sounds (aspirated s,
  ʃ for ll/y), numbers (digits are skipped), Spanish rhythm beyond vowel length (consonants keep
  the English rules), and the pitch accent on stressed syllables.

## Tests

`crates/chimera/tests/spanish.rs` has one case per rule above.
