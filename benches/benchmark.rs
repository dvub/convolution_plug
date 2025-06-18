use std::sync::Arc;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use fundsp::hacker32::*;

use convolution_plug::dsp::param_nodes::gain;
use convolution_plug::params::PluginParams;

fn render_noise(_dummy: usize) -> Wave {
    Wave::render(44110.0, 1.0, &mut (noise()))
}

fn render_with_params(_dummy: usize, p: &Arc<PluginParams>) -> Wave {
    Wave::render(44100.0, 1.0, &mut (noise() * gain(p)))
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let default_params = PluginParams::default();
    let p = Arc::new(default_params);

    c.bench_function("render_noise", |b| b.iter(|| render_noise(black_box(0))));

    c.bench_with_input(BenchmarkId::new("input_example", &p), &p, |b, s| {
        b.iter(|| render_with_params(black_box(0), s))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
