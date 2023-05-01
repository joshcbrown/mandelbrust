use crate::opts::Interval;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl ToString for Complex {
    fn to_string(&self) -> String {
        format!("{} + {}i", self.re, self.im)
    }
}

impl Default for Complex {
    fn default() -> Self {
        Self { re: 0., im: 0. }
    }
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub fn id() -> Self {
        Self::default()
    }

    pub fn mandelbrot_iter(&self, c: &Self) -> Self {
        Self {
            re: (self.re - self.im) * (self.re + self.im) + c.re,
            im: 2. * self.re * self.im + c.im,
        }
    }

    pub fn abs_value_sq(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    pub fn inverse(self) -> Self {
        let norm = self.abs_value_sq();
        Self::new(self.re / norm, -self.im / norm)
    }

    pub fn escape_count(&self, z_0: Self, bound: f64, max_iters: usize) -> (usize, Self) {
        if z_0.abs_value_sq() > bound {
            return (0, z_0);
        }
        let bound_sq = bound.powf(2.);
        let mut z_iter = z_0;
        for iter in 1..=max_iters {
            z_iter = z_iter.mandelbrot_iter(self);
            if z_iter.abs_value_sq() > bound_sq {
                return (iter, z_iter);
            }
        }
        return (max_iters, z_iter);
    }
}

pub fn generate_escape_counts<F>(
    x_range: &Interval,
    y_range: &Interval,
    width: usize,
    height: usize,
    max_iters: usize,
    post_fn: F,
) -> Vec<Vec<f64>>
where
    F: Fn(usize, Complex) -> f64 + std::marker::Sync,
{
    (0..width)
        .into_par_iter()
        .map(|x| {
            (0..height)
                .into_par_iter()
                .map(|y| {
                    let re = x_range.lerp(x as f64 / width as f64);
                    let im = y_range.lerp(y as f64 / height as f64);
                    let c = Complex::new(re, im);
                    let (escape_count, escape_num) = c.escape_count(Complex::id(), 2., max_iters);
                    post_fn(escape_count, escape_num)
                })
                .collect()
        })
        .collect()
}

pub fn normalise_escape_counts(escape_counts: &Vec<Vec<f64>>, max_iters: usize) -> Vec<Vec<f64>> {
    escape_counts
        .into_par_iter()
        .map(|col| {
            col.into_par_iter()
                .map(|&val| val / max_iters as f64)
                .collect()
        })
        .collect()
}

pub fn generate_hist_counts(
    escape_counts: &Vec<Vec<f64>>,
    max_iters: usize,
    total_points: usize,
) -> Vec<Vec<f64>> {
    let pixels_per_iter: &mut Vec<usize> = &mut vec![0; max_iters + 1];
    escape_counts.iter().for_each(|col| {
        col.iter()
            .for_each(|&count| pixels_per_iter[count as usize] += 1)
    });

    let iter_hist: Vec<usize> = (0..=max_iters)
        .into_par_iter()
        .map(|iter| pixels_per_iter[0..=iter].par_iter().sum())
        .collect();

    let total_points = total_points as f64;
    escape_counts
        .into_par_iter()
        .map(|col| {
            col.into_par_iter()
                .map(|&count| {
                    let floored_count = count as usize;
                    if (floored_count) < max_iters {
                        let interval = Interval {
                            lower: iter_hist[floored_count] as f64,
                            upper: iter_hist[floored_count + 1] as f64,
                        };
                        interval.lerp(decimal_part(count)) / total_points
                    } else {
                        iter_hist[floored_count] as f64 / total_points
                    }
                })
                .collect()
        })
        .collect()
}

fn decimal_part(float: f64) -> f64 {
    float - (float as usize) as f64
}
