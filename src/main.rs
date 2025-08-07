use crate::text_decomposition::ArgsEntity;
use crate::text_decomposition::TextDiff;

mod text_decomposition;

fn main() {
    let original_text: String = std::env::args().nth(1).expect("No original text given");
    let modified_text: String = std::env::args().nth(1).expect("No modified text given");

    let args: ArgsEntity = ArgsEntity {
        original_text,
        modified_text,
    };

    let mut current_text_diff: Option<TextDiff> = None;

    let pb: indicatif::ProgressBar = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        current_text_diff = Some(args.clone().get_text_diff());
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");

    if let Some(text_diff_result) = current_text_diff {
        text_diff_result.printer();
    } else {
        panic!();
    }

    
}

#[cfg(test)]
mod tests {

    #[test]
    fn case1() {
        assert_eq!(1, 1);
    }
}
