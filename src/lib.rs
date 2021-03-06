#![feature(box_syntax)]

fn formatting_is_broken_too()
{
    if { true } { /* oh and trigger clippy */}

    use std::os::unix; // and this will only be caught when running on windows

    std::time::Duration::ZERO; // will fail on pre-1.53 rust versions (for testing MSRV)
}

#[cfg(not(feature = "myfeature"))]
fn what() {
    ThisWillOnlyFailToCompileIfMyfeatureIsNotIncluded
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_doesnt_work() {
        assert_eq!(2 + 2, 3);
    }
}
