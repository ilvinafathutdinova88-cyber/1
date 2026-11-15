import { useEffect, useRef, useState } from "react";
import { useStore } from "../store.ts";
import { DEFAULT_LYRICS, useSing, playTune, stopSinging, soundingSyllable, sungNow, TUNES, ensureDictionary, arabicWordAt, lyricDisplayUnits } from "../singing.ts";
import { engine } from "../../audio/AudioEngine.ts";
import { canRecord, Clip } from "../../song/record.ts";
import { mouthWords } from "../mouthWords.ts";
import { V } from "../colors.ts";
import { Keys, MidiButton } from "./Keys.tsx";
import { ActionBar, PrimaryButton } from "./ActionBar.tsx";
import { Menu } from "./Controls.tsx";
import { PianoRoll } from "./PianoRoll.tsx";
import { SpectrumView } from "./SpectrumView.tsx";
import { Chip, Screen } from "./Screen.tsx";
import { EncoderPages } from "./Encoder.tsx";
import { noteName } from "./VoiceControls.tsx";
import type { InfoFn } from "./AnatomyView.tsx";
import { translate as t } from "../i18n.ts";
import { SAWT_SAFIR_AL_BULBUL_ID, SAWT_SAFIR_AL_BULBUL_LYRICS, TALA_AL_BADRU_ID, TALA_AL_BADRU_LYRICS } from "../../song/tunes.ts";

/** epilarynx narrowing for the singer's ring (measured: +3 to +12 dB at 2.4–3.6 kHz, adult male) */
const RING = 0.85;

const LYRIC_EXAMPLES = [
  { value: "dawn", label: "Arabic song: dawn", tune: "dreamy", text: DEFAULT_LYRICS },
  { value: "night", label: "Arabic song: night", tune: "somber", text: "يَا لَيْلُ يَا عَيْنُ\nغَنِّي مَعِي وَاسْمَعْ صَوْتِي" },
  { value: "garden", label: "Arabic song: garden", tune: "joyful", text: "فِي الْحَدِيقَةِ زَهْرَةٌ\nتَحْكِي لَنَا لَوْنَ الرَّبِيعِ" },
  { value: "home", label: "Arabic song: home", tune: "gentle", text: "بَيْتِي دَفِيءٌ وَالْقُلُوبُ قَرِيبَةٌ\nفِيهِ الْمَوَدَّةُ وَالسَّلَامُ" },
  { value: TALA_AL_BADRU_ID, label: "Nasheed: Tala al-Badru", tune: TALA_AL_BADRU_ID, text: TALA_AL_BADRU_LYRICS },
  { value: SAWT_SAFIR_AL_BULBUL_ID, label: "Poem: Sawt Safir al-Bulbul", tune: SAWT_SAFIR_AL_BULBUL_ID, text: SAWT_SAFIR_AL_BULBUL_LYRICS },
  { value: "morning-en", label: "English song: morning", tune: "dreamy", text: "Morning light is on the wall\nWake and hear the songbirds call" },
  { value: "night-en", label: "English song: night", tune: "somber", text: "Softly evening fills the sky\nLittle stars are shining high" },
];

/** the syllable being sung now: the tune's, or the one the held key is singing */
function syllableNow(): string | null {
  const s = useSing.getState();
  if (s.playing) return arabicWordAt(s.syllables, s.at)?.text ?? s.syllables[s.at]?.text ?? null;
  return soundingSyllable();
}

/** the syllable being sung, at the mouth */
export const singWords = mouthWords(sungNow, V.green);

/** Sing mode's lines over the face: the note, the syllable, the ring */
export const singInfo: InfoFn = (_m, _f, tele) => {
  const out: string[] = [];
  if (tele && tele.t.voicing > 0.3) out.push(`Note ${noteName(tele.t.f0)} ${tele.t.f0.toFixed(0)} Hz`);
  const syl = syllableNow();
  if (syl) out.push(`${/[\u0600-\u06ff\u0750-\u077f\u08a0-\u08ff]/u.test(syl) ? "Word" : "Syllable"} ${syl}`);
  out.push(useStore.getState().artic.epilarynx >= RING - 0.05 ? "Ring on" : "Ring off");
  return out;
};

function Lyrics() {
  const lyrics = useSing((s) => s.lyrics);
  const syllables = useSing((s) => s.syllables);
  const cursor = useSing((s) => s.cursor);
  const demo = useSing((s) => s.playing);
  const sungAt = useSing((s) => s.at);
  const setLyrics = useSing((s) => s.setLyrics);
  const setCursor = useSing((s) => s.setCursor);
  const at = demo ? sungAt : -1;
  const done = demo ? sungAt : cursor;
  const direction = /[\u0600-\u06ff\u0750-\u077f\u08a0-\u08ff]/u.test(lyrics) ? "rtl" : "ltr";
  const units = lyricDisplayUnits(syllables);
  const language = useStore((s) => s.language);
  return (
    <div className="lyrics">
      <textarea
        value={lyrics}
        dir={direction}
        onChange={(e) => setLyrics(e.target.value)}
        onKeyDown={(e) => e.stopPropagation()}
        rows={Math.min(4, lyrics.split("\n").length)}
        aria-label={t("Lyrics to sing", language)}
        placeholder={t("Type the words to sing", language)}
        title="Sing sings these words to the tune, a line to each of its phrases. Each key sings the next syllable; hold a key while you press the next one to sing legato. A lone “-” between words holds a vowel over several notes; hyphens split a word your way (“ti-ger”)."
      />
      <div className="syllables" dir={direction} role="group" aria-label="Pronunciation units. Each note sings the next syllable; Arabic words are grouped.">
        {units.map((unit) => {
          const current = at >= unit.start && at <= unit.end;
          const next = !demo && cursor >= unit.start && cursor <= unit.end;
          const sung = unit.end < done;
          return (
            <span key={unit.start} className={`syl-wrap${unit.wordEnd ? " word-end" : ""}`}>
              <button
                className={`syl${next ? " next" : ""}${current ? " now" : ""}${sung ? " sung" : ""}${unit.melisma ? " held" : ""}`}
                onClick={() => setCursor(unit.start)}
                aria-current={next ? "step" : undefined}
                title={unit.melisma ? "Held vowel: carries the previous syllable onto this note" : `Sing from “${unit.text}”`}
              >
                {unit.text}
              </button>
              {unit.phraseEnd && (
                <span className="breath-mark" aria-label="breath">
                  ’
                </span>
              )}
            </span>
          );
        })}
        {!syllables.length && <span className="dim">{t("No words: the keys sing “ah”.", language)}</span>}
        <Chip toggle={false} className="from-start" onClick={() => setCursor(0)} disabled={cursor === 0 && !demo}>
          From the start
        </Chip>
      </div>
    </div>
  );
}

function SongControls() {
  const source = useStore((s) => s.source);
  const setSource = useStore((s) => s.setSource);
  const epilarynx = useStore((s) => s.artic.epilarynx);
  const setArtic = useStore((s) => s.setArtic);
  const vibrato = useSing((s) => s.vibrato);
  const setVibrato = useSing((s) => s.setVibrato);
  const tempo = useSing((s) => s.tempo);
  const setTempo = useSing((s) => s.setTempo);
  const key = useSing((s) => s.key);
  const setKey = useSing((s) => s.setKey);
  return (
    <EncoderPages
      label="Song voice"
      pages={[
        {
          name: "Song",
          encoders: [
            { label: "Vibrato", value: vibrato, min: 0, max: 80, onChange: (v) => setVibrato(Math.round(v)), format: (v) => ({ value: String(Math.round(v)), unit: "cents" }) },
            { label: "Rate", value: source.vibratoRate, min: 3.5, max: 7.5, onChange: (v) => setSource({ vibratoRate: v }), format: (v) => ({ value: v.toFixed(1), unit: "Hz" }) },
            { label: "Breathy", value: source.breathiness, min: 0, max: 1, onChange: (v) => setSource({ breathiness: v }), format: (v) => ({ value: String(Math.round(v * 100)), unit: "%" }) },
            // the singer's ring narrows the tube just above the folds, adding the bright band near 3 kHz
            { label: "Ring", value: epilarynx, min: 0, max: 1, onChange: (v) => setArtic({ epilarynx: v }), format: (v) => ({ value: String(Math.round(v * 100)), unit: "%" }) },
          ],
        },
        {
          // the tune's speed and key, from the next time Sing is pressed
          name: "Tune",
          encoders: [
            { label: "Tempo", value: tempo, min: 0.5, max: 1.5, onChange: (v) => setTempo(Math.round(v * 20) / 20), format: (v) => ({ value: String(Math.round(v * 100)), unit: "%" }) },
            { label: "Key", value: key, min: -12, max: 12, onChange: (v) => setKey(Math.round(v)), format: (v) => ({ value: (v > 0.5 ? "+" : "") + String(Math.round(v)), unit: "semitones" }) },
          ],
        },
        {
          name: "Tone",
          encoders: [{ label: "Bright", value: source.brightness, min: 0, max: 1, onChange: (v) => setSource({ brightness: v }), format: (v) => ({ value: String(Math.round(v * 100)), unit: "%" }) }],
        },
      ]}
    />
  );
}

/**
 * Sing's main button: sing the lyrics to the chosen tune, or stop. A song getting ready (audio
 * starting, the dictionary loading) counts as playing: Stop cancels it, and choosing another tune
 * replaces it. Record video sings it again into a clip of the face screen (Share and Save once it's done).
 */
function SingActions() {
  const start = useStore((s) => s.start);
  const language = useStore((s) => s.language);
  const arabicDialect = useSing((s) => s.arabicDialect);
  const setArabicDialect = useSing((s) => s.setArabicDialect);
  const playing = useSing((s) => s.playing);
  const tune = useSing((s) => s.tune);
  const setTune = useSing((s) => s.setTune);
  const [pending, setPending] = useState(false);
  const [recording, setRecording] = useState(false);
  const [clip, setClip] = useState<{ file: File; url: string } | null>(null);
  /** bumped by every start and stop: a start still waiting no longer counts */
  const request = useRef(0);
  /** bumped by every Record video: a clip still being finished for an earlier one isn't offered */
  const take = useRef(0);
  /** the clip recording now, if any (one at a time) */
  const recorder = useRef<Clip | null>(null);
  // the audio must be running: if it failed to start, nothing is planned and the error shows
  const ready = async () => {
    await start();
    if (!useStore.getState().started) throw new Error("no audio");
  };
  const drop = () => {
    recorder.current?.cancel();
    recorder.current = null;
    setRecording(false);
  };
  // the clip's life is the tune's run: it starts as the tune is sent (never for a start that was
  // cancelled), is saved when it's sung to the end, and goes when it's stopped from anywhere
  const sing = (id: string, record = false) => {
    const n = ++request.current;
    const k = record ? ++take.current : take.current;
    drop();
    setPending(true);
    let mine: Clip | null = null;
    const hooks = !record
      ? {}
      : {
          onStart: () => {
            const face = () => document.querySelector<HTMLCanvasElement>(".screen.left canvas");
            const audio = engine.recordStream();
            if (n !== request.current || !face() || !audio) throw new Error("not recording");
            mine = recorder.current = new Clip(face, audio);
            setRecording(true);
            setClip(null);
          },
          onEnd: () => {
            const c = mine;
            if (!c || recorder.current !== c) return;
            recorder.current = null;
            setRecording(false);
            c.stop(`chimera-${id}`)
              .then((file) => take.current === k && setClip({ file, url: URL.createObjectURL(file) }))
              .catch((e) => console.warn("Record video:", e));
          },
          onStop: () => {
            if (mine && recorder.current === mine) drop();
          },
        };
    // a clip waits for the last sound (the song this one replaces) to die away, or it opens on its tail
    const go = record ? async () => (await ready(), await engine.quiet()) : ready;
    playTune(id, useStore.getState().body, go, hooks)
      .catch(() => {})
      .finally(() => request.current === n && setPending(false));
  };
  const stop = () => {
    request.current++; // the song getting ready, if any, no longer counts
    setPending(false);
    stopSinging(); // its run's onStop drops the clip
    drop();
  };
  useEffect(() => () => void (clip && URL.revokeObjectURL(clip.url)), [clip]);
  // leaving Sing: nothing still starting may begin a clip, and one recording goes
  useEffect(
    () => () => {
      request.current++;
      take.current++;
      recorder.current?.cancel();
      recorder.current = null;
    },
    [],
  );
  const press = () => (playing || pending ? stop() : sing(tune));
  const share = clip && typeof navigator.canShare === "function" && navigator.canShare({ files: [clip.file] });
  return (
    <ActionBar label="Song" onSpace={press}>
      <PrimaryButton label="Sing" playing={(!!playing || pending) && !recording} busy={recording ? "Recording…" : null} onClick={press} />
      <Menu
        label="Tune"
        icon="note"
        value={tune}
        options={TUNES.map((t) => ({ value: t.id, label: t.name }))}
        onChange={(id) => {
          setTune(id);
          if ((playing || pending) && !recording) sing(id);
        }}
      />
      <Menu
        label="Lyrics examples"
        value=""
        options={[
          { value: "", label: language === "ar" ? "أمثلة عربية" : "Arabic examples", disabled: true },
          ...LYRIC_EXAMPLES.filter(({ value }) => (language === "ar" ? !value.endsWith("-en") : value.endsWith("-en"))).map(({ value, label }) => ({ value, label })),
        ]}
        onChange={(value) => {
          const example = LYRIC_EXAMPLES.find((item) => item.value === value);
          if (example) {
            setTune(example.tune);
            useSing.getState().setLyrics(example.text);
          }
        }}
      />
      <Menu
        label="Arabic pronunciation"
        value={arabicDialect}
        options={[{ value: "egyptian", label: "Egyptian" }, { value: "msa", label: "Standard Arabic" }]}
        onChange={setArabicDialect}
      />
      {canRecord() && (
        <Chip toggle={false} onClick={() => (recording ? stop() : sing(tune, true))} title="Sing it again and record the face into a video">
          {recording ? "Cancel" : "Record video"}
        </Chip>
      )}
      {clip && share && (
        <Chip toggle={false} tone="amber" onClick={() => void navigator.share({ files: [clip.file] }).catch(() => {})}>
          Share
        </Chip>
      )}
      {clip && (
        <a className="chip" href={clip.url} download={clip.file.name}>
          Save
        </a>
      )}
    </ActionBar>
  );
}

/** Sing mode's right column: the song button, the pitch roll, the lyrics, the spectrum with the ring, the song voice, the keys. */
export function SingRight() {
  // the pronouncing dictionary for the lyrics: here rather than on the mode switch, since the app
  // opens on Sing without one
  useEffect(ensureDictionary, []);
  const language = useStore((s) => s.language);
  const lyrics = useSing((s) => s.lyrics);
  const ring = useStore((s) => s.artic.epilarynx >= RING - 0.05);
  const next = useSing((s) => (s.playing ? null : (s.syllables[s.cursor]?.text ?? null)));
  useEffect(() => {
    if (!LYRIC_EXAMPLES.some((example) => example.text === lyrics)) return;
    const example = LYRIC_EXAMPLES.find(({ value }) => (language === "ar" ? !value.endsWith("-en") : value.endsWith("-en")));
    if (example) {
      useSing.getState().setTune(example.tune);
      useSing.getState().setLyrics(example.text);
    }
  }, [language]);
  return (
    <div className="right right-sing">
      <SingActions />
      <Screen className="s-roll" title="Pitch">
        <PianoRoll />
      </Screen>
      <Screen className="s-lyrics" title="Lyrics">
        <Lyrics />
      </Screen>
      <Screen className="s-spec" title="Spectrum" sub={ring ? "Ring on" : ""}>
        <SpectrumView ring={ring} />
      </Screen>
      <Screen className="s-voice" title="Voice">
        <SongControls />
      </Screen>
      <Screen className="s-keys" title="Keys" sub={next ? `Next: ${next}` : ""} tools={<MidiButton />}>
        <Keys />
      </Screen>
    </div>
  );
}
