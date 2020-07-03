/// Bitcoin block version
///
/// This newtype doesn't enforce any validity constraints, but it's still useful for making code
/// more clear and for avoiding assigning semantically different vriables.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct BlockVersion(pub u32);

impl From<BlockVersion> for u32 {
    fn from(value: BlockVersion) -> Self {
        value.0
    }
}
