
use rayon::prelude::*;

fn main()
{
    sudoku::initialise();

    let dataset = sudoku::dataset::load();

    util::benchmark!("sudoku solve", 10, 1,
    {
        dataset.par_iter().for_each(|(sudoku,_)|
        {
            let _ = sudoku::solve(&sudoku);
        });
    });
}
