use cg_ufpel_project;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "OpenGL Project",
    author = "Jonathas-Conceicao <jadoliveira@inf.ufpel.edu.br>",
    about = "OpenGL project in Rust"
)]
struct Opt {
    #[structopt(short = "w", long = "width", default_value = "800")]
    scr_width: u32,
    #[structopt(short = "h", long = "height", default_value = "600")]
    scr_height: u32,
    #[structopt(short = "m", long = "models", default_value = "1")]
    n_models: usize,
    #[structopt(
        short = "c",
        long = "config",
        default_value = "configs/model_config.json"
    )]
    config: PathBuf,
}

pub fn run() -> Result<(), failure::Error> {
    let opt = Opt::from_args();
    let mut scene =
        cg_ufpel_project::Scene::init(opt.scr_width, opt.scr_height, opt.n_models, opt.config)?;
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
