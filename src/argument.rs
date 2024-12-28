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
    #[arg(short, long, default_value = "font.ttf", help = "filename to font to use")]
    pub font: String,

    /// path to directory that holds all the logos
    #[arg(short, long, default_value = "./logos", help = "path to directory that holds all the logos")]
    pub logos: String,

    /// path to directory which stores the origin images
    #[arg(short, long, default_value = "./input", help = "path to directory which stores the origin images")]
    pub input: String,

    /// path to directory where the outcomes will be stored
    #[arg(short, long, default_value = "./output", help = "path to directory where the outcomes will be stored")]
    pub output: String,

    /// painter that defines the instant-film layout
    #[arg(short, long, default_value = None, help = "optional, painter that defines the instant-film layout, use `tri-angular` as default")]
    pub painter: Option<String>,

    /// whether add all paddings around the image
    #[arg(long = "pos", default_value = None, help = "optional, where to paint the description content, use [ top/ bottom / left / right / middle ] (t/b/l/r/m for short). For some special painter this won't work, and different painter has their own implementation.")]
    pub position: Option<String>,

    /// whether add all paddings around the image
    #[arg(long = "pad", action = clap::ArgAction::SetTrue, help = "whether add paddings around the image")]
    pub padding: bool,
}
