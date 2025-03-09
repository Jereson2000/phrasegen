pub struct Settings {
    wordlistpath: String,
    delimiter: char,
    wordcount: usize,
}

// TODO: implement the from function for the settings to be taken from config file.
// TODO: implement setters for Settings.
impl Settings {
    // Constructor for settings.
    pub fn new() -> Self {
        let os = std::env::consts::OS;

        let path: String = match os {
            "windows" => String::from(r".\kaikkisanat.txt"),
            "linux" => String::from("./kaikkisanat.txt"),
            otheros => panic!("Support not implemented for {otheros}")
        };

        Self { wordlistpath: path, delimiter: '-', wordcount: 4}
    }
     // Getters (read access)
     pub fn wordlist(&self) -> &String {
        &self.wordlistpath
    }

    pub fn delimiter(&self) -> char {
        self.delimiter
    }

    pub fn wordcount(&self) -> usize {
        self.wordcount
    }
}
