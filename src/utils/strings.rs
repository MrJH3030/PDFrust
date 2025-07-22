pub enum StandardString {
    AddFile,
    SuccessfullyMerged,
    MoveUp,
    ChooseFile,
    ChooseCurrentFolder,
    ChooseOutputFolder,
    SelectPages,
    EnterOutputFileName,
    InvalidFileName,
    PdfLowerCase,
    PdfUpperCase,
    PatternHelperMessage,
    PdfHelperMessage,
}

pub fn standard_string(string: StandardString) -> &'static str {
    match string {
        StandardString::MoveUp => "..",
        StandardString::ChooseFile => "Choose a file !",
        StandardString::ChooseCurrentFolder => "[Choose current folder]",
        StandardString::ChooseOutputFolder => "Choose an output folder!",
        StandardString::SelectPages => "Please selct pages",
        StandardString::EnterOutputFileName => "Enter output file name!",
        StandardString::InvalidFileName => "Invalid file name",
        StandardString::PdfLowerCase => ".pdf",
        StandardString::PdfUpperCase => ".PDF",
        StandardString::PatternHelperMessage =>"Like 1,2,3,4-5",
        StandardString::PdfHelperMessage => "Must end in .PDF or .pdf",
        StandardString::AddFile => "Add another file?",
        StandardString::SuccessfullyMerged => "Successfully merged documents!",
    }
}
