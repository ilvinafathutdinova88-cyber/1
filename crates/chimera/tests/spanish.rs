//! Spanish spelling → segments and sung syllables (`phonetics::spanish`), one case per rule.
use chimera::anatomy::human::BodyMix;
use chimera::phonetics::english::EnglishG2P;
use chimera::phonetics::planner::{plan_utterance, PlanOptions};
use chimera::phonetics::spanish;
use chimera::phonetics::syllables::{lyrics_to_syllables, lyrics_to_syllables_in, Latin};
use chimera::phonetics::{Segment, Syllable};

fn ipa(segs: &[Segment]) -> String {
    segs.iter().map(|s| s.ipa.as_str()).collect::<Vec<_>>().join(" ")
}
/// a word alone (after a pause), Spain
fn es(w: &str) -> String {
    ipa(&spanish::word(w))
}
fn sung(text: &str) -> Vec<Syllable> {
    lyrics_to_syllables_in(text, &EnglishG2P::new(), Latin::Spanish)
}
fn texts(text: &str) -> Vec<String> {
    sung(text).into_iter().map(|s| s.text).collect()
}
/// the syllable carrying the stress, by its text
fn stressed(text: &str) -> String {
    sung(text).into_iter().find(|s| s.nucleus.stress == Some(1)).map(|s| s.text).unwrap_or_default()
}

#[test]
fn letters_to_sounds() {
    assert_eq!(es("casa"), "k a s a");
    assert_eq!(es("cielo"), "θ j e l o");
    assert_eq!(es("chico"), "tʃ i k o");
    assert_eq!(es("queso"), "k e s o");
    assert_eq!(es("guitarra"), "g i t a r a");
    assert_eq!(es("pingüino"), "p i ŋ g w i n o");
    assert_eq!(es("gente"), "x e n t e");
    assert_eq!(es("jamón"), "x a m o n");
    assert_eq!(es("hola"), "o l a");
    assert_eq!(es("llave"), "ʝ a β e");
    assert_eq!(es("yo"), "ʝ o");
    assert_eq!(es("año"), "a ɲ o");
    assert_eq!(es("zapato"), "θ a p a t o");
    assert_eq!(es("taxi"), "t a k s i");
    assert_eq!(es("vaca"), "b a k a");
}

#[test]
fn tap_and_trill() {
    assert_eq!(es("pero"), "p e ɾ o");
    assert_eq!(es("perro"), "p e r o");
    assert_eq!(es("rosa"), "r o s a");
    assert_eq!(es("honra"), "o n r a");
    assert_eq!(es("Israel"), "i s r a e l");
}

#[test]
fn b_d_g_soften_except_after_a_pause_a_nasal_or_l() {
    assert_eq!(es("cada"), "k a ð a");
    assert_eq!(es("lobo"), "l o β o");
    assert_eq!(es("lago"), "l a ɣ o");
    assert_eq!(es("ambos"), "a m b o s");
    assert_eq!(es("andar"), "a n d a ɾ");
    assert_eq!(es("tengo"), "t e ŋ g o");
    assert_eq!(es("caldo"), "k a l d o");
    assert_eq!(es("algo"), "a l ɣ o");
    // across words: la boca softens, con bata doesn't, a phrase's start doesn't
    let s = sung("la boca, con bata. dedo");
    assert_eq!(ipa(&s[1].onset), "β");
    assert_eq!(ipa(&s[4].onset), "b");
    assert_eq!(ipa(&s[6].onset), "d");
    assert_eq!(ipa(&s[7].onset), "ð");
}

#[test]
fn n_takes_the_place_of_the_next_consonant() {
    assert_eq!(es("enviar"), "e m b j a ɾ");
    assert_eq!(es("banco"), "b a ŋ k o");
    assert_eq!(es("ángel"), "a ŋ x e l");
}

#[test]
fn stops_are_unaspirated() {
    for s in spanish::word("pato").iter().chain(&spanish::word("chico")) {
        if matches!(s.ipa.as_str(), "p" | "t" | "k" | "tʃ") {
            assert_eq!(s.aspirated, Some(false), "{}", s.ipa);
        }
    }
}

#[test]
fn diphthongs_glide_and_accents_break_them() {
    assert_eq!(texts("bien"), ["bien"]);
    assert_eq!(texts("aire"), ["ai", "re"]);
    assert_eq!(es("aire"), "a j ɾ e");
    assert_eq!(texts("ciudad"), ["ciu", "dad"]);
    assert_eq!(es("ciudad"), "θ j u ð a ð");
    assert_eq!(texts("cuidar"), ["cui", "dar"]);
    assert_eq!(es("muy"), "m w i");
    assert_eq!(es("hoy"), "o j");
    // hiatus: two strong vowels, or a weak one with an accent
    assert_eq!(texts("poeta"), ["po", "e", "ta"]);
    assert_eq!(texts("día"), ["dí", "a"]);
    assert_eq!(texts("baúl"), ["ba", "úl"]);
    assert_eq!(texts("oía"), ["o", "í", "a"]);
}

#[test]
fn syllables_split_like_spanish_spelling() {
    assert_eq!(texts("corazón"), ["co", "ra", "zón"]);
    assert_eq!(texts("perro"), ["pe", "rro"]);
    assert_eq!(texts("calle"), ["ca", "lle"]);
    assert_eq!(texts("noche"), ["no", "che"]);
    assert_eq!(texts("acción"), ["ac", "ción"]);
    assert_eq!(texts("hablar"), ["ha", "blar"]);
    assert_eq!(texts("otro"), ["o", "tro"]);
    assert_eq!(texts("atlas"), ["at", "las"]);
    assert_eq!(texts("ahora"), ["a", "ho", "ra"]);
    assert_eq!(texts("Sol"), ["Sol"]);
    assert_eq!(texts("Canción"), ["Can", "ción"]);
    assert_eq!(texts("guerra"), ["gue", "rra"]);
}

#[test]
fn stress_follows_the_spelling() {
    // an accent mark wins
    assert_eq!(stressed("canción"), "ción");
    assert_eq!(stressed("lápiz"), "lá");
    assert_eq!(stressed("música"), "mú");
    // ending in a vowel, n or s: the last syllable but one
    assert_eq!(stressed("casa"), "ca");
    assert_eq!(stressed("cantan"), "can");
    assert_eq!(stressed("lunes"), "lu");
    // otherwise the last
    assert_eq!(stressed("cantar"), "tar");
    assert_eq!(stressed("ciudad"), "dad");
    assert_eq!(stressed("estoy"), "toy");
    // articles and prepositions carry none
    assert_eq!(sung("el")[0].nucleus.stress, Some(0));
    assert_eq!(sung("sol")[0].nucleus.stress, Some(1));
}

#[test]
fn decomposed_accents_are_read() {
    assert_eq!(texts("cancio\u{301}n"), ["can", "ción"]);
    assert_eq!(es("an\u{303}o"), "a ɲ o");
}

#[test]
fn hyphens_and_melisma() {
    // a hyphen keeps a hiatus the singer wants (pi-a-no), with the word's own stress
    let s = sung("pi-a-no");
    assert_eq!(s.iter().map(|s| s.text.as_str()).collect::<Vec<_>>(), ["pi", "a", "no"]);
    assert_eq!(s[1].nucleus.stress, Some(1));
    assert_eq!(texts("piano"), ["pia", "no"]);
    let s = sung("sol ~");
    assert_eq!(s.len(), 2);
    assert_eq!(s[1].melisma, Some(true));
}

#[test]
fn punctuation_and_inverted_marks() {
    let s = sung("¿Qué tal? ¡Hola!");
    assert_eq!(s.iter().map(|s| s.text.as_str()).collect::<Vec<_>>(), ["Qué", "tal", "Ho", "la"]);
    assert!(s[1].phrase_end && s[3].phrase_end);
    assert!(s.iter().all(|s| s.word_end || s.text == "Ho"));
}

#[test]
fn other_scripts_still_go_to_their_front_ends() {
    let s = sung("hola 별 ми́р");
    assert_eq!(s.iter().map(|s| s.text.as_str()).collect::<Vec<_>>(), ["ho", "la", "별", "ми́р"]);
}

#[test]
fn english_is_unchanged_by_default() {
    let g = EnglishG2P::new();
    assert_eq!(lyrics_to_syllables("twinkle little star", &g), lyrics_to_syllables_in("twinkle little star", &g, Latin::English));
    assert_eq!(Latin::from_code("es"), Some(Latin::Spanish));
    assert_eq!(Latin::from_code("es-419"), None);
    assert_eq!(Latin::from_code("fr"), None);
}

#[test]
fn every_sound_is_in_the_inventory_and_plans() {
    let text = "¿Dónde está la llave? Mañana, guitarra, pingüino, ciudad, acción, hoy, muy, jamás, taxi, Israel.";
    let segs: Vec<Segment> = sung(text).into_iter().flat_map(|s| s.onset.into_iter().chain(std::iter::once(s.nucleus)).chain(s.coda)).collect();
    for s in &segs {
        assert!(chimera::phonetics::inventory::phoneme(&s.ipa, BodyMix::MALE).is_some() || chimera::phonetics::planner::is_vowel(&s.ipa, BodyMix::MALE), "{} not in the inventory", s.ipa);
    }
    let plan = plan_utterance(&segs, &PlanOptions { body: BodyMix::MALE, ..Default::default() });
    assert!(plan.end > 1.0 && !plan.gestures.is_empty());
}

// ---------------------------------------------------------------- speech

fn said(text: &str) -> Vec<chimera::phonetics::english::Phrase> {
    spanish::to_phrases(text)
}

#[test]
fn speech_splits_phrases_and_marks_words() {
    let p = said("Hola, ¿qué tal? Bien.");
    assert_eq!(p.len(), 3);
    assert_eq!(p[0].words, ["Hola"]);
    assert_eq!(p[1].words, ["qué", "tal"]);
    assert_eq!(p[1].segments.iter().filter(|s| s.word_end == Some(true)).count(), 2);
    assert_eq!(ipa(&p[1].segments), "k e t a l");
    // a comma breathes less than a full stop
    assert!(p[0].pause < p[2].pause);
}

#[test]
fn questions_rise_unless_they_start_with_a_question_word() {
    use chimera::phonetics::Intonation;
    assert_eq!(said("¿Estás bien?")[0].intonation, Intonation::Question);
    assert_eq!(said("¿Dónde estás?")[0].intonation, Intonation::Statement);
    assert_eq!(said("¿Por qué no?")[0].intonation, Intonation::Statement);
    assert_eq!(said("¿Por la tarde?")[0].intonation, Intonation::Question);
    assert_eq!(said("Ya está.")[0].intonation, Intonation::Statement);
}

#[test]
fn speech_softens_across_words() {
    assert_eq!(ipa(&said("la boca")[0].segments), "l a β o k a");
    assert_eq!(ipa(&said("un beso")[0].segments), "u n b e s o");
    // a phrase starts afresh
    assert_eq!(said("sí. dedo")[1].segments[0].ipa, "d");
}

#[test]
fn unstressed_vowels_keep_their_length() {
    // Klatt's English rules would make these ~40 ms and swallow the syllable
    let p = said("la casa de mi madre");
    for s in &p[0].segments {
        if matches!(s.ipa.as_str(), "a" | "e" | "i" | "o" | "u") {
            assert!(s.dur.is_some_and(|d| d >= 0.07), "{} {:?}", s.ipa, s.dur);
        }
    }
    let inputs: Vec<_> = p.iter().map(|p| chimera::phonetics::planner::PhraseInput { segments: p.segments.clone(), intonation: Some(p.intonation), pause: Some(p.pause) }).collect();
    let plan = chimera::phonetics::planner::plan_phrases(&inputs, &PlanOptions { body: BodyMix::MALE, ..Default::default() });
    let vowels: Vec<f64> = plan.segments.iter().filter(|s| s.kind == "vowel").map(|s| s.t1 - s.t0).collect();
    assert_eq!(vowels.len(), 7);
    assert!(vowels.iter().all(|d| *d >= 0.065), "{vowels:?}");
}
