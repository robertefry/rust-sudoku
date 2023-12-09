
use std::io::{self, Write};
use rayon::prelude::*;
use chrono::prelude::*;

macro_rules! timed_activity
{
    ($desc:expr, $code:block) =>
    {{
        print!("{}...", $desc);
        io::stdout().flush().unwrap();

        let start_time = Local::now();

        let _result = $code;

        let duration = Local::now().signed_duration_since(start_time);
        println!(" Done! Took {} seconds.", duration.num_seconds());
    }};
}

fn main()
{
    let dataset;
    timed_activity!("Loading the dataset",
    {
        dataset = sudoku::load_dataset();
    });

    timed_activity!("Beginning the benchmark",
    {
        dataset.par_iter().for_each(|(sudoku,_)|
        {
            let _ = sudoku::solve(&sudoku);
        });
    });
}
