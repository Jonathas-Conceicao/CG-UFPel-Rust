use cg_ufpel_project;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "OpenGL Project",
    author = "Jonathas-Conceicao <jadoliveira@inf.ufpel.edu.br>",
    about = "OpenGL project in Rust"
)]
struct Opt {
    #[structopt(short = "m", long = "models")]
    n_models: usize,
}

pub fn run() -> Result<(), failure::Error> {
    cg_ufpel_project::run()
}

fn main() {
    if let Err(ref e) = run() {
        eprintln!("{}", e);
        e.iter_causes()
            .skip(1)
            .for_each(|e| eprintln!(" caused by: {}\n", e));

        std::process::exit(1);
    }
}
