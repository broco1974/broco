use clap;

error_chain! {
    foreign_links {
        Clap(clap::Error);
    }
}
