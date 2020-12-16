use clap::{App, AppSettings, Arg, ArgMatches};

pub fn get_matches<'a>() -> ArgMatches<'a> {
    App::new("rust.fm")
        .global_setting(AppSettings::ColoredHelp)
        .author("Jacek Chmielewski <jchmielewski@teonite.com>")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("frequency")
                .required(true)
                .help("FM frequency to play"),
        )
        .get_matches()
}
