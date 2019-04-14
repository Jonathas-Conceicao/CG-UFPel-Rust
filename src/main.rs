use cg_ufpel_project;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "OpenGL Project",
    author = "Jonathas-Conceicao <jadoliveira@inf.ufpel.edu.br>",
    about = "OpenGL project in Rust"
)]
struct Opt {
    #[structopt(short = "m", long = "models", default_value = "1")]
    n_models: usize,
}

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn run() -> Result<(), failure::Error> {
    let opt = Opt::from_args();
    let mut scene = cg_ufpel_project::Scene::init(SCR_WIDTH, SCR_HEIGHT, opt.n_models)?;
    scene.run()
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
