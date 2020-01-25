//! Audio library.

// TODO:Frequency is a basic component of the oscillator. Which basic type is
// better to use.
// pub struct Frequency(f64);

// TODO: Differnet frequency measurement units
// pub type Hz = Frequency;

// TODO:Technically note can be considered as a basic pitch, accepting octave
// as an argument and modified by accidentals (sharp and flat), e.g. C(4).sharp.sharp.
// May be some unary operators can be used instead if sharp/flat.
// pub enum Note { C, D, E, F, G, A, H }

// TODO:
// pub struct Pitch;

// TODO: Default concert pitch can be set as a frequency. From this frequency
// the note A(4) is starting.
// pub const DEFAULT_CONCERT_PITCH: Frequency = Hz(440.0);

// TODO:Frequency of all notes can be defined as a distance from A4.
// pub fn note_frequency(distance_from_a4: i8, concert_pitch: f64) -> f64 {
//    2 ** (distance_from_a4 as f64 / 12 as f64) * concert_pitch
// }

// TODO:How is it better to define a pitch system, because A4 is not a constant
// frequency. It is volatile.
// pub struct PitchSystem {
//    pub concert_pitch: Frequency
// }

// TODO: Define main oscillator components (frequency, etc).
// Is the pitch system simply an oscillator implementation that itself is a f(t)?
// pub struct Oscillator;
