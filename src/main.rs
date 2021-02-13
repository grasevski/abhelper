use clap::Clap;
use statrs::distribution::{Normal, Univariate};
use statrs::statistics::{Mean, Variance};

#[derive(Clap)]
enum Command {
    /// Used for testing a rate eg conversion rate
    Binomial {
        /// Conversions for A
        c1: f64,
        /// Conversions for B
        c2: f64,
    },
    /// Used for testing a continuous value eg revenue
    Normal {
        /// Mean for A
        x1: f64,
        /// Mean for B
        x2: f64,
        /// Variance for A
        v1: f64,
        /// Variance for B
        v2: f64,
    },
}

/// Calculate uplift and p value for an AB test
#[derive(Clap)]
struct Opts {
    /// Sample size A
    n1: f64,
    /// Sample size B
    n2: f64,
    /// AB test results probability distribution
    #[clap(subcommand)]
    cmd: Command,
}

/// Calculate p value based on z test
fn ztest(a: Normal, b: Normal) -> (f64, f64) {
    (
        Normal::new(0.0, 1.0)
            .unwrap()
            .cdf((b.mean() - a.mean()) / (a.variance() + b.variance()).sqrt()),
        b.mean() / a.mean() - 1.0,
    )
}

fn main() {
    let opts = Opts::parse();
    let (p, uplift) = match opts.cmd {
        Command::Binomial { c1, c2 } => {
            let (p1, p2) = (c1 / opts.n1, c2 / opts.n2);
            ztest(
                Normal::new(p1, (p1 * (1.0 - p1) / opts.n1).sqrt()).unwrap(),
                Normal::new(p2, (p2 * (1.0 - p2) / opts.n2).sqrt()).unwrap(),
            )
        }
        Command::Normal { x1, x2, v1, v2 } => ztest(
            Normal::new(x1, (v1 / opts.n1).sqrt()).unwrap(),
            Normal::new(x2, (v2 / opts.n2).sqrt()).unwrap(),
        ),
    };
    println!("{} {}", p, uplift)
}
