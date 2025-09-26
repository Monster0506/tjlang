//! Tests for MATH module

#[cfg(test)]
mod tests {
    use crate::stdlib::math::*;

    #[test]
    fn test_add() {
        let result = MATH::add(5.0, 3.0);
        assert_eq!(result, 8.0);
    }

    #[test]
    fn test_subtract() {
        let result = MATH::subtract(5.0, 3.0);
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_multiply() {
        let result = MATH::multiply(5.0, 3.0);
        assert_eq!(result, 15.0);
    }

    #[test]
    fn test_divide() {
        let result = MATH::divide(15.0, 3.0);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_modulo() {
        let result = MATH::modulo(10.0, 3.0);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_power() {
        let result = MATH::power(2.0, 3.0);
        assert_eq!(result, 8.0);
    }

    #[test]
    fn test_sqrt() {
        let result = MATH::sqrt(16.0);
        assert_eq!(result, 4.0);
    }

    #[test]
    fn test_cbrt() {
        let result = MATH::cbrt(27.0);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_abs() {
        let result = MATH::abs(-5.0);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_sign() {
        let result = MATH::sign(-5.0);
        assert_eq!(result, -1.0);
    }

    #[test]
    fn test_floor() {
        let result = MATH::floor(3.7);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_ceil() {
        let result = MATH::ceil(3.2);
        assert_eq!(result, 4.0);
    }

    #[test]
    fn test_round() {
        let result = MATH::round(3.5);
        assert_eq!(result, 4.0);
    }

    #[test]
    fn test_trunc() {
        let result = MATH::trunc(3.7);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_fract() {
        let result = MATH::fract(3.7);
        assert!((result - 0.7).abs() < 1e-10);
    }

    #[test]
    fn test_sin() {
        let result = MATH::sin(0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_cos() {
        let result = MATH::cos(0.0);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_tan() {
        let result = MATH::tan(0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_asin() {
        let result = MATH::asin(0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_acos() {
        let result = MATH::acos(1.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_atan() {
        let result = MATH::atan(0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_atan2() {
        let result = MATH::atan2(0.0, 1.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_sinh() {
        let result = MATH::sinh(0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_cosh() {
        let result = MATH::cosh(0.0);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_tanh() {
        let result = MATH::tanh(0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_asinh() {
        let result = MATH::asinh(0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_acosh() {
        let result = MATH::acosh(1.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_atanh() {
        let result = MATH::atanh(0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_ln() {
        let result = MATH::ln(1.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_log10() {
        let result = MATH::log10(1.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_log2() {
        let result = MATH::log2(1.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_log() {
        let result = MATH::log(8.0, 2.0);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_exp() {
        let result = MATH::exp(0.0);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_exp2() {
        let result = MATH::exp2(3.0);
        assert_eq!(result, 8.0);
    }

    #[test]
    fn test_exp_m1() {
        let result = MATH::exp_m1(0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_ln_1p() {
        let result = MATH::ln_1p(0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_mean() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = MATH::mean(&values);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_median() {
        let mut values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = MATH::median(&mut values);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_mode() {
        let values = vec![1.0, 2.0, 2.0, 3.0, 3.0, 3.0];
        let result = MATH::mode(&values);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_variance() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = MATH::variance(&values);
        assert!(result > 0.0);
    }

    #[test]
    fn test_std_dev() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = MATH::std_dev(&values);
        assert!(result > 0.0);
    }

    #[test]
    fn test_min() {
        let values = vec![5.0, 2.0, 8.0, 1.0, 9.0];
        let result = MATH::min(&values);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_max() {
        let values = vec![5.0, 2.0, 8.0, 1.0, 9.0];
        let result = MATH::max(&values);
        assert_eq!(result, 9.0);
    }

    #[test]
    fn test_sum() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = MATH::sum(&values);
        assert_eq!(result, 15.0);
    }

    #[test]
    fn test_product() {
        let values = vec![1.0, 2.0, 3.0, 4.0];
        let result = MATH::product(&values);
        assert_eq!(result, 24.0);
    }

    #[test]
    fn test_dot_product() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let result = MATH::dot_product(&a, &b);
        assert_eq!(result, 32.0);
    }

    #[test]
    fn test_cross_product() {
        let a = [1.0, 0.0, 0.0];
        let b = [0.0, 1.0, 0.0];
        let result = MATH::cross_product(&a, &b);
        assert_eq!(result, [0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_magnitude() {
        let vector = vec![3.0, 4.0];
        let result = MATH::magnitude(&vector);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_normalize() {
        let vector = vec![3.0, 4.0];
        let result = MATH::normalize(&vector);
        let expected = vec![0.6, 0.8];
        assert!((result[0] - expected[0]).abs() < 1e-10);
        assert!((result[1] - expected[1]).abs() < 1e-10);
    }

    #[test]
    fn test_matrix_multiply() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
        let result = MATH::matrix_multiply(&a, &b);
        let expected = vec![vec![19.0, 22.0], vec![43.0, 50.0]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_transpose() {
        let matrix = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let result = MATH::matrix_transpose(&matrix);
        let expected = vec![vec![1.0, 3.0], vec![2.0, 4.0]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_determinant() {
        let matrix = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let result = MATH::matrix_determinant(&matrix);
        assert_eq!(result, -2.0);
    }

    #[test]
    fn test_gcd() {
        let result = MATH::gcd(12, 8);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_lcm() {
        let result = MATH::lcm(12, 8);
        assert_eq!(result, 24);
    }

    #[test]
    fn test_is_prime() {
        let result = MATH::is_prime(17);
        assert!(result);
    }

    #[test]
    fn test_factorial() {
        let result = MATH::factorial(5);
        assert_eq!(result, 120);
    }

    #[test]
    fn test_fibonacci() {
        let result = MATH::fibonacci(10);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_derivative() {
        // Test derivative of x^2 at x=2
        let result = MATH::derivative(|x| x * x, 2.0, 0.001);
        assert!((result - 4.0).abs() < 0.01);
    }

    #[test]
    fn test_integral() {
        // Test integral of x from 0 to 2
        let result = MATH::integral(|x| x, 0.0, 2.0, 1000);
        assert!((result - 2.0).abs() < 0.01);
    }

    #[test]
    #[ignore]
    fn test_golden_section_search() {
        // Test finding minimum of x^2
        let result = MATH::golden_section_search(|x| x * x, -10.0, 10.0, 0.001);
        assert!(result.abs() < 0.001);
    }

    #[test]
    fn test_newton_raphson() {
        // Test finding root of x^2 - 4 = 0
        let result = MATH::newton_raphson(|x| x * x - 4.0, |x| 2.0 * x, 3.0, 0.001, 100);
        assert!((result - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_bisection() {
        // Test finding root of x^2 - 4 = 0
        let result = MATH::bisection(|x| x * x - 4.0, 0.0, 5.0, 0.001);
        assert!((result - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_constants() {
        assert_eq!(MATH::PI, std::f64::consts::PI);
        assert_eq!(MATH::E, std::f64::consts::E);
        assert_eq!(MATH::TAU, std::f64::consts::TAU);
        assert_eq!(MATH::SQRT_2, std::f64::consts::SQRT_2);
        assert_eq!(MATH::LN_2, std::f64::consts::LN_2);
        assert_eq!(MATH::LN_10, std::f64::consts::LN_10);
        assert_eq!(MATH::LOG2_E, std::f64::consts::LOG2_E);
        assert_eq!(MATH::LOG10_E, std::f64::consts::LOG10_E);
        assert_eq!(MATH::INFINITY, f64::INFINITY);
        assert_eq!(MATH::NEG_INFINITY, f64::NEG_INFINITY);
        assert!(MATH::NAN.is_nan());
    }
}
