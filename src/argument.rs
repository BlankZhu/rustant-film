use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "rustant-film",
    version = "0.1",
    author = "BlankZhu",
    about = "Add a instant film style layout to your EXIF photo."
)]
pub struct Arguments {
    /// filename to font to use
    #[arg(short, long, default_value = "font.ttf")]
    pub font: String,

    #[arg(short, long, default_value = "./logos")]
    pub logos: String,

    #[arg(short, long, default_value = "./input")]
    pub input: String,

    #[arg(short, long, default_value = "./output")]
    pub output: String,

    #[arg(short, long, default_value = "normal")]
    pub painter: String,
}
