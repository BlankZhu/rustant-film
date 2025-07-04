use std::fmt::Display;

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
#[command(
    name = "rustant-film",
    version = "0.1",
    author = "BlankZhu",
    about = "Add a instant film style layout to your EXIF photo."
)]
pub struct Arguments {
    /// filename to main font to use
    #[arg(short, long, default_value = "font.ttf", help = "filename to font to use")]
    pub font: String,

    /// filename to sub font to use
    #[arg(long = "sub-font", default_value = None, help = "optional, filename to sub font to use")]
    pub sub_font: Option<String>,

    /// path to directory that holds all the logos
    #[arg(short, long, default_value = "./logos", help = "path to directory that holds all the logos")]
    pub logos: String,

    /// mode of rustant-film, default is command
    #[arg(short, long, default_value = "command", help = "working mode of rustant-film, use server mode if set as `server`")]
    pub mode: String,

    /// port for server mode
    #[arg(long, default_value = "6400", help = "port for server mode")]
    pub port: u32,

    /// path to directory which stores the origin images
    #[arg(short, long, default_value = "./input", help = "path to directory which stores the origin images")]
    pub input: String,

    /// path to directory where the outcomes will be stored
    #[arg(short, long, default_value = "./output", help = "path to directory where the outcomes will be stored")]
    pub output: String,

    /// painter that defines the instant-film layout
    #[arg(short, long, default_value = None, help = "optional, painter that defines the instant-film layout, use `triangular` as default")]
    pub painter: Option<String>,

    /// whether add all paddings around the image
    #[arg(long = "pos", default_value = None, help = "optional, where to paint the description content, use [top/bottom/left/right/middle] (t/b/l/r/m for short). For some special painter this won't work, and different painter has their own implementation.")]
    pub position: Option<String>,

    /// whether add all paddings around the image
    #[arg(long = "pad", action = clap::ArgAction::SetTrue, help = "whether add paddings around the image")]
    pub padding: bool,
}

impl Display for Arguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Font: {}, Sub-Font: {}, Logos: {}, mode: {}, port: {}, input: {}, output: {}, painter: {}, position: {}, padding: {}",
            self.font.as_str(),
            self.sub_font.as_ref().unwrap_or(&"(None)".to_string()),
            self.logos.as_str(),
            self.mode.as_str(),
            self.port,
            self.input.as_str(),
            self.output.as_str(),
            self.painter.as_ref().unwrap_or(&"(None)".to_string()),
            self.position.as_ref().unwrap_or(&"(None)".to_string()),
            self.padding,
        )
    }
}
