# ternary-vu

**VU meter for ternary audio signals. Peak, RMS, crest factor, clipping detection.**

The VU (Volume Unit) meter is the most basic audio measurement: how loud is the signal right now? This crate implements peak detection, RMS (root mean square), crest factor (peak/RMS ratio), and clipping detection for signals in {-1, 0, +1}.

In ternary, the VU meter has a unique property: the only possible peak values are 0 and 1. RMS is always 0, 1/√3, or √(2/3) depending on the signal distribution. The crest factor — normally a continuous value — takes on only a few discrete values. This makes ternary VU measurement *exact* and *fast*.

## What's Inside

- **`VuMeter`** — tracks peak, RMS, and sample count
- **`peak(signal)`** — maximum absolute value
- **`rms(signal)`** — root mean square. The "average loudness"
- **`crest_factor(signal)`** — peak/RMS. High crest = transient (drums), low crest = sustained (synth pad)
- **`clip_count(signal)`** — how many samples are at ±1 (clipped)
- **`headroom(signal)`** — how much room before clipping. In ternary: how far from all-±1
- **`zero_crossing_rate(signal)`** — how often the signal passes through 0

## Quick Example

```rust
use ternary_vu::*;

let signal = vec![1, 0, -1, 1, 0, -1, 1, 1];

println!("Peak: {:.1}", peak(&signal));         // 1.0
println!("RMS: {:.3}", rms(&signal));           // ~0.790
println!("Crest: {:.2}", crest_factor(&signal)); // ~1.27
println!("Clips: {}", clip_count(&signal));     // 4 samples at ±1
println!("ZCR: {}", zero_crossing_rate(&signal)); // transitions through 0
```

## The Deeper Truth

**Ternary VU meters have quantized dynamics.** In continuous audio, RMS varies smoothly. In ternary, RMS depends only on the ratio of {-1, 0, +1} values in the window. A signal that's ⅓ each value has RMS = √(2/3) ≈ 0.816. A signal that's all ±1 has RMS = 1.0. A signal that's all 0 has RMS = 0. There are only a few possible RMS values for any given window size, making the measurement deterministic and reproducible.

The crest factor is particularly informative for ternary signals: it distinguishes between dense signals (lots of ±1, low crest ≈ 1.0) and sparse signals (mostly 0 with occasional ±1, high crest > 1.0). This is the ternary equivalent of the transient/sustained distinction in audio.

**Use cases:**
- **Audio metering** — measure ternary signal levels
- **Signal health monitoring** — is the signal too quiet (all zeros) or clipping (all ±1)?
- **Dynamic range analysis** — crest factor as a proxy for dynamic range
- **Effect chain calibration** — meter between effects to prevent clipping
- **Data quality** — how much information (non-zero) is in the signal?

## See Also

- **ternary-gate** — noise gating (uses VU measurements for threshold decisions)
- **ternary-echo** — echo processing (VU meters before and after echo)
- **ternary-mixer** — mixing (need VU to prevent clipping when combining signals)
- **ternary-pan** — stereo panning (VU meters per channel)

## Install

```bash
cargo add ternary-vu
```

## License

MIT
