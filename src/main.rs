use chablo::builder::build;
use chablo::cli::{ChabloOpt, Commands};
use chablo::errors::ChabloError;

fn main() -> Result<(), ChabloError> {
    let args = ChabloOpt::parse();
    match args.command {
        Commands::Convert { file: _ } => Ok(()),
        Commands::Build {} => build(),
    }
}
