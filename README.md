# ternary-vu

Signal analysis and VU (Volume Unit) metering for **ternary signals** — discrete-valued audio-like signals over the alphabet {-1, 0, +1}. Provides peak, RMS, crest factor, zero-crossing rate, DC offset, stereo balance, and correlation metrics. `#![forbid(unsafe_code)]`.

## Why It Matters

Ternary signals {-1, 0, +1} arise naturally in delta-sigma modulation, ternary logic circuits, and the ternary agent ecosystem. Analyzing their statistical properties — energy distribution, spectral content, stereo coherence — is essential for evaluating agent communication channels, detecting signal degradation, and measuring the "health" of ternary data streams.

These metrics are the audio-engineering analogues of the population statistics used in ternary agent systems: RMS measures average energy, crest factor measures dynamic range, and zero-crossing rate approximates spectral centroid.

## How It Works

### Peak Amplitude

```
peak(x) = max(|x_i|),  x_i ∈ {-1, 0, +1}
```

Since |x_i| ≤ 1, peak ∈ {0, 1}. Peak = 0 indicates silence.

- **Complexity:** O(n)

### RMS (Root Mean Square)

```
RMS(x) = √( (1/n) Σᵢ x_i² )
```

For ternary signals, x_i² ∈ {0, 1}, so RMS = √(fraction of non-zero samples).

- **Complexity:** O(n)
- **Range:** [0, 1]

### Crest Factor

```
CF(x) = peak(x) / RMS(x)
```

- **Range:** [1, ∞) for non-silent signals
- CF = 1: constant-amplitude signal (all ±1)
- High CF: sparse impulses amid silence
- Returns 0.0 for silent signals (RMS = 0)

### Zero-Crossing Rate

```
ZCR(x) = (1/(n−1)) · |{ i : sign(x_i) ≠ sign(x_{i+1}) }|
```

Where sign is defined on the ternary alphabet: negative (−1) vs. non-negative (0, +1).

- **Complexity:** O(n)
- **Range:** [0, 1]
- High ZCR → high-frequency content; ZCR ≈ 0 → DC or low-frequency

### DC Offset

```
DC(x) = (1/n) Σᵢ x_i
```

- **Range:** [−1, +1]
- DC = 0: balanced signal (equal +1 and −1)
- DC > 0: positive bias; DC < 0: negative bias

### Stereo Balance

```
balance(L, R) = (E_R − E_L) / (E_L + E_R)
```

Where E = mean squared energy. This is the normalized energy difference:

- **Range:** [−1, +1]
- −1: full left, 0: centered, +1: full right

### Stereo Correlation

Pearson correlation between left and right channels:

```
ρ(L, R) = Σᵢ(L_i − L̄)(R_i − R̄) / √(Σᵢ(L_i − L̄)² · Σᵢ(R_i − R̄)²)
```

- **Range:** [−1, +1]
- +1: identical channels (mono)
- 0: uncorrelated
- −1: phase-inverted

### Spectral Centroid (Proxy)

```
SC(x) = ZCR(x) × peak(x)
```

A rough brightness proxy: higher ZCR × peak = more high-frequency energy at full amplitude.

## Quick Start

```rust
use ternary_vu::*;

let signal = vec![1, -1, 0, 1, -1, 1, 0, -1];

println!("Peak:   {:.3}", peak(&signal));      // 1.0
println!("RMS:    {:.3}", rms(&signal));        // ~0.707
println!("Crest:  {:.3}", crest_factor(&signal));
println!("ZCR:    {:.3}", zero_crossing_rate(&signal));
println!("DC:     {:.3}", dc_offset(&signal));

// Stereo analysis
let left  = vec![1, -1, 0, 1];
let right = vec![1, -1, 0, 1];
println!("Balance:     {:.3}", balance(&left, &right));  // 0.0 (centered)
println!("Correlation: {:.3}", correlation(&left, &right)); // 1.0 (identical)
```

## API

| Function | Input | Returns |
|---|---|---|
| `peak` | `&[i8]` | `f64` ∈ {0, 1} |
| `rms` | `&[i8]` | `f64` ∈ [0, 1] |
| `crest_factor` | `&[i8]` | `f64` ∈ [1, ∞) or 0 |
| `zero_crossing_rate` | `&[i8]` | `f64` ∈ [0, 1] |
| `dc_offset` | `&[i8]` | `f64` ∈ [−1, +1] |
| `balance` | `&[i8], &[i8]` | `f64` ∈ [−1, +1] |
| `correlation` | `&[i8], &[i8]` | `f64` ∈ [−1, +1] |
| `spectrum_centroid` | `&[i8]` | `f64` (brightness proxy) |

## Architecture Notes

The DC offset metric is directly related to the **γ + η = C** conservation law. For a ternary signal, the DC offset equals (count of +1 − count of −1) / n, which is the net ternary bias. When the signal represents an agent population — γ fraction at +1, η fraction at −1 — the DC offset is (γ − η) / (γ + η + neutrals). A DC offset of 0 means γ = η, satisfying the symmetric conservation point where the population is evenly split between choose and avoid.

The RMS metric provides the "active fraction" √((γ + η) / total), quantifying how much of the population is non-neutral.

## References

- Smith, J. O. (2011). *Spectral Audio Signal Processing.* W3K Publishing. — RMS, crest factor, spectral centroid.
- ITU-R BS.468-4 (1970). *"Measurement of Audio-Frequency Noise Voltage Level in Sound Broadcasting."*
- Brandenburg, K. & Stoll, G. (1994). *"The ISO/MPEG-Audio Layer-3 Codec."* — ZCR as spectral proxy.

## License

MIT
