/// Helper function to let Serde set a default value of `true`. Check this
/// [GitHub issue](https://github.com/serde-rs/serde/issues/368) for more information.
pub(crate) fn default_true() -> bool {
    true
}
