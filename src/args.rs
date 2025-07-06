use clap::{//Args,
     Parser//,
     //Subcommand
};

#[derive(Debug, Parser)]
pub struct CliArgs{
    /// path to first pdf file
    pub file_path_1: std::path::PathBuf,

    /// page numbers of the first file [optional]
    #[arg(short = 'f', long = "pages-first")]
    pub pages_1: Option<String>,

    /// path to second pdf file
    pub file_path_2: std::path::PathBuf,

    /// page numbers of the second file [optional]
    #[arg(short = 's', long = "pages-second")]
    pub pages_2: Option<String>,

    /// path to output location [optional]
    #[arg(short = 'o',long = "output")]
    pub output_path: Option<std::path::PathBuf>,
}