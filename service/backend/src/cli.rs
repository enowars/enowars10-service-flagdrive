use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "FlagDrive Backend", long_about = None)]
pub struct Args {
    #[arg(long = "database", short = 'd', default_value = "flagdrive.db")]
    pub database: String,

    #[arg(default_value = "0.0.0.0:4859")]
    pub addr: String,
}
