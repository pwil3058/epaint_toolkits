// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

// To tide us over until std library gets its act together

use num_traits::Float;

pub trait UsefulFloatConstants: Float {
    /// Mathematical constants
    ///
    /// Euler's number (e)
    const E: Self;
    /// 1/π
    const FRAC_1_PI: Self;
    /// 2/π
    const FRAC_2_PI: Self;
    /// 2/sqrt(π)
    const FRAC_2_SQRT_PI: Self;
    /// 1/sqrt(2)
    const FRAC_1_SQRT_2: Self;
    /// π/2
    const FRAC_PI_2: Self;
    /// π/3
    const FRAC_PI_3: Self;
    /// π/4
    const FRAC_PI_4: Self;
    /// π/6
    const FRAC_PI_6: Self;
    /// π/8
    const FRAC_PI_8: Self;
    /// ln(2)
    const LN_2: Self;
    /// ln(10)
    const LN_10: Self;
    /// log2(e)
    const LOG2_E: Self;
    /// log10(e)
    const LOG10_E: Self;
    /// Archimedes' constant (π)
    const PI: Self;
    /// sqrt(2)
    const SQRT_2: Self;

    /// Other values useful in generic float code
    const SQRT_3: Self;
    const TWO_PI: Self;
}

impl UsefulFloatConstants for f32 {
    /// Euler's number (e)
    const E: Self = std::f32::consts::E;
    /// 1/π
    const FRAC_1_PI: Self = std::f32::consts::FRAC_1_PI;
    /// 2/π
    const FRAC_2_PI: Self = std::f32::consts::FRAC_2_PI;
    /// 2/sqrt(π)
    const FRAC_2_SQRT_PI: Self = std::f32::consts::FRAC_2_SQRT_PI;
    /// 1/sqrt(2)
    const FRAC_1_SQRT_2: Self = std::f32::consts::FRAC_1_SQRT_2;
    /// π/2
    const FRAC_PI_2: Self = std::f32::consts::FRAC_PI_2;
    /// π/3
    const FRAC_PI_3: Self = std::f32::consts::FRAC_PI_3;
    /// π/4
    const FRAC_PI_4: Self = std::f32::consts::FRAC_PI_4;
    /// π/6
    const FRAC_PI_6: Self = std::f32::consts::FRAC_PI_6;
    /// π/8
    const FRAC_PI_8: Self = std::f32::consts::FRAC_PI_8;
    /// ln(2)
    const LN_2: Self = std::f32::consts::LN_2;
    /// ln(10)
    const LN_10: Self = std::f32::consts::LN_10;
    /// log2(e)
    const LOG2_E: Self = std::f32::consts::LOG2_E;
    /// log10(e)
    const LOG10_E: Self = std::f32::consts::LOG10_E;
    /// Archimedes' constant (π)
    const PI: Self = std::f32::consts::PI;
    /// sqrt(2)
    const SQRT_2: Self = std::f32::consts::SQRT_2;

    /// Other values useful in generic float code
    const SQRT_3: Self = 1.73205_08;
    const TWO_PI: Self = std::f32::consts::PI * 2.0;
}

impl UsefulFloatConstants for f64 {
    /// Euler's number (e)
    const E: Self = std::f64::consts::E;
    /// 1/π
    const FRAC_1_PI: Self = std::f64::consts::FRAC_1_PI;
    /// 2/π
    const FRAC_2_PI: Self = std::f64::consts::FRAC_2_PI;
    /// 2/sqrt(π)
    const FRAC_2_SQRT_PI: Self = std::f64::consts::FRAC_2_SQRT_PI;
    /// 1/sqrt(2)
    const FRAC_1_SQRT_2: Self = std::f64::consts::FRAC_1_SQRT_2;
    /// π/2
    const FRAC_PI_2: Self = std::f64::consts::FRAC_PI_2;
    /// π/3
    const FRAC_PI_3: Self = std::f64::consts::FRAC_PI_3;
    /// π/4
    const FRAC_PI_4: Self = std::f64::consts::FRAC_PI_4;
    /// π/6
    const FRAC_PI_6: Self = std::f64::consts::FRAC_PI_6;
    /// π/8
    const FRAC_PI_8: Self = std::f64::consts::FRAC_PI_8;
    /// ln(2)
    const LN_2: Self = std::f64::consts::LN_2;
    /// ln(10)
    const LN_10: Self = std::f64::consts::LN_10;
    /// log2(e)
    const LOG2_E: Self = std::f64::consts::LOG2_E;
    /// log10(e)
    const LOG10_E: Self = std::f64::consts::LOG10_E;
    /// Archimedes' constant (π)
    const PI: Self = std::f64::consts::PI;
    /// sqrt(2)
    const SQRT_2: Self = std::f64::consts::SQRT_2;

    /// Other values useful in generic float code
    const SQRT_3: Self = 1.73205_08075_68878;
    const TWO_PI: Self = std::f64::consts::PI * 2.0;
}
