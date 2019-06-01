#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;
use freja_parser::*;
use freja_vm::vm::VM;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn fib() {
    let mut vm = VM::new();

    vm.eval_script(
        r#"fn fib(num) {
            if num <= 1 {
                return 1
            }
            return fib(num - 1) + fib(num - 2)
        }
        fib(20)"#,
    )
    .unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fib()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
