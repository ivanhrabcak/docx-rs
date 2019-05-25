extern crate docx;
#[macro_use]
extern crate docx_codegen;
#[macro_use]
extern crate log;
extern crate quick_xml;

use docx::errors::{Error, Result};
use quick_xml::{Reader, Writer};
use std::io::Cursor;

#[derive(Xml, PartialEq, Debug)]
#[xml(event = "Start")]
#[xml(tag = "tag1")]
struct Tag1 {
    #[xml(attr = "att1")]
    att1: Option<String>,
    #[xml(text)]
    content: String,
}

#[derive(Xml, PartialEq, Debug)]
#[xml(event = "Empty")]
#[xml(tag = "tag2")]
struct Tag2 {
    #[xml(attr = "att1")]
    att1: String,
    #[xml(attr = "att2")]
    att2: String,
}

#[derive(Xml, PartialEq, Debug)]
#[xml(event = "Start")]
#[xml(tag = "tag3")]
struct Tag3 {
    #[xml(attr = "att1")]
    att1: String,
    #[xml(child)]
    #[xml(tag = "tag1")]
    tag1: Vec<Tag1>,
    #[xml(child)]
    #[xml(tag = "tag2")]
    tag2: Option<Tag2>,
    #[xml(flatten_text)]
    #[xml(tag = "text")]
    text: Option<String>,
    #[xml(flatten_empty)]
    #[xml(tag = "tag4")]
    tag4: bool,
    #[xml(flatten_empty)]
    #[xml(tag = "tag5")]
    #[xml(attr = "att1")]
    tag5: Option<String>,
}

#[derive(Xml, PartialEq, Debug)]
enum Tag {
    #[xml(event = "Start")]
    #[xml(tag = "tag1")]
    Tag1(Tag1),
    #[xml(event = "Empty")]
    #[xml(tag = "tag2")]
    Tag2(Tag2),
    #[xml(event = "Start")]
    #[xml(tag = "tag3")]
    Tag3(Tag3),
}

macro_rules! assert_write_eq {
    ($l:tt, $r:expr) => {
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        $r.write(&mut writer).unwrap();
        let result = writer.into_inner().into_inner();

        assert_eq!($l, String::from_utf8(result).unwrap());
    };
}

macro_rules! assert_read_eq {
    ($t:tt, $l:tt, $r:expr) => {
        let mut reader = Reader::from_str($l);
        reader.trim_text(true);

        assert_eq!($t::read(&mut reader, None).unwrap(), $r);
    };
}

#[test]
fn test_write() {
    assert_write_eq!(
    r#"<tag3 att1="att1"><tag1 att1="tag1_att1">tag1_content</tag1><tag2 att1="tag2_att1" att2="tag2_att2"/></tag3>"#,
    Tag3 {
      att1: String::from("att1"),
      tag1: vec![Tag1 {
        att1: Some(String::from("tag1_att1")),
        content: String::from("tag1_content"),
      }],
      tag2: Some(Tag2 {
        att1: String::from("tag2_att1"),
        att2: String::from("tag2_att2"),
      }),
      text: None,
      tag4: false,
      tag5: None,
    }
  );

    assert_write_eq!(
    r#"<tag3 att1="att1"><tag1>tag1_content</tag1><text>tag3_content</text><tag4/><tag5 att1="tag5_att1"/></tag3>"#,
    Tag3 {
      att1: String::from("att1"),
      tag1: vec![Tag1 {
        att1: None,
        content: String::from("tag1_content"),
      }],
      tag2: None,
      text: Some(String::from("tag3_content")),
      tag4: true,
      tag5: Some(String::from("tag5_att1")),
    }
  );

    assert_write_eq!(
    r#"<tag3 att1="att1"><tag1>content</tag1><tag1>tag1</tag1><text>tag3_content</text><tag5 att1="tag5_att1"/></tag3>"#,
    Tag3 {
      att1: String::from("att1"),
      tag1: vec![
        Tag1 {
          att1: None,
          content: String::from("content"),
        },
        Tag1 {
          att1: None,
          content: String::from("tag1"),
        },
      ],
      tag2: None,
      text: Some(String::from("tag3_content")),
      tag4: false,
      tag5: Some(String::from("tag5_att1")),
    }
  );

    assert_write_eq!(
        r#"<tag1>tag1_content</tag1>"#,
        Tag::Tag1(Tag1 {
            att1: None,
            content: String::from("tag1_content"),
        })
    );
}

#[test]
fn test_read() {
    assert_read_eq!(
    Tag3,
    r#"<tag3 att1="att1"><text>tag3_content</text><tag2 att2="att2" att1="att1"/><tag1 att1="att1">content</tag1></tag3>"#,
    Tag3 {
      att1: String::from("att1"),
      tag1: vec![Tag1 {
        att1: Some(String::from("att1")),
        content: String::from("content"),
      }],
      tag2: Some(Tag2 {
        att1: String::from("att1"),
        att2: String::from("att2"),
      }),
      text: Some(String::from("tag3_content")),
      tag4: false,
      tag5: None,
    }
  );

    assert_read_eq!(
    Tag3,
    r#"<tag3 att1="att1"><tag1>content</tag1><text>tag3_content</text><tag4/><tag5 att1="tag5_att1"/></tag3>"#,
    Tag3 {
      att1: String::from("att1"),
      tag1: vec![Tag1 {
        att1: None,
        content: String::from("content"),
      }],
      tag2: None,
      tag4: true,
      text: Some(String::from("tag3_content")),
      tag5: Some(String::from("tag5_att1")),
    }
  );

    assert_read_eq!(
    Tag3,
    r#"<tag3 att1="att1"><tag1 att1="att11">content1</tag1><tag1 att1="att12">content2</tag1></tag3>"#,
    Tag3 {
      att1: String::from("att1"),
      tag1: vec![
        Tag1 {
          att1: Some(String::from("att11")),
          content: String::from("content1"),
        },
        Tag1 {
          att1: Some(String::from("att12")),
          content: String::from("content2"),
        },
      ],
      tag2: None,
      tag4: false,
      text: None,
      tag5: None,
    }
  );

    assert_read_eq!(
        Tag,
        r#"<tag1 att1="att1">content</tag1>"#,
        Tag::Tag1(Tag1 {
            att1: Some(String::from("att1")),
            content: String::from("content"),
        })
    );

    assert_read_eq!(
        Tag,
        r#"<tag2 att2="att2" att1="att1"/>"#,
        Tag::Tag2(Tag2 {
            att1: String::from("att1"),
            att2: String::from("att2"),
        })
    );
}
