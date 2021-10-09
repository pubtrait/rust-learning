use clap::App;

fn main() {
    App::new("testcmd")
        .version("1.0")
        .about("test thing")
        .author("mobus")
        .get_matches();
}
