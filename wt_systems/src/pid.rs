use std::{fmt, ops};
use uom::{
    num_traits::{Num, Zero, clamp, zero}, 
    si::{
        quantities::*,
    }
};

pub struct PidConfiguration<In, Out, V = f64>
where
    V: uom::Conversion<V> + Num,
    Out: ops::Div<In> + ops::Div<<Time<V> as ops::Mul<In>>::Output>,
    Time<V>: ops::Mul<In> + ops::Div<In>,
{
    pub gain_proportion: <Out as ops::Div<In>>::Output,
    pub gain_integral: <Out as ops::Div<<Time<V> as ops::Mul<In>>::Output>>::Output,
    pub gain_derivative: <Time<V> as ops::Div<In>>::Output,
    pub output_range: (Out, Out),
    pub derivative_range: (Out, Out),
    pub tolerance: In,
}

impl<In, Out, V> Clone for PidConfiguration<In, Out, V>
where
    In: Clone,
    Out: Clone,
    V: uom::Conversion<V> + Num,
    Out: ops::Div<In> + ops::Div<<Time<V> as ops::Mul<In>>::Output> + Clone,
    Time<V>: ops::Mul<In> + ops::Div<In>,
    <Out as ops::Div<In>>::Output: Clone,
    <Out as ops::Div<<Time<V> as ops::Mul<In>>::Output>>::Output: Clone,
    <Time<V> as ops::Div<In>>::Output: Clone,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            gain_proportion: self.gain_proportion.clone(),
            gain_integral: self.gain_integral.clone(),
            gain_derivative: self.gain_derivative.clone(),
            output_range: self.output_range.clone(),
            derivative_range: self.derivative_range.clone(),
            tolerance: self.tolerance.clone(),
        }
    }
}

impl<In, Out, V> Copy for PidConfiguration<In, Out, V>
where
    In: Copy,
    V: uom::Conversion<V> + Num,
    Out: ops::Div<In> + ops::Div<<Time<V> as ops::Mul<In>>::Output> + Copy,
    Time<V>: ops::Mul<In> + ops::Div<In>,
    <Out as ops::Div<In>>::Output: Copy,
    <Out as ops::Div<<Time<V> as ops::Mul<In>>::Output>>::Output: Copy,
    <Time<V> as ops::Div<In>>::Output: Copy,
{}

impl<In, Out, V> fmt::Debug for PidConfiguration<In, Out, V>
where
    In: fmt::Debug,
    V: uom::Conversion<V> + Num + fmt::Debug,
    Out: ops::Div<In> + ops::Div<<Time<V> as ops::Mul<In>>::Output> + fmt::Debug,
    Time<V>: ops::Mul<In> + ops::Div<In>,
    <Out as ops::Div<In>>::Output: fmt::Debug,
    <Out as ops::Div<<Time<V> as ops::Mul<In>>::Output>>::Output: fmt::Debug,
    <Time<V> as ops::Div<In>>::Output: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("PidConfiguration")
            .field("gain_proportion", &self.gain_proportion)
            .field("gain_integral", &self.gain_integral)
            .field("gain_derivative", &self.gain_derivative)
            .field("output_range", &format_args!("[{:?}, {:?}]", &self.output_range.0, &self.output_range.1))
            .field("derivative_range", &format_args!("[{:?}, {:?}]", &self.derivative_range.0, &self.derivative_range.1))
            .field("tolerance", &self.tolerance)
            .finish()
    }
}

pub struct PidState<In, V = f64>
where
    V: uom::Conversion<V> + Num,
    Time<V>: ops::Mul<In>,
{
    prior_error: In,
    integral: <Time<V> as ops::Mul<In>>::Output,
}

impl<In, V> Clone for PidState<In, V>
where
    In: Clone,
    V: uom::Conversion<V> + Num,
    Time<V>: ops::Mul<In>,
    <Time<V> as ops::Mul<In>>::Output: Clone,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            prior_error: self.prior_error.clone(),
            integral: self.integral.clone(),
        }
    }
}

impl<In, V> Copy for PidState<In, V>
where
    In: Copy,
    V: uom::Conversion<V> + Num,
    Time<V>: ops::Mul<In>,
    <Time<V> as ops::Mul<In>>::Output: Copy,
{}

impl<In, V> fmt::Debug for PidState<In, V>
where
    In: fmt::Debug,
    V: uom::Conversion<V> + Num + fmt::Debug,
    Time<V>: ops::Mul<In>,
    <Time<V> as ops::Mul<In>>::Output: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("PidState")
            .field("prior_error", &self.prior_error)
            .field("integral", &self.integral)
            .finish()
    }
}

impl<In, V> Default for PidState<In, V>
where
    In: Zero,
    V: uom::Conversion<V> + Num,
    Time<V>: ops::Mul<In>,
    <Time<V> as ops::Mul<In>>::Output: Zero,
{
    #[inline]
    fn default() -> Self {
        Self {
            prior_error: zero(),
            integral: zero(),
        }
    }
}

impl<In, V> PidState<In, V>
where
    V: uom::Conversion<V> + Num,
    Time<V>: ops::Mul<In>,
{
    #[inline]
    pub fn with_initial(prior_error: In, integral: <Time<V> as ops::Mul<In>>::Output) -> Self {
        Self {
            prior_error,
            integral,
        }
    }

    #[inline]
    pub fn prior_error_ref(&self) -> &In {
        &self.prior_error
    }

    #[inline]
    pub fn integral_ref(&self) -> &<Time<V> as ops::Mul<In>>::Output {
        &self.integral
    }
}

impl<In, V> PidState<In, V>
where
    In: Clone,
    V: uom::Conversion<V> + Num,
    Time<V>: ops::Mul<In>,
{
    #[inline]
    pub fn prior_error(&self) -> In {
        self.prior_error.clone()
    }
}

impl<In, V> PidState<In, V>
where
    V: uom::Conversion<V> + Num,
    Time<V>: ops::Mul<In>,
    <Time<V> as ops::Mul<In>>::Output: Clone
{
    #[inline]
    pub fn integral(&self) -> <Time<V> as ops::Mul<In>>::Output {
        self.integral.clone()
    }
}

impl<In, V> PidState<In, V>
where
    V: uom::Conversion<V> + Num,
    Time<V>: ops::Mul<In>,
    Self: Default,
{
    #[inline]
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

impl<In, V> PidState<In, V>
where
    V: uom::Conversion<V> + Num,
    Time<V>: ops::Mul<In>,
{
    pub fn tick<Out>(&mut self, goal: In, config: &PidConfiguration<In, Out, V>, current: In, delta_t: Time<V>) -> Out
    where
        In: PartialOrd + Zero + ops::Sub<In, Output=In> + ops::Neg<Output=In> + ops::Mul<Time<V>> + ops::Div<Time<V>> + Copy,
        Out: Zero + PartialOrd + ops::Div<In> + ops::Div<<Time<V> as ops::Mul<In>>::Output> + Copy,
        <In as ops::Div<Time<V>>>::Output: ops::Mul<<Time<V> as ops::Div<In>>::Output, Output = Out> + Copy,
        <Out as ops::Div<In>>::Output: ops::Mul<In, Output = Out> + Copy,
        <Out as ops::Div<<Time<V> as ops::Mul<In>>::Output>>::Output: Copy,
        Time<V>: ops::Mul<In> + ops::Div<In> + Copy,
        <Time<V> as ops::Div<In>>::Output: Copy,
        <Time<V> as ops::Mul<In>>::Output: Zero +
            ops::Add<<Time<V> as ops::Mul<In>>::Output, Output=<Time<V> as ops::Mul<In>>::Output> + 
            ops::Div<f64, Output=<Time<V> as ops::Mul<In>>::Output> + 
            ops::Mul<<Out as ops::Div<<Time<V> as ops::Mul<In>>::Output>>::Output, Output=Out> +
            Copy,
    {
        let error = goal - current;

        // If the error is within tolerances, remove momentum and don't command a change
        if error < config.tolerance && -config.tolerance < error {
            self.integral = zero();
            return zero();
        }

        // Proportional
        let gained_error: Out = config.gain_proportion * error;

        // Integral
        // If the new error has changed signs, remove momentum
        let integral: <Time<V> as ops::Mul<In>>::Output = if (error > zero()) != (self.prior_error >= zero()) {
            zero()
        } else {
            self.integral + (delta_t * error) + (delta_t * (error - self.prior_error) / 2.)
        };
        let gained_integral = integral * config.gain_integral;

        // Derivative
        let error_over_time = (error - self.prior_error) / delta_t;
        let raw_gained_derivative: Out = error_over_time * config.gain_derivative;
        let gained_derivative = clamp(raw_gained_derivative, config.derivative_range.0, config.derivative_range.1);

        // Combination
        let raw_output: Out = gained_error + gained_integral + gained_derivative;
        let output = clamp(raw_output, config.output_range.0, config.output_range.1);

        // println!("Output: {} ({}): Derivative: {} ({}), Integral: {}, proportion: {}", output.into_format_args(ratio, uom::fmt::DisplayStyle::Abbreviation), raw_output.into_format_args(ratio, uom::fmt::DisplayStyle::Abbreviation), gained_derivative.into_format_args(ratio, uom::fmt::DisplayStyle::Abbreviation), raw_gained_derivative.into_format_args(ratio, uom::fmt::DisplayStyle::Abbreviation), gained_integral.into_format_args(ratio, uom::fmt::DisplayStyle::Abbreviation), gained_error.into_format_args(ratio, uom::fmt::DisplayStyle::Abbreviation));

        self.prior_error = error;
        self.integral = integral;
        output
    }
}

pub mod testing {
    #[macro_export]
    macro_rules! pid_tick_tests {
        {
            name: $name:ident,
            config: $config:expr,
            initial: $initial:expr,
            steps: [
                $({
                    inputs: ($current:expr, $delta_t:expr),
                    expect: ($expected_output:expr, $expected_integral:expr)$(,)?
                }),*$(,)?
            ],
            tolerances: {
                output: $output_tolerance:expr,
                integral: $integral_tolerance:expr$(,)?
            }$(,)?
        } => {
            #[test]
            fn $name() {
                let config = $config;
                let mut state = $initial;
                println!("Initial:    {:?}", state);

                let mut step = 0;
                let mut failed = false;
                $(
                    #[allow(unused_assignments)]
                    {
                        step += 1;
                        let goal = ::uom::num_traits::zero();
                        let current = $current;
                        let delta_t = $delta_t;

                        println!("Step {:>2} Goal: {:?}; Current: {:?}; Step: {:?}", step, goal, current, delta_t);

                        let actual = state.tick(goal, &config, -current, $delta_t);
                        let expected = (
                            $crate::PidState::with_initial(goal - current, $expected_integral),
                            $expected_output
                        );

                        let difference = (
                            $crate::PidState::with_initial(
                                expected.0.prior_error() - state.prior_error(),
                                expected.0.integral() - state.integral(),
                            ),
                            expected.1 - actual
                        );

                        println!("    Expected:   {:?}", expected);
                        println!("    Actual:     {:?}", (state, actual));
                        println!("    Difference: {:?}", difference);

                        #[allow(clippy::float_cmp)]
                        if state.prior_error() != expected.0.prior_error() && ::uom::num_traits::Zero::is_zero(&state.prior_error()) {
                            eprintln!(" !!! error mismatch !!!");
                            failed = true
                        }

                        if difference.0.integral() > $integral_tolerance || difference.0.integral() < -$integral_tolerance  {
                            eprintln!(" !!! integral mismatch !!!");
                            failed = true;
                        }

                        if difference.1 > $output_tolerance || difference.1 < -$output_tolerance {
                            eprintln!(" !!! output mismatch !!!");
                            failed = true;
                        }
                    }
                )*

                if failed {
                    panic!("One of the test steps had a result outside of tolerances");
                }
            }
        };
    }
}