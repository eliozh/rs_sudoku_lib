use criterion::{criterion_group, criterion_main, Criterion};

use rs_sudoku_lib::sudoku::{BaseSudoku, Board};

fn bench_play_game(c: &mut Criterion) {
    c.bench_function("bench_generate_board", |b| {
        b.iter(|| {
            std::hint::black_box(for i in 1..=1000 {
                Board::new(3, 10).unwrap().generate_board(i);
            });
        });
    });
}

criterion_group!(benches, bench_play_game,);
criterion_main!(benches);
