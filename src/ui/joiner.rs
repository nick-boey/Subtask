/// Represents a visual joiner element between tasks.
pub(crate) struct Joiner {}

impl Joiner {
    /// Renders a joiner between two tasks provided their depths.
    /// Doesn't render joiners longer than three characters.
    pub(crate) fn create(depth_above: i8, depth_below: i8) -> Option<String> {
        let depth_diff = depth_below - depth_above;

        if !(-1..=1).contains(&depth_diff) {
            return None;
        }

        match depth_diff {
            1 => {
                // Create a ╰─╮ joiner
                Some(String::from("╰─╮"))
            }
            0 => {
                // Create a │ joiner
                Some(String::from("│"))
            }
            -1 => {
                // Create a ╭─╯ joiner
                Some(String::from("╭─╯"))
            }
            _ => None,
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn create_joiner_at_depths_correct_joiner() {
        assert_eq!(&Joiner::create(0, 1).unwrap(), "╰─╮");
        assert_eq!(&Joiner::create(1, 0).unwrap(), "╭─╯");
        assert_eq!(&Joiner::create(0, 0).unwrap(), "│");
        assert_eq!(&Joiner::create(1, 2).unwrap(), "╰─╮");
        assert!(Option::is_none(&Joiner::create(1, 3)));
        assert!(Option::is_none(&Joiner::create(1, 3)));
    }
}
