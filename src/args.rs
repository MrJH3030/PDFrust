use clap::{
     Parser,
     Subcommand
};

#[derive(Debug, Parser)]
pub struct CliArgs{

    #[clap(subcommand)]
    pub command: Commands,
    
}

#[derive(Debug, Subcommand)]
 pub enum Commands{

    Merge{
        /// path to first pdf file
        file_path_1: std::path::PathBuf,

        /// page numbers of the first file [optional]
        #[arg(short = 'f', long = "pages-first")]
        page_string_1: Option<String>,

        /// path to second pdf file
        file_path_2: std::path::PathBuf,

        /// page numbers of the second file [optional]
        #[arg(short = 's', long = "pages-second")]
        page_string_2: Option<String>,

        /// path to output location [optional]
        #[arg(short = 'p',long = "output_path")]
        output_path: Option<std::path::PathBuf>,

        /// output filename [optional]
        #[arg(short = 'o',long = "output_name")]
        output_name: Option<String>,
    },

    Browse{

    },

    Delete{

    },

}