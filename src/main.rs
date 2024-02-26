mod sites;
mod utils;

use clap::Parser;
use sites::smbcnikko::smbcnikko;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = false)]
    athome: bool,
    #[arg(long, default_value_t = false)]
    beer: bool,
    #[arg(long, default_value_t = false)]
    cybernet: bool,
    #[arg(long, default_value_t = false)]
    ena: bool,
    #[arg(long, default_value_t = false)]
    goonet: bool,
    #[arg(long, default_value_t = false)]
    ryugaku: bool,
    #[arg(long, default_value_t = false)]
    smbcnikko: bool,
    #[arg(long, default_value_t = false)]
    soccer: bool,
    #[arg(long, default_value_t = false)]
    webtan: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.smbcnikko {
        smbcnikko("smbcnikko.json").await;
    }
}
