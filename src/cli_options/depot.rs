use bpaf::{construct, OptionParser, Parser};

#[derive(Clone, Debug)]
pub enum Depot {
    Download {
        //
    },
}

pub fn depot() -> OptionParser<Depot> {
    let download = download()
        .to_options()
        .descr("Download depot files")
        .command("download");

    construct!([download]).to_options()
}

fn download() -> impl Parser<Depot> {
    //

    construct!(Depot::Download {})
}
