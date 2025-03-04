// TODO: change this to a .txt file and implement this so that you can change the wordlist
pub fn get_wordlist() -> Vec<String> {
    // Fetch 10 years old wordlist provided by KOTUS I found on github.
    // TODO: Do this with an scraper to get better memory usage.
    let body = reqwest::blocking::get("https://raw.githubusercontent.com/Jereson2000/everyfinnishword/refs/heads/master/kaikkisanat.txt").expect("Couldn't make request")
    .text().expect("No text found");

    body.split("\n").map(|s| s.to_string()).collect()
}