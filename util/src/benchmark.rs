
#[macro_export]
macro_rules! benchmark
{
    ( $desc:expr, $num_runs:expr, $num_iters:expr, $code:block ) =>
    {{
        use std::io::*;
        use std::time::*;
        use num_format::*;

        print!("Benchmarking {}", $desc);
        std::io::stdout().flush().unwrap();

        let mut run_times = Vec::new();

        for _ in 0..$num_runs
        {
            print!(".");
            std::io::stdout().flush().unwrap();

            let timer = SystemTime::now();

            for _ in 0..$num_iters
            {
                let _result = $code;
            }

            let duration = timer.elapsed().unwrap().as_micros();
            run_times.push(duration);
        }

        let sum: u128 = run_times.iter().sum();
        let average_run_time = sum / (run_times.len() as u128);

        println!(" Done!\nTook an average of {} microseconds over {} runs of {} iterations.",
            average_run_time.to_formatted_string(&Locale::en),
            $num_runs, $num_iters);
    }};
}
