
use rayon::prelude::*;

mod benchmark;

fn main()
{
    let dataset = sudoku::load_dataset();

    benchmark!("sudoku solve", 10, 1,
    {
        dataset.par_iter().for_each(|(sudoku,_)|
        {
            let _ = sudoku::solve(&sudoku);
        });
    });
}
