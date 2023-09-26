use crate::models::Rect;
use crate::models::style::{StyledLine, StyledSpan, StyledText};

pub struct Status {
    rect: Rect,
}

impl Status {
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
        }
    }

    pub fn create_bar<'a>(self, name: StyledText<'a>, first: StyledText<'a>, second: StyledText<'a>) -> StyledText<'a> {
        let mut bar = StyledLine::new();

        bar.extend(name.into());
        bar.push(StyledSpan::from(" "));
        bar.extend(first.into());

        //let remaining = self.rect.width - bar.len() * 2;

        //bar.push(StyledSpan::from(" ".repeat(remaining)));
        bar.extend(second.into());

        bar.into()
    }
}
