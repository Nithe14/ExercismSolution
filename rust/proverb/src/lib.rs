pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => String::new(),
        1 => format!("And all for the want of a {}.", list[0]),
        _ => {
            let mut proverb = String::new();

            let windows = list.windows(2);
            for window in windows {
                proverb.push_str(
                    format!("For want of a {} the {} was lost.\n", window[0], window[1]).as_str(),
                );
            }
            proverb.push_str(format!("And all for the want of a {}.", list[0]).as_str());

            proverb
        }
    }
}
