//! MATH Module - Mathematical operations
//!
//! Provides comprehensive mathematical functionality including:
//! - Basic arithmetic operations
//! - Trigonometric functions
//! - Logarithmic functions
//! - Statistical functions
//! - Linear algebra operations
//! - Number theory functions
//! - Calculus operations
//! - Probability functions
//! - Optimization algorithms
//! - Numerical analysis

use crate::values::Value;
use std::f64::consts::*;

/// MATH module for mathematical operations
pub struct MATH;

impl MATH {
    // Basic arithmetic
    pub fn add(a: f64, b: f64) -> f64 { a + b }
    pub fn subtract(a: f64, b: f64) -> f64 { a - b }
    pub fn multiply(a: f64, b: f64) -> f64 { a * b }
    pub fn divide(a: f64, b: f64) -> f64 { a / b }
    pub fn modulo(a: f64, b: f64) -> f64 { a % b }
    pub fn power(a: f64, b: f64) -> f64 { a.powf(b) }
    pub fn sqrt(a: f64) -> f64 { a.sqrt() }
    pub fn cbrt(a: f64) -> f64 { a.cbrt() }
    pub fn abs(a: f64) -> f64 { a.abs() }
    pub fn sign(a: f64) -> f64 { a.signum() }
    pub fn floor(a: f64) -> f64 { a.floor() }
    pub fn ceil(a: f64) -> f64 { a.ceil() }
    pub fn round(a: f64) -> f64 { a.round() }
    pub fn trunc(a: f64) -> f64 { a.trunc() }
    pub fn fract(a: f64) -> f64 { a.fract() }
    
    // Trigonometric functions
    pub fn sin(a: f64) -> f64 { a.sin() }
    pub fn cos(a: f64) -> f64 { a.cos() }
    pub fn tan(a: f64) -> f64 { a.tan() }
    pub fn asin(a: f64) -> f64 { a.asin() }
    pub fn acos(a: f64) -> f64 { a.acos() }
    pub fn atan(a: f64) -> f64 { a.atan() }
    pub fn atan2(y: f64, x: f64) -> f64 { y.atan2(x) }
    pub fn sinh(a: f64) -> f64 { a.sinh() }
    pub fn cosh(a: f64) -> f64 { a.cosh() }
    pub fn tanh(a: f64) -> f64 { a.tanh() }
    pub fn asinh(a: f64) -> f64 { a.asinh() }
    pub fn acosh(a: f64) -> f64 { a.acosh() }
    pub fn atanh(a: f64) -> f64 { a.atanh() }
    
    // Logarithmic functions
    pub fn ln(a: f64) -> f64 { a.ln() }
    pub fn log10(a: f64) -> f64 { a.log10() }
    pub fn log2(a: f64) -> f64 { a.log2() }
    pub fn log(a: f64, base: f64) -> f64 { a.log(base) }
    pub fn exp(a: f64) -> f64 { a.exp() }
    pub fn exp2(a: f64) -> f64 { a.exp2() }
    pub fn exp_m1(a: f64) -> f64 { a.exp_m1() }
    pub fn ln_1p(a: f64) -> f64 { a.ln_1p() }
    
    // Statistical functions
    pub fn mean(values: &[f64]) -> f64 {
        if values.is_empty() { 0.0 } else { values.iter().sum::<f64>() / values.len() as f64 }
    }
    
    pub fn median(values: &mut [f64]) -> f64 {
        if values.is_empty() { 0.0 } else {
            values.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let mid = values.len() / 2;
            if values.len() % 2 == 0 {
                (values[mid - 1] + values[mid]) / 2.0
            } else {
                values[mid]
            }
        }
    }
    
    pub fn mode(values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        
        let mut frequency_map = std::collections::HashMap::new();
        for &value in values {
            *frequency_map.entry(value.to_bits()).or_insert(0) += 1;
        }
        
        let mut max_freq = 0;
        let mut mode_value = values[0];
        
        for (&value_bits, &freq) in &frequency_map {
            if freq > max_freq {
                max_freq = freq;
                mode_value = f64::from_bits(value_bits);
            }
        }
        
        mode_value
    }
    
    pub fn variance(values: &[f64]) -> f64 {
        if values.is_empty() { 0.0 } else {
            let mean = Self::mean(values);
            let sum_squared_diff = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>();
            sum_squared_diff / values.len() as f64
        }
    }
    
    pub fn std_dev(values: &[f64]) -> f64 {
        Self::variance(values).sqrt()
    }
    
    pub fn min(values: &[f64]) -> f64 {
        values.iter().fold(f64::INFINITY, |a, &b| a.min(b))
    }
    
    pub fn max(values: &[f64]) -> f64 {
        values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    }
    
    pub fn sum(values: &[f64]) -> f64 {
        values.iter().sum()
    }
    
    pub fn product(values: &[f64]) -> f64 {
        values.iter().product()
    }
    
    // Linear algebra
    pub fn dot_product(a: &[f64], b: &[f64]) -> f64 {
        a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
    }
    
    pub fn cross_product(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    }
    
    pub fn magnitude(vector: &[f64]) -> f64 {
        vector.iter().map(|x| x * x).sum::<f64>().sqrt()
    }
    
    pub fn normalize(vector: &[f64]) -> Vec<f64> {
        let mag = Self::magnitude(vector);
        if mag == 0.0 { vec![0.0; vector.len()] } else {
            vector.iter().map(|x| x / mag).collect()
        }
    }
    
    // Number theory
    pub fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 { a } else { Self::gcd(b, a % b) }
    }
    
    pub fn lcm(a: i64, b: i64) -> i64 {
        (a * b).abs() / Self::gcd(a, b)
    }
    
    pub fn is_prime(n: u64) -> bool {
        if n < 2 { return false; }
        if n == 2 { return true; }
        if n % 2 == 0 { return false; }
        for i in (3..=(n as f64).sqrt() as u64).step_by(2) {
            if n % i == 0 { return false; }
        }
        true
    }
    
    pub fn factorial(n: u64) -> u64 {
        if n <= 1 { 1 } else { n * Self::factorial(n - 1) }
    }
    
    pub fn fibonacci(n: u64) -> u64 {
        if n <= 1 { n } else { Self::fibonacci(n - 1) + Self::fibonacci(n - 2) }
    }
    
    // Calculus
    pub fn derivative<F>(f: F, x: f64, h: f64) -> f64 
    where F: Fn(f64) -> f64 {
        (f(x + h) - f(x - h)) / (2.0 * h)
    }
    
    pub fn integral<F>(f: F, a: f64, b: f64, n: u32) -> f64 
    where F: Fn(f64) -> f64 {
        let h = (b - a) / n as f64;
        let mut sum = 0.0;
        for i in 0..n {
            let x = a + i as f64 * h;
            sum += f(x);
        }
        sum * h
    }
    
    // Probability
    pub fn normal_pdf(x: f64, mean: f64, std_dev: f64) -> f64 {
        let variance = std_dev * std_dev;
        (1.0 / (2.0 * PI * variance).sqrt()) * (-((x - mean).powi(2)) / (2.0 * variance)).exp()
    }
    
    pub fn normal_cdf(x: f64, mean: f64, std_dev: f64) -> f64 {
        // TODO: Implement normal CDF using approximation
        0.5 * (1.0 + Self::erf((x - mean) / (std_dev * 2.0_f64.sqrt())))
    }
    
    pub fn erf(x: f64) -> f64 {
        // Approximation of error function
        let a1 = 0.254829592;
        let a2 = -0.284496736;
        let a3 = 1.421413741;
        let a4 = -1.453152027;
        let a5 = 1.061405429;
        let p = 0.3275911;
        
        let sign = if x < 0.0 { -1.0 } else { 1.0 };
        let x = x.abs();
        
        let t = 1.0 / (1.0 + p * x);
        let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();
        
        sign * y
    }
    
    // Optimization
    pub fn golden_section_search<F>(f: F, a: f64, b: f64, tol: f64) -> f64 
    where F: Fn(f64) -> f64 {
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
        let resphi = 2.0 - phi;
        
        let mut x1 = a + resphi * (b - a);
        let mut x2 = a + resphi * (b - a);
        let mut f1 = f(x1);
        let mut f2 = f(x2);
        
        while (b - a).abs() > tol {
            if f1 < f2 {
                let b = x2;
                x2 = x1;
                x2 = a + resphi * (b - a);
                f2 = f(x2);
            } else {
                let a = x1;
                x1 = x2;
                x1 = a + resphi * (b - a);
                f1 = f(x1);
            }
        }
        
        if f1 < f2 { x1 } else { x2 }
    }
    
    // Numerical analysis
    pub fn newton_raphson<F, G>(f: F, f_prime: G, x0: f64, tol: f64, max_iter: u32) -> f64 
    where F: Fn(f64) -> f64, G: Fn(f64) -> f64 {
        let mut x = x0;
        for _ in 0..max_iter {
            let fx = f(x);
            if fx.abs() < tol { break; }
            let fpx = f_prime(x);
            if fpx == 0.0 { break; }
            x = x - fx / fpx;
        }
        x
    }
    
    pub fn bisection<F>(f: F, a: f64, b: f64, tol: f64) -> f64 
    where F: Fn(f64) -> f64 {
        let mut a = a;
        let mut b = b;
        while (b - a).abs() > tol {
            let c = (a + b) / 2.0;
            if f(a) * f(c) < 0.0 {
                b = c;
            } else {
                a = c;
            }
        }
        (a + b) / 2.0
    }
    
    // Matrix operations
    pub fn matrix_multiply(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let rows = a.len();
        let cols = b[0].len();
        let inner = b.len();
        
        let mut result = vec![vec![0.0; cols]; rows];
        for i in 0..rows {
            for j in 0..cols {
                for k in 0..inner {
                    result[i][j] += a[i][k] * b[k][j];
                }
            }
        }
        result
    }
    
    pub fn matrix_transpose(matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
        if matrix.is_empty() { return vec![]; }
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut result = vec![vec![0.0; rows]; cols];
        for i in 0..rows {
            for j in 0..cols {
                result[j][i] = matrix[i][j];
            }
        }
        result
    }
    
    pub fn matrix_determinant(matrix: &[Vec<f64>]) -> f64 {
        if matrix.len() == 1 { return matrix[0][0]; }
        if matrix.len() == 2 {
            return matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
        }
        
        let mut det = 0.0;
        for j in 0..matrix.len() {
            let minor = Self::matrix_minor(matrix, 0, j);
            det += matrix[0][j] * Self::matrix_determinant(&minor) * if j % 2 == 0 { 1.0 } else { -1.0 };
        }
        det
    }
    
    fn matrix_minor(matrix: &[Vec<f64>], row: usize, col: usize) -> Vec<Vec<f64>> {
        let mut minor = Vec::new();
        for i in 0..matrix.len() {
            if i != row {
                let mut row_vec = Vec::new();
                for j in 0..matrix[i].len() {
                    if j != col {
                        row_vec.push(matrix[i][j]);
                    }
                }
                minor.push(row_vec);
            }
        }
        minor
    }
    
    // Constants
    pub const PI: f64 = PI;
    pub const E: f64 = E;
    pub const TAU: f64 = TAU;
    pub const SQRT_2: f64 = SQRT_2;
    pub const SQRT_PI: f64 = 1.77245385090551602729816748334114518;
    pub const LN_2: f64 = LN_2;
    pub const LN_10: f64 = LN_10;
    pub const LOG2_E: f64 = LOG2_E;
    pub const LOG10_E: f64 = LOG10_E;
    pub const INFINITY: f64 = f64::INFINITY;
    pub const NEG_INFINITY: f64 = f64::NEG_INFINITY;
    pub const NAN: f64 = f64::NAN;
    
    // Utility functions
    pub fn is_finite(x: f64) -> bool { x.is_finite() }
    pub fn is_infinite(x: f64) -> bool { x.is_infinite() }
    pub fn is_nan(x: f64) -> bool { x.is_nan() }
    pub fn clamp(x: f64, min: f64, max: f64) -> f64 { x.max(min).min(max) }
    pub fn lerp(a: f64, b: f64, t: f64) -> f64 { a + t * (b - a) }
    pub fn smoothstep(edge0: f64, edge1: f64, x: f64) -> f64 {
        let t = Self::clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }
}
