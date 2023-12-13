
use rayon::prelude::*;

fn main()
{
    sudoku::initialise();

    let dataset = sudoku::dataset::first(1_000);

    util::benchmark!("sudoku solve", 100, 10,
    {
        dataset.par_iter().for_each(|(sudoku,_)|
        {
            let _ = sudoku::solve(&sudoku);
        });
    });
}
