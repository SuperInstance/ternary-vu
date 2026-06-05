# ternary-vu

**Metering for ternary audio. Peak, RMS, crest factor — the vital signs of your signal.**

Every mixing console has meters. They tell you what your ears can't: is the signal too loud? Too quiet? Is it transient (high crest factor, like drums) or sustained (low crest factor, like a synth pad)? Is it clipping? The meters are the doctor's chart — objective vital signs for subjective sound.

This crate implements audio metering for ternary signals: peak detection (maximum absolute value), RMS (the "average loudness"), crest factor (peak/RMS — the dynamic range indicator), clipping detection, and zero-crossing rate (how often the signal changes sign, a proxy for frequency content).

## What's Inside

- **`VuMeter`** — tracks peak, RMS, and sample count across a signal
- **`peak(signal)`** — maximum absolute value (0 or 1 for ternary)
- **`rms(signal)`** — root mean square. The "average energy"
- **`crest_factor(signal)`** — peak/RMS ratio. Drums ≈ 4-6. Sustained pad ≈ 1.0
- **`clip_count(signal)`** — how many samples are at ±1 (maximum level)
- **`headroom(signal)`** — how much room before everything is ±1
- **`zero_crossing_rate(signal)`** — how often the signal passes through 0 (frequency proxy)
- **`DynamicRange`** — the difference between loudest and quietest moments

## Quick Example

```rust
use ternary_vu::*;

let drums = vec![1, 0, 0, 0, -1, 0, 0, 0, 1, 0, 0, 0, -1, 0, 0, 0];
let pad = vec![1, 1, 1, 1, -1, -1, -1, -1, 1, 1, 1, 1, -1, -1, -1, -1];

println!("Drums RMS: {:.3}", rms(&drums));  // Low RMS — mostly silent
println!("Pad RMS: {:.3}", rms(&pad));      // High RMS — always active

println!("Drums crest: {:.2}", crest_factor(&drums));  // High — transient
println!("Pad crest: {:.2}", crest_factor(&pad));      // ~1.0 — sustained

println!("Drums ZCR: {}", zero_crossing_rate(&drums)); // Low — slow changes
println!("Pad ZCR: {}", zero_crossing_rate(&pad));     // Higher — more transitions
```

## The Deeper Truth

**Ternary metering is quantized but decisive.** In continuous audio, RMS varies smoothly from 0 to 1. In ternary, it can only take specific values determined by the ratio of {-1, 0, +1} in the signal window. A signal that's ⅓ each state has RMS = √(2/3) ≈ 0.816. A signal that's all ±1 has RMS = 1.0. A signal that's all 0 has RMS = 0.0. There are only a few possible meter readings — and each one tells you exactly what's happening.

The crest factor is the most musical meter: it distinguishes between *how* a signal is loud. Two signals with the same peak can have completely different crest factors. A drum hit (brief, loud, surrounded by silence) has high crest. A distorted guitar (constantly loud) has low crest. The crest factor tells you whether the signal is *dynamic* (interesting) or *compressed* (flat). In mixing, it's the single most important number for deciding what needs compression or expansion.

The zero-crossing rate is a secret weapon for genre detection: high ZCR = bright/noisy (hi-hats, distortion), low ZCR = dark/smooth (bass, pads). In ternary, it's the easiest proxy for spectral content without doing an actual FFT.

**Use cases:**
- **Audio metering** — measure signal levels in ternary audio chains
- **Mixing** — balance levels between tracks using RMS and peak
- **Dynamic processing** — set gate/compressor thresholds using crest factor
- **Signal health** — detect clipping (all ±1) or silence (all 0)
- **Genre detection** — ZCR as a timbre classifier

## See Also

- **ternary-gate** — gating decisions based on VU measurements
- **ternary-mixer** — mixing multiple signals (needs metering to avoid clipping)
- **ternary-echo** — echo changes the RMS and crest factor
- **ternary-bite** — degradation changes all meter readings
- **ternary-rack** — meter between processing stages in the signal chain

## Install

```bash
cargo add ternary-vu
```

## License

MIT
