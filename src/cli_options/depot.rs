use bpaf::{construct, OptionParser, Parser};

#[derive(Clone, Debug)]
pub enum DepotOptions {
    Download {
        //
    },
}

pub fn depot() -> OptionParser<DepotOptions> {
    let download = download()
        .to_options()
        .descr("Download depot files")
        .command("download");

    construct!([download]).to_options()
}

fn download() -> impl Parser<DepotOptions> {
    //

    construct!(DepotOptions::Download {})
}
