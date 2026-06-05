#![forbid(unsafe_code)]

#[derive(Debug, Clone, Default)]
pub struct VuMeter {
    pub peak: f64,
    pub rms: f64,
    pub count: usize,
}

/// Compute peak absolute value of signal.
pub fn peak(signal: &[i8]) -> f64 {
    signal
        .iter()
        .map(|&s| (s as i16).unsigned_abs() as f64)
        .fold(0.0_f64, f64::max)
}

/// Compute RMS of signal.
pub fn rms(signal: &[i8]) -> f64 {
    if signal.is_empty() {
        return 0.0;
    }
    let sum: f64 = signal.iter().map(|&s| (s as f64) * (s as f64)).sum();
    (sum / signal.len() as f64).sqrt()
}

/// Crest factor: peak/rms ratio. Returns 0.0 if rms is 0.
pub fn crest_factor(signal: &[i8]) -> f64 {
    let r = rms(signal);
    if r == 0.0 {
        return 0.0;
    }
    peak(signal) / r
}

/// Zero-crossing rate (fraction of samples).
pub fn zero_crossing_rate(signal: &[i8]) -> f64 {
    if signal.len() < 2 {
        return 0.0;
    }
    let crossings = signal
        .windows(2)
        .filter(|w| (w[0] >= 0) != (w[1] >= 0))
        .count();
    crossings as f64 / (signal.len() - 1) as f64
}

/// DC offset (mean of signal).
pub fn dc_offset(signal: &[i8]) -> f64 {
    if signal.is_empty() {
        return 0.0;
    }
    signal.iter().map(|&s| s as f64).sum::<f64>() / signal.len() as f64
}

/// L-R balance: -1.0 = full left, 0.0 = center, 1.0 = full right.
pub fn balance(left: &[i8], right: &[i8]) -> f64 {
    let n = left.len().min(right.len());
    if n == 0 {
        return 0.0;
    }
    let l_energy: f64 = left[..n].iter().map(|&s| (s as f64).powi(2)).sum::<f64>() / n as f64;
    let r_energy: f64 = right[..n].iter().map(|&s| (s as f64).powi(2)).sum::<f64>() / n as f64;
    let total = l_energy + r_energy;
    if total == 0.0 {
        return 0.0;
    }
    (r_energy - l_energy) / total
}

/// Stereo correlation: -1.0 (inverse) to 1.0 (identical).
pub fn correlation(left: &[i8], right: &[i8]) -> f64 {
    let n = left.len().min(right.len());
    if n == 0 {
        return 0.0;
    }
    let l_mean = left[..n].iter().map(|&s| s as f64).sum::<f64>() / n as f64;
    let r_mean = right[..n].iter().map(|&s| s as f64).sum::<f64>() / n as f64;
    let mut num = 0.0;
    let mut den_l = 0.0;
    let mut den_r = 0.0;
    for i in 0..n {
        let dl = left[i] as f64 - l_mean;
        let dr = right[i] as f64 - r_mean;
        num += dl * dr;
        den_l += dl * dl;
        den_r += dr * dr;
    }
    let den = den_l * den_r;
    if den == 0.0 {
        return 0.0;
    }
    num / den.sqrt()
}

/// Spectral centroid (brightness measure) using zero-crossing rate as proxy.
pub fn spectrum_centroid(signal: &[i8]) -> f64 {
    // Use weighted position of energy: higher-frequency content has more zero crossings
    if signal.len() < 2 {
        return 0.0;
    }
    let zcr = zero_crossing_rate(signal);
    let pk = peak(signal);
    zcr * pk
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peak_all_zeros() {
        assert_eq!(peak(&[0, 0, 0]), 0.0);
    }

    #[test]
    fn peak_mixed() {
        assert_eq!(peak(&[1, -1, 0]), 1.0);
    }

    #[test]
    fn peak_empty() {
        assert_eq!(peak(&[]), 0.0);
    }

    #[test]
    fn rms_zeros() {
        assert_eq!(rms(&[0, 0, 0]), 0.0);
    }

    #[test]
    fn rms_values() {
        let r = rms(&[1, -1]);
        assert!((r - 1.0).abs() < 1e-9);
    }

    #[test]
    fn rms_empty() {
        assert_eq!(rms(&[]), 0.0);
    }

    #[test]
    fn crest_factor_flat() {
        // All same value: peak == rms, crest == 1.0
        let cf = crest_factor(&[1, 1, 1, 1]);
        assert!((cf - 1.0).abs() < 1e-9);
    }

    #[test]
    fn crest_factor_silent() {
        assert_eq!(crest_factor(&[0, 0, 0]), 0.0);
    }

    #[test]
    fn zero_crossing_rate_alternating() {
        let zcr = zero_crossing_rate(&[1, -1, 1, -1]);
        assert!((zcr - 1.0).abs() < 1e-9);
    }

    #[test]
    fn zero_crossing_rate_flat() {
        let zcr = zero_crossing_rate(&[1, 1, 1, 1]);
        assert_eq!(zcr, 0.0);
    }

    #[test]
    fn zero_crossing_rate_short() {
        assert_eq!(zero_crossing_rate(&[1]), 0.0);
    }

    #[test]
    fn dc_offset_positive() {
        let dc = dc_offset(&[1, 1, 1]);
        assert!((dc - 1.0).abs() < 1e-9);
    }

    #[test]
    fn dc_offset_balanced() {
        let dc = dc_offset(&[1, -1, 1, -1]);
        assert!(dc.abs() < 1e-9);
    }

    #[test]
    fn dc_offset_empty() {
        assert_eq!(dc_offset(&[]), 0.0);
    }

    #[test]
    fn balance_centered() {
        let b = balance(&[1, -1], &[1, -1]);
        assert!(b.abs() < 1e-9);
    }

    #[test]
    fn balance_left_only() {
        let b = balance(&[1, 1], &[0, 0]);
        assert!((b - (-1.0)).abs() < 1e-9);
    }

    #[test]
    fn balance_right_only() {
        let b = balance(&[0, 0], &[1, 1]);
        assert!((b - 1.0).abs() < 1e-9);
    }

    #[test]
    fn correlation_identical() {
        let c = correlation(&[1, -1, 1], &[1, -1, 1]);
        assert!((c - 1.0).abs() < 1e-9);
    }

    #[test]
    fn correlation_inverse() {
        let c = correlation(&[1, -1, 1], &[-1, 1, -1]);
        assert!((c - (-1.0)).abs() < 1e-9);
    }

    #[test]
    fn correlation_empty() {
        assert_eq!(correlation(&[], &[]), 0.0);
    }

    #[test]
    fn spectrum_centroid_basic() {
        let sc = spectrum_centroid(&[1, -1, 1, -1]);
        assert!(sc > 0.0);
    }

    #[test]
    fn vu_meter_default() {
        let m = VuMeter::default();
        assert_eq!(m.peak, 0.0);
        assert_eq!(m.rms, 0.0);
        assert_eq!(m.count, 0);
    }
}
