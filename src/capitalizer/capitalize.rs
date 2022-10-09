use regex::Regex;

pub fn capitalize(string: &str) -> String {
    let s = string.to_lowercase(); // lowercase to make the string uniform
    let s_bind = &s.split_whitespace().collect::<Vec<&str>>(); // split string at whitespace, store into an array
    let find_first_letter = Regex::new(r"\b(\w)").unwrap(); // "take every character after the word boundary"
    let mut return_this = String::new(); // an empty string for storing away the modified string

    for i in s_bind {
        // iterate through array (s_bind; splitted at whitespace strings)
        return_this.push_str(
            &find_first_letter
                .replace(i, i.chars().next().unwrap().to_uppercase().to_string())
                .to_string()
                .chars()
                .collect::<String>(),
        ); // push the matched and uppercased character (replaced) to the empty string, join together with the unmodified string
        return_this.push_str(" "); // join a whitespace
    }
    return_this.pop(); // because of the loop, there will be an extra whitespace at the end of the string. Delete it.

    return return_this; // this should now capitalize the first letter of each given set of string. Correctly.
}