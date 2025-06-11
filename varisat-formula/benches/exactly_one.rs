use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;
use varisat_formula::{tools::*, Var};


const MIN_TWOEXP: usize = 10;
const MAX_TWOEXP: usize = 12;

pub fn criterion_benchmark(c: &mut Criterion) {
	let sizes = (MIN_TWOEXP..=MAX_TWOEXP)
		.map(|exp| 2usize.pow(exp as u32))
		.map(|size| (size, (1..=size).map(Var::from_index).collect()))
		.collect::<Vec<(usize, Vec<Var>)>>();

	macro_rules! twoexp_group {
		(
			$(
				$group_name:expr => [$($bench_name:expr => $routine:expr),*]
			),*
		) => {
				$(
					let mut group = c.benchmark_group($group_name);
					// group.measurement_time(std::time::Duration::from_secs(TARGET_TIME));
					for (size, vars) in &sizes {
						$(
							group.bench_with_input(BenchmarkId::new($bench_name, size), size, |b, &_| {
								b.iter(|| $routine(black_box(&vars)));
							});
						)*
					}
					group.finish();
				)*
		};
	}
	twoexp_group![
		"exactly_one" => [
			"exactly_one" => exactly_one,
			"exactly_one_rayon" => exactly_one_rayon,
			"exactly_one_rayon_simd_hint" => exactly_one_rayon_simd_hint
		],
		"no_two_true" => [
			"no_two_true" => no_two_true,
			"no_two_true_rayon" => no_two_true_rayon
		],
		"all_true" => [
			"all_true" => all_true,
			"all_true_rayon" => all_true_rayon,
			"all_true_simd_hint" => all_true_simd_hint
		]
	];

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
