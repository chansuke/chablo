use chablo::builder::build;
use chablo::cli::{ChabloOpt, Commands};
use chablo::errors::ChabloError;
use chablo::server::run;

fn main() -> Result<(), ChabloError> {
    env_logger::init();

    let args = ChabloOpt::parse();
    match args.command {
        Commands::Convert { file: _ } => Ok(()),
        Commands::Build => build(),
        Commands::Serve => run(),
    }
}
