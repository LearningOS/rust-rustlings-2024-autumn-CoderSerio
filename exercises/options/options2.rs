// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        // word = optional_target {
        //     assert_eq!(word, target);
        // }
        if let optional_target = target {
            let word = optional_target;
            assert_eq!(word, target)
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        // integer = optional_integers.pop() {
        //     assert_eq!(integer, cursor);
        //     cursor -= 1;
        // }
        while let Some(maybe_integer) = optional_integers.pop() {
            if let Some(integer) = maybe_integer {
                assert_eq!(integer, cursor);
                cursor -= 1;
            } else {
                // 如果 maybe_integer 是 None，跳过这一轮循环
                continue;
            }
        }

        assert_eq!(cursor, 0);
    }
}
