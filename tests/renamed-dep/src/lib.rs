//! `clml` is pulled in here under the name `mycolor`.
//!
//! The printing macros expand to `$crate::__private::..`, which the compiler
//! resolves to the defining crate regardless of the local alias. A hardcoded
//! `clml::..` path would fail to compile in this crate.

#[cfg(test)]
mod tests {
    use mycolor::{ceprintln, cformat, cformatdoc, cprint, cprintln};

    #[test]
    fn printing_macros_work_under_a_renamed_dependency() {
        let target = "renamed-dep";

        cprint!("<green>ok</green> {target}");
        cprintln!();
        cprintln!("<bold>{target}</bold> built");
        ceprintln!("<red>note</red>: stderr also works");

        // The value-producing macros go through the same crate path.
        assert_eq!(
            cformat!("<red>{target}</red>"),
            "\u{1b}[31mrenamed-dep\u{1b}[39m"
        );
    }

    /// `cformatdoc!` also dedents the format string and keeps implicit named captures (`{target}`)
    /// working under a renamed dependency.
    #[test]
    fn doc_macros_work_under_a_renamed_dependency() {
        let target = "renamed-dep";

        assert_eq!(
            cformatdoc!(
                "
                <red>{target}
                    built</red>
                "
            ),
            "\u{1b}[31mrenamed-dep\n    built\u{1b}[39m\n"
        );
    }
}
