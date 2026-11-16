//! Spanish spelling → the existing inventory (no new sounds). Spanish is written in Latin letters,
//! so the router can't tell it from English by script: it is chosen explicitly
//! ([`super::syllables::Latin`]).
//!
//! Spelling gives the sounds: c/z before e, i are /θ/ (Spain's distinción); ll and y before a
//! vowel are /ʝ/ (yeísmo); j, and g before e, i, are /x/; h is silent; qu, gu before e, i are /k/,
//! /g/ (gü keeps its /w/); rr, and r at the start of a word or after n, l, s, are the trill.
//! b, d, g soften to /β ð ɣ/ after a vowel or most consonants (not after a pause or a nasal, nor d
//! after l), across words too. n takes the place of a following labial or velar.
//!
//! Vowels: a, e, o are strong; i, u (and y at the end of a word) are weak and glide next to
//! another vowel (diphthongs: bien, aire, ciudad); an accent mark makes a weak vowel strong (dí-a).
//! Stress: the vowel with an accent mark; otherwise the last syllable but one if the word ends in a
//! vowel, n or s, else the last. Stops are unaspirated.
//!
//! Speech ([`to_phrases`]): the same reading, with Spanish vowel lengths (the duration rules are
//! English's, which all but swallow unstressed vowels, and Spanish keeps them), a question that
//! starts with a question word (¿dónde…?) falling like a statement, and the others rising.
//!
//! Approximations: t, d are the alveolar ones, not dental; words don't merge their vowels
//! (sinalefa: "mi amor" is three notes; write "mia-mor" to sing it in two).
use super::english::Phrase;
use super::syllables::{syllabify_with, FrontEnd};
use super::{Intonation, Segment, Syllable};

/// A letter of a Spanish word (base letters, accented vowels, ñ, ü, either case).
pub fn is_letter(c: char) -> bool {
    c.is_ascii_alphabetic() || "áéíóúüñÁÉÍÓÚÜÑ".contains(c)
}

/// Combining marks a word may be typed with (decomposed input): acute, tilde, diaeresis.
fn is_mark(c: char) -> bool {
    matches!(c, '\u{301}' | '\u{303}' | '\u{308}')
}

/// The word lowercased with decomposed accents composed (a + U+0301 → á).
fn normalize(word: &str) -> Vec<char> {
    let mut out: Vec<char> = Vec::new();
    for c in word.chars() {
        let composed = match (out.last().copied(), c) {
            (Some(b), '\u{301}') => match b.to_ascii_lowercase() {
                'a' => Some('á'), 'e' => Some('é'), 'i' => Some('í'), 'o' => Some('ó'), 'u' => Some('ú'), _ => None,
            },
            (Some(b), '\u{303}') if b.eq_ignore_ascii_case(&'n') => Some('ñ'),
            (Some(b), '\u{308}') if b.eq_ignore_ascii_case(&'u') => Some('ü'),
            _ => None,
        };
        if let Some(k) = composed {
            *out.last_mut().unwrap() = k;
        } else if is_letter(c) {
            out.extend(c.to_lowercase());
        }
    }
    out
}

/// The vowel a letter spells, without its accent (y is not one here: see `units`).
fn vowel_of(c: char) -> Option<char> {
    Some(match c {
        'a' | 'á' => 'a',
        'e' | 'é' => 'e',
        'i' | 'í' => 'i',
        'o' | 'ó' => 'o',
        'u' | 'ú' | 'ü' => 'u',
        _ => return None,
    })
}
fn is_front(c: Option<&char>) -> bool {
    c.and_then(|c| vowel_of(*c)).is_some_and(|v| v == 'e' || v == 'i')
}

/// One sound of the word, with the letter it starts at.
#[derive(Clone, Debug)]
struct Unit {
    ipa: &'static str,
    letter: usize,
    /// a vowel: (strong, written with an accent)
    vowel: Option<(bool, bool)>,
}

fn cons(ipa: &'static str, letter: usize) -> Unit {
    Unit { ipa, letter, vowel: None }
}

/// Letters → sounds, before vowels group into syllables.
fn units(l: &[char]) -> Vec<Unit> {
    let theta = "θ";
    let mut out: Vec<Unit> = Vec::new();
    let mut i = 0;
    while i < l.len() {
        let at = i;
        let c = l[i];
        let next = l.get(i + 1);
        i += 1;
        if let Some(v) = vowel_of(c) {
            let accent = matches!(c, 'á' | 'é' | 'í' | 'ó' | 'ú');
            let ipa = match v { 'a' => "a", 'e' => "e", 'i' => "i", 'o' => "o", _ => "u" };
            out.push(Unit { ipa, letter: at, vowel: Some((matches!(v, 'a' | 'e' | 'o') || accent, accent)) });
            continue;
        }
        let ipa = match c {
            'h' => continue, // silent (ch is read at the c)
            'c' if next == Some(&'h') => {
                i += 1;
                "tʃ"
            }
            'c' if is_front(next) => theta,
            'c' | 'k' => "k",
            'q' => {
                if next == Some(&'u') {
                    i += 1;
                }
                "k"
            }
            'g' if next == Some(&'u') && is_front(l.get(i + 1)) => {
                i += 1; // gue, gui: a silent u (güe keeps it, as a vowel)
                "g"
            }
            'g' if is_front(next) => "x",
            'g' => "g",
            'j' => "x",
            'l' if next == Some(&'l') => {
                i += 1;
                "ʝ"
            }
            'l' => "l",
            'r' if next == Some(&'r') => {
                i += 1;
                "r"
            }
            'r' if at == 0 || (at > 0 && matches!(l[at - 1], 'n' | 'l' | 's')) => "r",
            'r' => "ɾ",
            'y' if next.and_then(|c| vowel_of(*c)).is_some() => "ʝ",
            'y' => {
                // y alone or after a vowel is the vowel i (y, hoy, muy)
                out.push(Unit { ipa: "i", letter: at, vowel: Some((false, false)) });
                continue;
            }
            'x' if at == 0 => "s",
            'x' => {
                out.push(cons("k", at));
                "s"
            }
            'z' => theta,
            'ñ' => "ɲ",
            'b' | 'v' => "b",
            'd' => "d",
            'f' => "f",
            'm' => "m",
            'n' => "n",
            'p' => "p",
            's' => "s",
            't' => "t",
            'w' => "w",
            _ => continue,
        };
        out.push(cons(ipa, at));
    }
    out
}

/// n takes the place of the consonant after it; b, d, g soften except after a pause (`prev`
/// None), a nasal, or (d) l. `prev`: the sound before the word.
fn assimilate(u: &mut [Unit], prev: Option<&str>) {
    for i in 0..u.len() {
        if u[i].ipa == "n" {
            match u.get(i + 1).map(|n| n.ipa) {
                Some("p" | "b" | "m") => u[i].ipa = "m",
                Some("k" | "g" | "x") => u[i].ipa = "ŋ",
                _ => {}
            }
        }
    }
    for i in 0..u.len() {
        let before = if i == 0 { prev } else { Some(u[i - 1].ipa) };
        let Some(before) = before else { continue };
        let hard = matches!(before, "m" | "n" | "ŋ" | "ɲ") || (u[i].ipa == "d" && before == "l");
        if !hard {
            u[i].ipa = match u[i].ipa { "b" => "β", "d" => "ð", "g" => "ɣ", s => s };
        }
    }
}

/// A word's sounds as segments, each with its letter; and per syllable nucleus whether its vowels
/// carry an accent mark. Vowels next to each other group into one nucleus unless both are strong:
/// the strong one (or, of two weak ones, the second) is the nucleus and the others glide.
fn segments(l: &[char], prev: Option<&str>) -> (Vec<(Segment, usize)>, Vec<bool>) {
    let mut u = units(l);
    assimilate(&mut u, prev);
    let mut segs: Vec<(Segment, usize)> = Vec::new();
    let mut accents = Vec::new();
    let mut i = 0;
    while i < u.len() {
        if u[i].vowel.is_none() {
            let mut s = Segment::new(u[i].ipa);
            if matches!(u[i].ipa, "p" | "t" | "k" | "tʃ") {
                s.aspirated = Some(false);
            }
            segs.push((s, u[i].letter));
            i += 1;
            continue;
        }
        // a run of vowels → groups, each one syllable
        let mut j = i;
        while j < u.len() && u[j].vowel.is_some() {
            j += 1;
        }
        let mut groups: Vec<Vec<&Unit>> = Vec::new();
        for v in &u[i..j] {
            let strong = v.vowel.unwrap().0;
            let join = groups.last().is_some_and(|g| !(strong && g.iter().any(|x| x.vowel.unwrap().0)) && g.len() < 3);
            if join {
                groups.last_mut().unwrap().push(v);
            } else {
                groups.push(vec![v]);
            }
        }
        for g in groups {
            let core = g.iter().position(|x| x.vowel.unwrap().0).unwrap_or(g.len() - 1);
            for (k, v) in g.iter().enumerate() {
                let ipa = if k == core { v.ipa } else if v.ipa == "i" { "j" } else { "w" };
                segs.push((Segment::new(ipa), v.letter));
            }
            accents.push(g.iter().any(|x| x.vowel.unwrap().1));
        }
        i = j;
    }
    (segs, accents)
}

fn is_nucleus(s: &Segment) -> bool {
    matches!(s.ipa.as_str(), "a" | "e" | "i" | "o" | "u")
}

/// Onsets: one consonant (and a glide), or p b t d k g f + l or r (and a glide), but not tl, dl.
pub fn is_onset(cluster: &[Segment]) -> bool {
    let ipa: Vec<&str> = cluster.iter().map(|s| s.ipa.as_str()).collect();
    let core = match ipa.split_last() {
        Some((g, rest)) if (*g == "j" || *g == "w") && !rest.is_empty() => rest,
        _ => &ipa[..],
    };
    match core {
        [] | [_] => true,
        [a, b] => {
            let obstruent = matches!(*a, "p" | "b" | "β" | "t" | "d" | "ð" | "k" | "g" | "ɣ" | "f");
            obstruent && matches!(*b, "l" | "ɾ") && !(matches!(*a, "t" | "d" | "ð") && *b == "l")
        }
        _ => false,
    }
}

/// Unstressed one-syllable words: articles, prepositions, pronouns before a verb, conjunctions.
#[rustfmt::skip]
const CLITICS: &[&str] = &[
    "el", "la", "los", "las", "lo", "un", "de", "del", "a", "al", "en", "con", "por", "sin", "y", "e", "o", "u", "ni",
    "que", "se", "me", "te", "le", "les", "nos", "os", "mi", "tu", "su", "mis", "tus", "sus", "si", "pues",
];

/// One sung syllable of a word, before stress.
struct Part {
    onset: Vec<Segment>,
    nucleus: Segment,
    coda: Vec<Segment>,
    text: String,
    accent: bool,
}

/// One written chunk (a word, or one hyphenated part of it) → syllables with their spelling.
fn read(chars: &[char], prev: Option<&str>) -> Vec<Part> {
    let (segs, accents) = segments(chars, prev);
    let plain: Vec<Segment> = segs.iter().map(|(s, _)| s.clone()).collect();
    let sy = syllabify_with(&plain, is_onset);
    // each syllable's text starts at its first sound's letter, with a silent h before it (a-ho-ra)
    let mut starts = Vec::new();
    let mut k = 0;
    for s in &sy {
        let mut at = if starts.is_empty() { 0 } else { segs[k].1 };
        while at > 0 && chars[at - 1] == 'h' && !(at >= 2 && chars[at - 2] == 'c') && starts.last().is_some_and(|&p| at - 1 > p) {
            at -= 1;
        }
        starts.push(at);
        k += s.onset.len() + 1 + s.coda.len();
    }
    starts.push(chars.len());
    let ordered = starts.windows(2).all(|w| w[0] < w[1]);
    sy.into_iter()
        .enumerate()
        .map(|(n, s)| Part {
            text: if ordered { chars[starts[n]..starts[n + 1]].iter().collect() } else if n == 0 { chars.iter().collect() } else { "~".into() },
            accent: accents.get(n).copied().unwrap_or(false),
            onset: s.onset,
            nucleus: s.nucleus,
            coda: s.coda,
        })
        .collect()
}

/// The stressed syllable of a word written `clean` (lowercase letters) read as `parts`.
fn stress_of(clean: &[char], parts: &[Part]) -> Option<usize> {
    if let Some(k) = parts.iter().position(|p| p.accent) {
        return Some(k);
    }
    let n = parts.len();
    if n == 1 {
        let w: String = clean.iter().collect();
        return (!CLITICS.contains(&w.as_str())).then_some(0);
    }
    let last = clean.last().copied().unwrap_or('a');
    let paroxytone = vowel_of(last).is_some() || matches!(last, 'n' | 's');
    Some(if paroxytone { n - 2 } else { n - 1 })
}

/// One word → segments, stress marked on its vowels (for tests and the CLI).
pub fn word(input: &str) -> Vec<Segment> {
    word_syllables(input, None)
        .into_iter()
        .flat_map(|s| s.onset.into_iter().chain(std::iter::once(s.nucleus)).chain(s.coda))
        .collect()
}

/// The last sound of a word as said alone (the context for the next word's b, d, g).
fn last_sound(word: &str) -> Option<String> {
    let chars = normalize(word.split('-').next_back().unwrap_or(word));
    let (segs, _) = segments(&chars, None);
    segs.last().map(|(s, _)| s.ipa.clone())
}

fn word_syllables(tok: &str, prev: Option<&str>) -> Vec<Syllable> {
    // typed hyphens split syllables explicitly (pi-a-no keeps the hiatus); stress is the whole word's
    let pieces: Vec<Vec<char>> = tok.split('-').map(normalize).filter(|p| !p.is_empty()).collect();
    let clean: Vec<char> = pieces.concat();
    let mut parts: Vec<Part> = Vec::new();
    let mut before: Option<String> = prev.and_then(last_sound);
    for piece in &pieces {
        let mut p = read(piece, before.as_deref());
        if pieces.len() > 1 && p.len() > 1 {
            for (k, part) in p.iter_mut().enumerate() {
                part.text = if k == 0 { piece.iter().collect() } else { "~".into() };
            }
        }
        before = p.last().map(|s| s.coda.last().unwrap_or(&s.nucleus).ipa.clone());
        parts.append(&mut p);
    }
    let stress = stress_of(&clean, &parts);
    // the spelling as typed (case kept) when it lines up with the lowercased letters
    let typed: Vec<char> = tok.chars().filter(|c| is_letter(*c)).collect();
    let keep_case = typed.len() == clean.len();
    let mut at = 0;
    parts
        .into_iter()
        .enumerate()
        .map(|(k, p)| {
            let mut nucleus = p.nucleus;
            debug_assert!(is_nucleus(&nucleus));
            nucleus.stress = Some(u8::from(stress == Some(k)));
            let n = p.text.chars().count();
            let text = if keep_case && p.text != "~" && at + n <= typed.len() && typed[at..at + n].iter().flat_map(|c| c.to_lowercase()).eq(p.text.chars()) {
                typed[at..at + n].iter().collect()
            } else {
                p.text.clone()
            };
            if p.text != "~" {
                at += n;
            }
            Syllable { text, onset: p.onset, nucleus, coda: p.coda, ..Default::default() }
        })
        .collect()
}

/// The Spanish (Spain) front end for sung lyrics.
pub struct Spanish;

impl FrontEnd for Spanish {
    fn starts(&self, c: char) -> bool {
        is_letter(c)
    }
    fn continues(&self, c: char) -> bool {
        is_letter(c) || is_mark(c) || c == '-'
    }
    fn word(&self, word: &str) -> Vec<Syllable> {
        word_syllables(word, None)
    }
    fn word_after(&self, word: &str, prev: Option<&str>) -> Vec<Syllable> {
        word_syllables(word, prev)
    }
}

fn is_phrase_punct(c: char) -> bool {
    matches!(c, '.' | '!' | '?' | ';' | ':' | ',' | '\n')
}

/// Question words: a question that starts with one falls at the end (¿Dónde estás?); a yes/no
/// question rises (¿Estás bien?).
const QUESTION_WORDS: &[&str] = &["qué", "quién", "quiénes", "cómo", "dónde", "adónde", "cuándo", "cuánto", "cuánta", "cuántos", "cuántas", "cuál", "cuáles", "por"];

/// Spanish vowel lengths (s, before the speaking rate): stressed and unstressed differ little
/// (Spanish is syllable-timed); the phrase's last vowel is half again as long. A glide is short.
fn time_vowels(segs: &mut [Segment]) {
    let last = segs.iter().rposition(is_nucleus);
    for (i, s) in segs.iter_mut().enumerate() {
        if is_nucleus(s) {
            let d = if s.stress == Some(1) { 0.105 } else { 0.075 };
            s.dur = Some(if Some(i) == last { d * 1.5 } else { d });
        } else if s.ipa == "j" || s.ipa == "w" {
            s.dur = Some(0.05);
        }
    }
}

/// Spanish text → phrases for speech, split at punctuation like English's: each word read with
/// the one before it (b, d, g soften across words), word ends marked, Spanish vowel lengths.
/// Digits and other scripts are skipped.
pub fn to_phrases(text: &str) -> Vec<Phrase> {
    let mut phrases = Vec::new();
    let mut words: Vec<String> = Vec::new();
    let mut cur = String::new();
    let flush = |delim: Option<char>, words: &mut Vec<String>, phrases: &mut Vec<Phrase>| {
        if words.is_empty() {
            return;
        }
        let mut segs: Vec<Segment> = Vec::new();
        let mut said = Vec::new();
        for (i, w) in words.iter().enumerate() {
            let prev = i.checked_sub(1).map(|p| words[p].as_str());
            let mut ws: Vec<Segment> = word_syllables(w, prev)
                .into_iter()
                .flat_map(|s| s.onset.into_iter().chain(std::iter::once(s.nucleus)).chain(s.coda))
                .collect();
            let Some(last) = ws.last_mut() else { continue };
            last.word_end = Some(true);
            segs.append(&mut ws);
            said.push(w.clone());
        }
        time_vowels(&mut segs);
        let first: String = normalize(&words[0]).into_iter().collect();
        let wh = QUESTION_WORDS.contains(&first.as_str()) && (first != "por" || words.get(1).is_some_and(|w| normalize(w).into_iter().collect::<String>() == "qué"));
        let intonation = if delim == Some('?') && !wh { Intonation::Question } else { Intonation::Statement };
        let pause = match delim {
            Some('.' | '!' | '?' | '\n') => 0.35,
            Some(_) => 0.18,
            None => 0.3,
        };
        words.clear();
        if !segs.is_empty() {
            phrases.push(Phrase { segments: segs, intonation, pause, words: said });
        }
    };
    for c in text.replace('’', "'").chars() {
        if is_letter(c) || (is_mark(c) && !cur.is_empty()) || (c == '-' && !cur.is_empty()) {
            cur.push(c);
            continue;
        }
        if !cur.is_empty() {
            words.push(std::mem::take(&mut cur));
        }
        if is_phrase_punct(c) {
            flush(Some(c), &mut words, &mut phrases);
        }
    }
    if !cur.is_empty() {
        words.push(cur);
    }
    flush(None, &mut words, &mut phrases);
    phrases
}
