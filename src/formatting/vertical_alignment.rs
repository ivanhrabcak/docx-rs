use strong_xml::{XmlRead, XmlWrite};

use crate::__string_enum;

#[derive(Debug, Default, XmlRead, XmlWrite)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:vertAlign")]
pub struct VerticalAlignment {
    #[xml(attr = "w:val")]
    pub val: Option<VerticalAlignmentType>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum VerticalAlignmentType {
    Superscript,
    Subscript,
}

impl VerticalAlignment {
    pub fn superscript() -> Self {
        Self {
            val: Some(VerticalAlignmentType::Superscript),
        }
    }

    pub fn subscript() -> Self {
        Self {
            val: Some(VerticalAlignmentType::Subscript),
        }
    }
}

__string_enum! {
    VerticalAlignmentType {
        Superscript = "superscript",
        Subscript = "subscript",
    }
}
