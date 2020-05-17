use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BlockTime(pub u32);

impl From<BlockTime> for u32 {
    fn from(value: BlockTime) -> Self {
        value.0
    }
}

impl fmt::Display for BlockTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use chrono::format::{Item, Fixed};
        use chrono::offset::TimeZone;

        fmt::Display::fmt(&chrono::Utc
                .timestamp(i64::from(self.0), 0u32)
                .format_with_items(std::iter::once(Item::Fixed(Fixed::RFC2822))), f)
    }
}
