use byteorder::{LittleEndian, WriteBytesExt};
use std::fs::File;
use std::iter::zip;

type Seconds = f64;
type Samples = i32;
type Hz = f64;
type Pulse = f64;
type Semitone = i64;
type Beats = u64;

fn save_file(v: Vec<f64>, f: &mut std::fs::File) -> std::io::Result<()> {
    for x in v {
        f.write_f64::<LittleEndian>(x)?;
    }
    Ok(())
}

fn semitone_to_hz(semitone: Semitone) -> Hz {
    440.0 * 2.0_f64.powf(semitone as f64 / 12.0)
}

fn sine_wave(hz: Hz, sample_rate: Samples, duration: Seconds, volume: f64) -> Vec<Pulse> {
    let step = hz * 2.0 * std::f64::consts::PI / sample_rate as f64;
    (0..(sample_rate as f64 * duration) as i64)
        .map(|x| x as f64 * step)
        .map(|x| x.sin())
        .map(|x| x * volume)
        .collect()
}

fn apply_attack_decay(wave: Vec<Pulse>) -> Vec<Pulse> {
    let min_attack = |i: usize| -> f64 {
        let res = i as f64 / 1000.0;
        if res < 1.0 {
            res
        } else {
            1.0
        }
    };

    let min_decay = |i: usize| -> f64 {
        let res = (wave.len() - i) as f64 / 1000.0;
        if res < 1.0 {
            res
        } else {
            1.0
        }
    };

    zip(wave.iter(), 0..)
        .map(|(x, i)| *x * min_attack(i) * min_decay(i))
        .collect()
}

fn make_note(n: Semitone, sample_rate: Samples, beats: f64, volume: f64, bpm: Beats) -> Vec<Pulse> {
    let hz = semitone_to_hz(n as i64);
    let beat_duration: Seconds = 60.0 / bpm as f64;
    let wave = sine_wave(hz, sample_rate, beat_duration * beats as f64, volume);
    apply_attack_decay(wave)
}

fn str_to_semitone(st: &str) -> Semitone {
    match st {
        "C" => 0,
        "C#" => 1,
        "D" => 2,
        "D#" => 3,
        "E" => 4,
        "F" => 5,
        "F#" => 6,
        "G" => 7,
        "G#" => 8,
        "A" => 9,
        "A#" => 10,
        "B" => 11,
        _ => panic!("Invalid note"),
    }
}

fn main() {
    let bpm = 113;
    let volume = 0.4;
    let sample_rate: Samples = 48000;

    // https://www.irish-folk-songs.com/never-gonna-give-you-up-easy-sheet-music-and-piano-letter-notes.html
    let wave = vec![
        make_note(str_to_semitone("D"), sample_rate, 0.5, volume, bpm),
        make_note(str_to_semitone("E"), sample_rate, 0.5, volume, bpm),
        make_note(str_to_semitone("G"), sample_rate, 0.5, volume, bpm),
        make_note(str_to_semitone("E"), sample_rate, 0.5, volume, bpm),
        make_note(str_to_semitone("B"), sample_rate, 1., volume, bpm),
        make_note(str_to_semitone("B"), sample_rate, 1., volume, bpm),
        make_note(str_to_semitone("A"), sample_rate, 1., volume, bpm),
        make_note(str_to_semitone("D"), sample_rate, 0.5, volume, bpm),
        make_note(str_to_semitone("E"), sample_rate, 0.5, volume, bpm),
        make_note(str_to_semitone("G"), sample_rate, 0.5, volume, bpm),
        make_note(str_to_semitone("E"), sample_rate, 0.5, volume, bpm),
        make_note(str_to_semitone("A"), sample_rate, 1., volume, bpm),
        make_note(str_to_semitone("A"), sample_rate, 1., volume, bpm),
        make_note(str_to_semitone("G"), sample_rate, 1., volume, bpm),

        make_note(str_to_semitone("D"), sample_rate, 0.5, volume, bpm),
        make_note(str_to_semitone("E"), sample_rate, 0.5, volume, bpm),
        make_note(str_to_semitone("G"), sample_rate, 0.5, volume, bpm),
        make_note(str_to_semitone("E"), sample_rate, 0.5, volume, bpm),
        make_note(str_to_semitone("B"), sample_rate, 1., volume, bpm),
        make_note(str_to_semitone("A"), sample_rate, 1., volume, bpm),
        make_note(str_to_semitone("F#"), sample_rate, 1., volume, bpm),
        make_note(str_to_semitone("D"), sample_rate, 0.5, volume, bpm),
        make_note(str_to_semitone("D"), sample_rate, 0.5, volume, bpm),
        make_note(str_to_semitone("A"), sample_rate, 1., volume, bpm),
        make_note(str_to_semitone("G"), sample_rate, 1., volume, bpm),
        // make_note(str_to_semitone("E"), sample_rate, 1., volume, bpm),
        // make_note(str_to_semitone("F#"), sample_rate, 1., volume, bpm),
    ]
    .into_iter()
    .flatten()
    .collect();

    let mut f = File::create("test.pcm").unwrap();

    save_file(wave, &mut f).unwrap();
}
