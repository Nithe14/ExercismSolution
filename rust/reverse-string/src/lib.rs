use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut reversed = String::new();
    let unicode_vec = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
    unicode_vec.iter().rev().for_each(|x| reversed.push_str(x));
    reversed
    //unimplemented!("Write a function to reverse {input}");
}
