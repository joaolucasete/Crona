use clap::clap_app;

fn main() {
    let matches = clap_app!(CronaVM =>
        (version: "0.0.1")
        (author: "Chiyoku <chiyokuoni@gmail.com>")
        (about: "Runs crona binaries")
        (@arg INPUT: +required "Sets the input file to use")
    )
    .get_matches();

    let _ = matches.value_of("INPUT").unwrap();
}
