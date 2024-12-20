//! A parser for the WebVTT (`.vtt`) format provided by [`subtp::vtt::WebVtt`](WebVtt).
//!
//! ## Example
//! ```
//! use subtp::vtt::WebVtt;
//! use subtp::vtt::VttCue;
//! use subtp::vtt::VttTimings;
//! use subtp::vtt::VttTimestamp;
//!
//! let text = r#"WEBVTT
//!
//! 00:00:01.000 --> 00:00:04.000
//! - Never drink liquid nitrogen.
//!
//! 00:00:05.000 --> 00:00:09.000
//! - It will perforate your stomach.
//! - You could die.
//! "#;
//!
//! let vtt = WebVtt::parse(text).unwrap();
//!
//! assert_eq!(
//!     vtt,
//!     WebVtt {
//!         blocks: vec![
//!             VttCue {
//!                 timings: VttTimings {
//!                     start: VttTimestamp {
//!                         seconds: 1,
//!                         ..Default::default()
//!                     },
//!                     end: VttTimestamp {
//!                         seconds: 4,
//!                         ..Default::default()
//!                     },
//!                 },
//!                 payload: vec!["- Never drink liquid nitrogen.".to_string()],
//!                 ..Default::default()
//!             }
//!             .into(),
//!             VttCue {
//!                 timings: VttTimings {
//!                     start: VttTimestamp {
//!                         seconds: 5,
//!                         ..Default::default()
//!                     },
//!                     end: VttTimestamp {
//!                         seconds: 9,
//!                         ..Default::default()
//!                     },
//!                 },
//!                 payload: vec![
//!                     "- It will perforate your stomach.".to_string(),
//!                     "- You could die.".to_string(),
//!                 ],
//!                 ..Default::default()
//!             }
//!             .into(),
//!         ],
//!         ..Default::default()
//!     }
//! );
//!
//! let rendered = vtt.render();
//!
//! assert_eq!(rendered, text);
//! ```

use std::fmt::Display;
use std::time::Duration;

/// The WebVTT (`.vtt`) format.
///
/// Parses from text by [`WebVtt::parse`](WebVtt::parse)
/// and renders to text by [`WebVtt::render`](WebVtt::render).
///
/// ## Example
/// ```
/// use subtp::vtt::WebVtt;
/// use subtp::vtt::VttCue;
/// use subtp::vtt::VttTimings;
/// use subtp::vtt::VttTimestamp;
///
/// let text = r#"WEBVTT
///
/// 00:00:01.000 --> 00:00:04.000
/// - Never drink liquid nitrogen.
///
/// 00:00:05.000 --> 00:00:09.000
/// - It will perforate your stomach.
/// - You could die.
/// "#;
///
/// let vtt = WebVtt::parse(text).unwrap();
///
/// assert_eq!(
///     vtt,
///     WebVtt {
///         blocks: vec![
///             VttCue {
///                 timings: VttTimings {
///                     start: VttTimestamp {
///                         seconds: 1,
///                         ..Default::default()
///                     },
///                     end: VttTimestamp {
///                         seconds: 4,
///                         ..Default::default()
///                     },
///                 },
///                 payload: vec!["- Never drink liquid nitrogen.".to_string()],
///                 ..Default::default()
///             }
///             .into(),
///             VttCue {
///                 timings: VttTimings {
///                     start: VttTimestamp {
///                         seconds: 5,
///                         ..Default::default()
///                     },
///                     end: VttTimestamp {
///                         seconds: 9,
///                         ..Default::default()
///                     },
///                 },
///                 payload: vec![
///                     "- It will perforate your stomach.".to_string(),
///                     "- You could die.".to_string()
///                 ],
///                 ..Default::default()
///             }
///             .into(),
///         ],
///         ..Default::default()
///     }
/// );
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct WebVtt {
    /// The header of the WebVTT.
    pub header: VttHeader,
    /// The blocks of the WebVTT.
    pub blocks: Vec<VttBlock>,
}

impl WebVtt {
    /// Parses the WebVTT format from the given text.
    ///
    /// ## Example
    /// ```
    /// use subtp::vtt::WebVtt;
    ///
    /// let text = r#"WEBVTT
    ///
    /// 00:00:01.000 --> 00:00:04.000
    /// - Never drink liquid nitrogen.
    ///
    /// 00:00:05.000 --> 00:00:09.000
    /// - It will perforate your stomach.
    /// - You could die.
    /// "#;
    ///
    /// let vtt = WebVtt::parse(text).unwrap();
    /// ```
    pub fn parse(input: &str) -> Result<Self, crate::error::ParseError> {
        crate::vtt_parser::vtt(input).map_err(Into::into)
    }

    /// Renders the text from the WebVTT format.
    ///
    /// ## Example
    /// ```
    /// use subtp::vtt::WebVtt;
    /// use subtp::vtt::VttCue;
    /// use subtp::vtt::VttTimings;
    /// use subtp::vtt::VttTimestamp;
    ///
    /// let vtt = WebVtt {
    ///     blocks: vec![
    ///         VttCue {
    ///             timings: VttTimings {
    ///                 start: VttTimestamp {
    ///                     seconds: 1,
    ///                     ..Default::default()
    ///                 },
    ///                 end: VttTimestamp {
    ///                     seconds: 4,
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             payload: vec!["- Never drink liquid nitrogen.".to_string()],
    ///             ..Default::default()
    ///         }
    ///         .into(),
    ///         VttCue {
    ///             timings: VttTimings {
    ///                 start: VttTimestamp {
    ///                     seconds: 5,
    ///                     ..Default::default()
    ///                 },
    ///                 end: VttTimestamp {
    ///                     seconds: 9,
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             payload: vec![
    ///                 "- It will perforate your stomach.".to_string(),
    ///                 "- You could die.".to_string()
    ///             ],
    ///             ..Default::default()
    ///         }
    ///         .into(),
    ///     ],
    ///    ..Default::default()
    /// };
    ///
    /// let rendered = vtt.render();
    /// ```
    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl Default for WebVtt {
    fn default() -> Self {
        Self {
            header: VttHeader::default(),
            blocks: vec![],
        }
    }
}

impl Display for WebVtt {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}\n", self.header)?;

        let length = self.blocks.len();
        for (i, block) in self.blocks.iter().enumerate() {
            if i + 1 < length {
                write!(f, "{}\n", block)?;
            } else {
                write!(f, "{}", block)?;
            }
        }

        Ok(())
    }
}

impl Iterator for WebVtt {
    type Item = VttBlock;

    fn next(&mut self) -> Option<Self::Item> {
        if self.blocks.is_empty() {
            None
        } else {
            Some(self.blocks.remove(0))
        }
    }
}

/// The header block.
///
/// ## Example
/// ```
/// use subtp::vtt::VttHeader;
/// use subtp::vtt::VttDescription;
///
/// // A header without description.
/// let header = VttHeader {
///     description: None,
/// };
/// assert_eq!(
///     header.to_string(),
///     "WEBVTT\n".to_string()
/// );
///
/// // A header with description from side of "WEBVTT".
/// let header = VttHeader {
///    description: Some(VttDescription::Side("This is a description.".to_string())),
/// };
/// assert_eq!(
///     header.to_string(),
///     "WEBVTT This is a description.\n".to_string()
/// );
///
/// // A header with description from below of "WEBVTT".
/// let header = VttHeader {
///     description: Some(VttDescription::Below("This is a description.".to_string())),
/// };
/// assert_eq!(
///     header.to_string(),
///     "WEBVTT\nThis is a description.\n".to_string()
/// );
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct VttHeader {
    /// The description of this file.
    pub description: Option<VttDescription>,
}

impl Default for VttHeader {
    fn default() -> Self {
        Self {
            description: None,
        }
    }
}

impl Display for VttHeader {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if let Some(description) = &self.description {
            write!(f, "WEBVTT{}\n", description)
        } else {
            write!(f, "WEBVTT\n")
        }
    }
}

/// The description of the WebVTT.
///
/// ## Example
/// ```
/// use subtp::vtt::VttDescription;
///
/// // A description from side.
/// let description = VttDescription::Side("This is a description.".to_string());
/// assert_eq!(
///     description.to_string(),
///     " This is a description.".to_string()
/// );
///
/// // A description from below.
/// let description = VttDescription::Below("This is a description.".to_string());
/// assert_eq!(
///     description.to_string(),
///     "\nThis is a description.".to_string()
/// );
#[derive(Debug, Clone, PartialEq)]
pub enum VttDescription {
    /// From side with "WEBVTT".
    Side(String),
    /// From below with "WEBVTT".
    Below(String),
}

impl Default for VttDescription {
    fn default() -> Self {
        Self::Side(String::new())
    }
}

impl Display for VttDescription {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | Self::Side(description) => {
                write!(f, " {}", description)
            },
            | Self::Below(description) => {
                write!(f, "\n{}", description)
            },
        }
    }
}

/// The block of WebVTT.
///
/// ## Example
///
/// From [`VttCue`](VttCue):
/// ```
/// use subtp::vtt::VttBlock;
/// use subtp::vtt::VttCue;
/// use subtp::vtt::VttTimings;
/// use subtp::vtt::VttTimestamp;
///
/// let block: VttBlock = VttCue {
///     timings: VttTimings {
///         start: VttTimestamp {
///             seconds: 1,
///             ..Default::default()
///         },
///         end: VttTimestamp {
///             seconds: 4,
///             ..Default::default()
///         },
///     },
///     payload: vec!["- Never drink liquid nitrogen.".to_string()],
///     ..Default::default()
/// }.into();
///
/// assert_eq!(
///     block.to_string(),
///     "00:00:01.000 --> 00:00:04.000\n- Never drink liquid nitrogen.\n".to_string()
/// );
/// ```
///
/// From [`VttComment`](VttComment):
/// ```
/// use subtp::vtt::VttBlock;
/// use subtp::vtt::VttComment;
///
/// let block: VttBlock = VttComment::Side("This is a comment.".to_string()).into();
///
/// assert_eq!(
///     block.to_string(),
///     "NOTE This is a comment.\n".to_string()
/// );
/// ```
///
/// From [`VttStyle`](VttStyle):
/// ```
/// use subtp::vtt::VttBlock;
/// use subtp::vtt::VttStyle;
///
/// let block: VttBlock = VttStyle {
///     style: r#"video::cue {
///   background-image: linear-gradient(to bottom, dimgray, lightgray);
///   color: papayawhip;
/// }"#
///     .to_string(),
/// }.into();
///
/// assert_eq!(
///     block.to_string(),
///     "STYLE\nvideo::cue {\n  background-image: linear-gradient(to bottom, dimgray, lightgray);\n  color: papayawhip;\n}\n".to_string()
/// );
/// ```
///
/// From [`VttRegion`](VttRegion):
/// ```
/// use subtp::vtt::VttBlock;
/// use subtp::vtt::VttRegion;
/// use subtp::vtt::Percentage;
///
/// let block: VttBlock = VttRegion {
///     id: Some("region_id".to_string()),
///     width: Some(Percentage {
///         value: 50.0,
///     }),
///     lines: Some(3),
///     ..Default::default()
/// }.into();
///
/// assert_eq!(
///     block.to_string(),
///     "REGION\nid:region_id\nwidth:50%\nlines:3\n".to_string()
/// );
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum VttBlock {
    /// The cue block.
    Que(VttCue),
    /// The comment block.
    Comment(VttComment),
    /// The style block.
    Style(VttStyle),
    /// The region block.
    Region(VttRegion),
}

impl From<VttCue> for VttBlock {
    fn from(value: VttCue) -> Self {
        VttBlock::Que(value)
    }
}

impl From<VttComment> for VttBlock {
    fn from(value: VttComment) -> Self {
        VttBlock::Comment(value)
    }
}

impl From<VttStyle> for VttBlock {
    fn from(value: VttStyle) -> Self {
        VttBlock::Style(value)
    }
}

impl From<VttRegion> for VttBlock {
    fn from(value: VttRegion) -> Self {
        VttBlock::Region(value)
    }
}

impl Display for VttBlock {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | Self::Que(que) => {
                write!(f, "{}", que)
            },
            | Self::Comment(comment) => {
                write!(f, "{}", comment)
            },
            | Self::Style(style) => {
                write!(f, "{}", style)
            },
            | Self::Region(region) => {
                write!(f, "{}", region)
            },
        }
    }
}

/// The region block.
///
/// ## Example
/// ```
/// use subtp::vtt::VttRegion;
/// use subtp::vtt::Percentage;
/// use subtp::vtt::Anchor;
/// use subtp::vtt::Scroll;
///
/// let region = VttRegion {
///     id: Some("region_id".to_string()),
///     width: Some(Percentage {
///         value: 50.0,
///     }),
///     lines: Some(3),
///     region_anchor: Some(Anchor {
///         x: Percentage {
///             value: 50.0,
///         },
///         y: Percentage {
///             value: 50.0,
///         },
///     }),
///     viewport_anchor: Some(Anchor {
///         x: Percentage {
///             value: 50.0,
///         },
///         y: Percentage {
///             value: 50.0,
///         },
///     }),
///     scroll: Some(Scroll::Up),
/// };
///
/// assert_eq!(
///     region.to_string(),
///     "REGION\nid:region_id\nwidth:50%\nlines:3\nregionanchor:50%,50%\nviewportanchor:50%,50%\nscroll:up\n".to_string()
/// );
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct VttRegion {
    /// The identifier.
    pub id: Option<RegionId>,
    /// The width.
    pub width: Option<Percentage>,
    /// The lines.
    pub lines: Option<u32>,
    /// The region anchor.
    pub region_anchor: Option<Anchor>,
    /// The viewport anchor.
    pub viewport_anchor: Option<Anchor>,
    /// The scroll.
    pub scroll: Option<Scroll>,
}

/// The region identifier.
pub type RegionId = String;

impl Default for VttRegion {
    fn default() -> Self {
        Self {
            id: None,
            width: None,
            lines: None,
            region_anchor: None,
            viewport_anchor: None,
            scroll: None,
        }
    }
}

impl Display for VttRegion {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "REGION\n")?;

        if let Some(id) = &self.id {
            write!(f, "id:{}\n", id)?;
        }

        if let Some(width) = self.width {
            write!(f, "width:{}\n", width)?;
        }

        if let Some(lines) = self.lines {
            write!(f, "lines:{}\n", lines)?;
        }

        if let Some(region_anchor) = self.region_anchor {
            write!(f, "regionanchor:{}\n", region_anchor)?;
        }

        if let Some(viewport_anchor) = self.viewport_anchor {
            write!(
                f,
                "viewportanchor:{}\n",
                viewport_anchor
            )?;
        }

        if let Some(scroll) = self.scroll {
            write!(f, "scroll:{}\n", scroll)?;
        }

        Ok(())
    }
}

/// The comment block.
///
/// ## Example
/// ```
/// use subtp::vtt::VttComment;
///
/// let comment = VttComment::Side("This is a comment.".to_string());
/// assert_eq!(
///     comment.to_string(),
///     "NOTE This is a comment.\n".to_string()
/// );
///
/// let comment = VttComment::Below("This is a comment.".to_string());
/// assert_eq!(
///     comment.to_string(),
///     "NOTE\nThis is a comment.\n".to_string()
/// );
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum VttComment {
    /// Side with "NOTE".
    Side(String),
    /// Below with "NOTE".
    Below(String),
}

impl Default for VttComment {
    fn default() -> Self {
        Self::Side(String::new())
    }
}

impl Display for VttComment {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | Self::Side(comment) => {
                write!(f, "NOTE {}\n", comment)
            },
            | Self::Below(comment) => {
                write!(f, "NOTE\n{}\n", comment)
            },
        }
    }
}

/// The style block.
///
/// ## Example
/// ```
/// use subtp::vtt::VttStyle;
///
/// let style = VttStyle {
///    style: r#"video::cue {
/// background-image: linear-gradient(to bottom, dimgray, lightgray);
/// color: papayawhip;
/// }"#
///     .to_string(),
/// };
///
/// assert_eq!(
///     style.to_string(),
///     "STYLE\nvideo::cue {\nbackground-image: linear-gradient(to bottom, dimgray, lightgray);\ncolor: papayawhip;\n}\n".to_string()
/// );
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct VttStyle {
    pub style: String,
}

impl Default for VttStyle {
    fn default() -> Self {
        Self {
            style: String::new(),
        }
    }
}

impl Display for VttStyle {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "STYLE\n{}\n", self.style)
    }
}

/// The cue block.
///
/// ## Example
/// ```
/// use subtp::vtt::VttBlock;
/// use subtp::vtt::VttCue;
/// use subtp::vtt::VttTimings;
/// use subtp::vtt::VttTimestamp;
///
/// // A simple cue block.
/// let cue = VttCue {
///     timings: VttTimings {
///         start: VttTimestamp {
///             seconds: 1,
///             ..Default::default()
///         },
///         end: VttTimestamp {
///             seconds: 4,
///             ..Default::default()
///         },
///     },
///     payload: vec!["- Never drink liquid nitrogen.".to_string()],
///     ..Default::default()
/// };
///
/// assert_eq!(
///     cue.to_string(),
///     "00:00:01.000 --> 00:00:04.000\n- Never drink liquid nitrogen.\n".to_string()
/// );
///
/// use subtp::vtt::CueSettings;
/// use subtp::vtt::Vertical;
/// use subtp::vtt::Line;
/// use subtp::vtt::LineAlignment;
/// use subtp::vtt::Position;
/// use subtp::vtt::Percentage;
/// use subtp::vtt::PositionAlignment;
/// use subtp::vtt::Alignment;
///
/// // A cue block with identifier and settings.
/// let cue = VttCue {
///     identifier: Some("cue_id".to_string()),
///     timings: VttTimings {
///         start: VttTimestamp {
///             seconds: 3,
///             ..Default::default()
///         },
///         end: VttTimestamp {
///             seconds: 4,
///             ..Default::default()
///         },
///     },
///     settings: Some(CueSettings {
///         vertical: Some(Vertical::Lr),
///         line: Some(Line::LineNumber(3, Some(LineAlignment::Center))),
///         position: Some(Position {
///             value: Percentage {
///                 value: 50.0,
///             },
///             alignment: Some(PositionAlignment::Center),
///         }),
///         size: Some(Percentage {
///             value: 100.0,
///         }),
///         align: Some(Alignment::End),
///         region: Some("region_id".to_string()),
///     }),
///     payload: vec!["- It will perforate your stomach.".to_string()],
/// };
///
/// assert_eq!(
///     cue.to_string(),
///     "cue_id\n00:00:03.000 --> 00:00:04.000 vertical:lr line:3,center position:50%,center size:100% align:end region:region_id\n- It will perforate your stomach.\n".to_string()
/// );
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct VttCue {
    /// The identifier.
    pub identifier: Option<String>,
    /// The timings.
    pub timings: VttTimings,
    /// The settings.
    pub settings: Option<CueSettings>,
    /// The payload of subtitle text.
    pub payload: Vec<String>,
}

impl Default for VttCue {
    fn default() -> Self {
        Self {
            identifier: None,
            timings: VttTimings::default(),
            settings: None,
            payload: vec![],
        }
    }
}

impl Display for VttCue {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if let Some(identifier) = &self.identifier {
            write!(f, "{}\n", identifier)?;
        }

        write!(f, "{}", self.timings)?;

        if let Some(settings) = &self.settings {
            write!(f, " {}", settings)?;
        }

        write!(f, "\n{}\n", self.payload.join("\n"))
    }
}

/// The timings.
///
/// ## Example
/// ```
/// use subtp::vtt::VttTimings;
/// use subtp::vtt::VttTimestamp;
///
/// let timings = VttTimings {
///     start: VttTimestamp {
///         seconds: 1,
///         ..Default::default()
///     },
///     end: VttTimestamp {
///         seconds: 4,
///         ..Default::default()
///     },
/// };
///
/// assert_eq!(
///     timings.to_string(),
///     "00:00:01.000 --> 00:00:04.000".to_string()
/// );
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct VttTimings {
    /// The start timestamp.
    pub start: VttTimestamp,
    /// The end timestamp.
    pub end: VttTimestamp,
}

impl Default for VttTimings {
    fn default() -> Self {
        Self {
            start: VttTimestamp::default(),
            end: VttTimestamp::default(),
        }
    }
}

impl Display for VttTimings {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{} --> {}", self.start, self.end)
    }
}

/// The timestamp.
///
/// ## Example
/// ```
/// use subtp::vtt::VttTimestamp;
///
/// let timestamp = VttTimestamp {
///     hours: 1,
///     minutes: 2,
///     seconds: 3,
///     milliseconds: 4,
/// };
///
/// assert_eq!(
///     timestamp.to_string(),
///     "01:02:03.004".to_string()
/// );
///
/// // With default values.
/// let timestamp = VttTimestamp {
///     seconds: 1,
///     ..Default::default()
/// };
///
/// assert_eq!(
///     timestamp.to_string(),
///     "00:00:01.000".to_string()
/// );
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct VttTimestamp {
    /// The hours.
    pub hours: u32,
    /// The minutes.
    pub minutes: u8,
    /// The seconds.
    pub seconds: u8,
    /// The milliseconds.
    pub milliseconds: u16,
}

impl Default for VttTimestamp {
    fn default() -> Self {
        Self {
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: 0,
        }
    }
}

impl Display for VttTimestamp {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}.{:03}",
            self.hours, self.minutes, self.seconds, self.milliseconds
        )
    }
}

impl From<Duration> for VttTimestamp {
    fn from(duration: Duration) -> Self {
        let seconds = duration.as_secs();
        let milliseconds = duration.subsec_millis() as u16;

        let hours = (seconds / 3600) as u32;
        let minutes = ((seconds % 3600) / 60) as u8;
        let seconds = (seconds % 60) as u8;

        Self {
            hours,
            minutes,
            seconds,
            milliseconds,
        }
    }
}

impl Into<Duration> for VttTimestamp {
    fn into(self) -> Duration {
        Duration::new(
            self.hours as u64 * 3600
                + self.minutes as u64 * 60
                + self.seconds as u64,
            self.milliseconds as u32 * 1_000_000,
        )
    }
}

/// The settings of cue.
///
/// ## Example
/// ```
/// use subtp::vtt::CueSettings;
/// use subtp::vtt::Vertical;
/// use subtp::vtt::Line;
/// use subtp::vtt::LineAlignment;
/// use subtp::vtt::Position;
/// use subtp::vtt::Percentage;
/// use subtp::vtt::PositionAlignment;
/// use subtp::vtt::Alignment;
///
/// let settings = CueSettings {
///     vertical: Some(Vertical::Lr),
///     line: Some(Line::LineNumber(3, Some(LineAlignment::Center))),
///     position: Some(Position {
///         value: Percentage {
///             value: 50.0,
///         },
///         alignment: Some(PositionAlignment::Center),
///         }),
///     size: Some(Percentage {
///         value: 100.0,
///     }),
///     align: Some(Alignment::End),
///     region: Some("region_id".to_string()),
/// };
///
/// assert_eq!(
///     settings.to_string(),
///     "vertical:lr line:3,center position:50%,center size:100% align:end region:region_id".to_string()
/// );
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct CueSettings {
    /// The vertical setting.
    pub vertical: Option<Vertical>,
    /// The line setting.
    pub line: Option<Line>,
    /// The position setting.
    pub position: Option<Position>,
    /// The size setting.
    pub size: Option<Percentage>,
    /// The alignment setting.
    pub align: Option<Alignment>,
    /// The region setting.
    pub region: Option<RegionId>,
}

impl Default for CueSettings {
    fn default() -> Self {
        Self {
            vertical: None,
            line: None,
            position: None,
            size: None,
            align: None,
            region: None,
        }
    }
}

impl Display for CueSettings {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let mut settings = Vec::new();

        if let Some(vertical) = self.vertical {
            settings.push(format!("vertical:{}", vertical));
        }

        if let Some(line) = self.line {
            settings.push(format!("line:{}", line));
        }

        if let Some(position) = self.position {
            settings.push(format!("position:{}", position));
        }

        if let Some(size) = self.size {
            settings.push(format!("size:{}", size));
        }

        if let Some(align) = self.align {
            settings.push(format!("align:{}", align));
        }

        if let Some(region) = &self.region {
            settings.push(format!("region:{}", region));
        }

        write!(f, "{}", settings.join(" "))
    }
}

/// The percentage in range 0.0 to 100.0, inclusive.
///
/// ## Example
/// ```
/// use subtp::vtt::Percentage;
///
/// let percentage = Percentage {
///     value: 50.0,
/// };
///
/// assert_eq!(
///     percentage.to_string(),
///     "50%".to_string()
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percentage {
    pub value: f32,
}

impl Default for Percentage {
    fn default() -> Self {
        Self {
            value: 0.0,
        }
    }
}

impl Display for Percentage {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if self.value.fract() == 0.0 {
            write!(f, "{}%", self.value as i32)
        } else {
            write!(f, "{}%", self.value)
        }
    }
}

/// The anchor by percentages.
///
/// ## Example
/// ```
/// use subtp::vtt::Anchor;
/// use subtp::vtt::Percentage;
///
/// let anchor = Anchor {
///     x: Percentage {
///         value: 0.0,
///     },
///     y: Percentage {
///         value: 100.0,
///     },
/// };
///
/// assert_eq!(
///     anchor.to_string(),
///     "0%,100%".to_string()
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Anchor {
    /// The horizontal setting.
    pub x: Percentage,
    /// The vertical setting.
    pub y: Percentage,
}

impl Default for Anchor {
    fn default() -> Self {
        Self {
            x: Percentage {
                value: 0.0,
            },
            y: Percentage {
                value: 100.0,
            },
        }
    }
}

impl Display for Anchor {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

/// The scroll setting of region.
///
/// ## Example
/// ```
/// use subtp::vtt::Scroll;
///
/// let scroll = Scroll::Up;
/// assert_eq!(
///     scroll.to_string(),
///     "up".to_string()
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Scroll {
    /// The scroll up.
    Up,
}

impl Default for Scroll {
    fn default() -> Self {
        Self::Up
    }
}

impl Display for Scroll {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | Self::Up => {
                write!(f, "up")
            },
        }
    }
}

/// The vertical setting of cue.
///
/// ## Example
/// ```
/// use subtp::vtt::Vertical;
///
/// let vertical = Vertical::Lr;
///
/// assert_eq!(
///     vertical.to_string(),
///     "lr".to_string()
/// );
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Vertical {
    /// From right to left.
    Rl,
    /// From left to right.
    Lr,
}

impl Default for Vertical {
    fn default() -> Self {
        Self::Rl
    }
}

impl Display for Vertical {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | Self::Rl => {
                write!(f, "rl")
            },
            | Self::Lr => {
                write!(f, "lr")
            },
        }
    }
}

/// The line setting of cue.
///
/// ## Example
/// ```
/// use subtp::vtt::Line;
/// use subtp::vtt::LineAlignment;
///
/// // Without alignment.
/// let line = Line::LineNumber(3, None);
///
/// assert_eq!(
///     line.to_string(),
///     "3".to_string()
/// );
///
/// // With alignment.
/// let line = Line::LineNumber(3, Some(LineAlignment::Center));
///
/// assert_eq!(
///     line.to_string(),
///     "3,center".to_string()
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Line {
    /// The percentage.
    Percentage(Percentage, Option<LineAlignment>),
    /// The line number.
    LineNumber(i32, Option<LineAlignment>),
}

impl Default for Line {
    fn default() -> Self {
        Self::Percentage(Percentage::default(), None)
    }
}

impl Display for Line {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | Self::Percentage(percentage, align) => {
                if let Some(align) = align {
                    write!(f, "{},{}", percentage, align)
                } else {
                    write!(f, "{}", percentage)
                }
            },
            | Self::LineNumber(line_number, align) => {
                if let Some(align) = align {
                    write!(f, "{},{}", line_number, align)
                } else {
                    write!(f, "{}", line_number)
                }
            },
        }
    }
}

/// The alignment setting of line.
///
/// ## Example
/// ```
/// use subtp::vtt::LineAlignment;
///
/// let alignment = LineAlignment::Center;
///
/// assert_eq!(
///     alignment.to_string(),
///     "center".to_string()
/// );
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum LineAlignment {
    /// The start alignment.
    Start,
    /// The center alignment.
    Center,
    /// The end alignment.
    End,
}

impl Default for LineAlignment {
    fn default() -> Self {
        Self::Start
    }
}

impl Display for LineAlignment {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | Self::Start => {
                write!(f, "start")
            },
            | Self::Center => {
                write!(f, "center")
            },
            | Self::End => {
                write!(f, "end")
            },
        }
    }
}

/// The position setting of cue.
///
/// ## Example
/// ```
/// use subtp::vtt::Position;
/// use subtp::vtt::Percentage;
/// use subtp::vtt::PositionAlignment;
///
/// // Without alignment.
/// let position = Position {
///     value: Percentage {
///         value: 50.0,
///     },
///     alignment: None,
/// };
///
/// assert_eq!(
///     position.to_string(),
///     "50%".to_string()
/// );
///
/// // With alignment.
/// let position = Position {
///     value: Percentage {
///         value: 50.0,
///     },
///     alignment: Some(PositionAlignment::Center),
/// };
///
/// assert_eq!(
///     position.to_string(),
///     "50%,center".to_string()
/// );
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    /// The position value.
    pub value: Percentage,
    /// The alignment setting.
    pub alignment: Option<PositionAlignment>,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            value: Percentage::default(),
            alignment: None,
        }
    }
}

impl Display for Position {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if let Some(alignment) = self.alignment {
            write!(f, "{},{}", self.value, alignment)
        } else {
            write!(f, "{}", self.value)
        }
    }
}

/// The alignment setting of position.
///
/// ## Example
/// ```
/// use subtp::vtt::PositionAlignment;
///
/// let alignment = PositionAlignment::Center;
///
/// assert_eq!(
///     alignment.to_string(),
///     "center".to_string()
/// );
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PositionAlignment {
    /// The line left alignment.
    LineLeft,
    /// The line center alignment.
    Center,
    /// The line right alignment.
    LineRight,
}

impl Default for PositionAlignment {
    fn default() -> Self {
        Self::LineLeft
    }
}

impl Display for PositionAlignment {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | Self::LineLeft => {
                write!(f, "line-left")
            },
            | Self::Center => {
                write!(f, "center")
            },
            | Self::LineRight => {
                write!(f, "line-right")
            },
        }
    }
}

/// The alignment setting.
///
/// ## Example
/// ```
/// use subtp::vtt::Alignment;
///
/// let alignment = Alignment::End;
///
/// assert_eq!(
///     alignment.to_string(),
///     "end".to_string()
/// );
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Alignment {
    /// The start alignment.
    Start,
    /// The center alignment.
    Center,
    /// The end alignment.
    End,
    /// The left alignment.
    Left,
    /// The right alignment.
    Right,
}

impl Default for Alignment {
    fn default() -> Self {
        Self::Start
    }
}

impl Display for Alignment {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | Self::Start => {
                write!(f, "start")
            },
            | Self::Center => {
                write!(f, "center")
            },
            | Self::End => {
                write!(f, "end")
            },
            | Self::Left => {
                write!(f, "left")
            },
            | Self::Right => {
                write!(f, "right")
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let text = r#"WEBVTT

00:01.000 --> 00:04.000
- Never drink liquid nitrogen.

00:05.000 --> 00:09.000
- It will perforate your stomach.
- You could die.
"#;

        let expected = WebVtt {
            blocks: vec![
                VttCue {
                    timings: VttTimings {
                        start: VttTimestamp {
                            seconds: 1,
                            ..Default::default()
                        },
                        end: VttTimestamp {
                            seconds: 4,
                            ..Default::default()
                        },
                    },
                    payload: vec!["- Never drink liquid nitrogen.".to_string()],
                    ..Default::default()
                }
                .into(),
                VttCue {
                    timings: VttTimings {
                        start: VttTimestamp {
                            seconds: 5,
                            ..Default::default()
                        },
                        end: VttTimestamp {
                            seconds: 9,
                            ..Default::default()
                        },
                    },
                    payload: vec![
                        "- It will perforate your stomach.".to_string(),
                        "- You could die.".to_string(),
                    ],
                    ..Default::default()
                }
                .into(),
            ],
            ..Default::default()
        };

        assert_eq!(WebVtt::parse(text).unwrap(), expected);
    }

    #[test]
    fn render() {
        let vtt = WebVtt {
            blocks: vec![
                VttCue {
                    timings: VttTimings {
                        start: VttTimestamp {
                            seconds: 1,
                            ..Default::default()
                        },
                        end: VttTimestamp {
                            seconds: 4,
                            ..Default::default()
                        },
                    },
                    payload: vec!["- Never drink liquid nitrogen.".to_string()],
                    ..Default::default()
                }
                .into(),
                VttCue {
                    timings: VttTimings {
                        start: VttTimestamp {
                            seconds: 5,
                            ..Default::default()
                        },
                        end: VttTimestamp {
                            seconds: 9,
                            ..Default::default()
                        },
                    },
                    payload: vec![
                        "- It will perforate your stomach.".to_string(),
                        "- You could die.".to_string(),
                    ],
                    ..Default::default()
                }
                .into(),
            ],
            ..Default::default()
        };

        let expected = r#"WEBVTT

00:00:01.000 --> 00:00:04.000
- Never drink liquid nitrogen.

00:00:05.000 --> 00:00:09.000
- It will perforate your stomach.
- You could die.
"#;

        assert_eq!(vtt.render(), expected);

        let vtt = WebVtt {
            header: VttHeader {
                description: Some(VttDescription::Side(
                    "This is a description.".to_string(),
                )),
            },
            blocks: vec![
                VttComment::Side("This is a comment.".to_string()).into(),
                VttRegion {
                    id: Some("region_id".to_string()),
                    width: Some(Percentage {
                        value: 50.0,
                    }),
                    lines: Some(3),
                    region_anchor: Some(Anchor {
                        x: Percentage {
                            value: 50.0,
                        },
                        y: Percentage {
                            value: 50.0,
                        },
                    }),
                    viewport_anchor: Some(Anchor {
                        x: Percentage {
                            value: 50.0,
                        },
                        y: Percentage {
                            value: 50.0,
                        },
                    }),
                    scroll: Some(Scroll::Up),
                }
                .into(),
                VttStyle {
                    style: r#"video::cue {
  background-image: linear-gradient(to bottom, dimgray, lightgray);
  color: papayawhip;
}"#
                    .to_string(),
                }
                .into(),
                VttCue {
                    identifier: Some("1".to_string()),
                    timings: VttTimings {
                        start: VttTimestamp {
                            hours: 1,
                            minutes: 2,
                            seconds: 3,
                            milliseconds: 4,
                        },
                        end: VttTimestamp {
                            hours: 1,
                            minutes: 2,
                            seconds: 5,
                            milliseconds: 6,
                        },
                    },
                    settings: Some(CueSettings {
                        vertical: Some(Vertical::Lr),
                        line: Some(Line::Percentage(
                            Percentage {
                                value: 100.0,
                            },
                            Some(LineAlignment::Center),
                        )),
                        position: Some(Position {
                            value: Percentage {
                                value: 50.0,
                            },
                            alignment: Some(PositionAlignment::Center),
                        }),
                        size: Some(Percentage {
                            value: 50.0,
                        }),
                        align: Some(Alignment::Center),
                        region: Some("region_id".to_string()),
                    }),
                    payload: vec!["- Never drink liquid nitrogen.".to_string()],
                }
                .into(),
            ],
        };

        let expected = r#"WEBVTT This is a description.

NOTE This is a comment.

REGION
id:region_id
width:50%
lines:3
regionanchor:50%,50%
viewportanchor:50%,50%
scroll:up

STYLE
video::cue {
  background-image: linear-gradient(to bottom, dimgray, lightgray);
  color: papayawhip;
}

1
01:02:03.004 --> 01:02:05.006 vertical:lr line:100%,center position:50%,center size:50% align:center region:region_id
- Never drink liquid nitrogen.
"#;

        assert_eq!(vtt.render(), expected);
    }

    #[test]
    fn iterator() {
        let vtt = WebVtt {
            blocks: vec![
                VttCue {
                    timings: VttTimings {
                        start: VttTimestamp {
                            seconds: 1,
                            ..Default::default()
                        },
                        end: VttTimestamp {
                            seconds: 4,
                            ..Default::default()
                        },
                    },
                    payload: vec!["- Never drink liquid nitrogen.".to_string()],
                    ..Default::default()
                }
                .into(),
                VttCue {
                    timings: VttTimings {
                        start: VttTimestamp {
                            seconds: 5,
                            ..Default::default()
                        },
                        end: VttTimestamp {
                            seconds: 9,
                            ..Default::default()
                        },
                    },
                    payload: vec![
                        "- It will perforate your stomach.".to_string(),
                        "- You could die.".to_string(),
                    ],
                    ..Default::default()
                }
                .into(),
            ],
            ..Default::default()
        };

        let mut iter = vtt.into_iter();

        assert_eq!(iter.next(), Some(VttCue {
            timings: VttTimings {
                start: VttTimestamp {
                    seconds: 1,
                    ..Default::default()
                },
                end: VttTimestamp {
                    seconds: 4,
                    ..Default::default()
                },
            },
            payload: vec![
                "- Never drink liquid nitrogen.".to_string(),
            ],
            ..Default::default()
        }.into()));

        assert_eq!(
            iter.next(),
            Some(
                VttCue {
                    timings: VttTimings {
                        start: VttTimestamp {
                            seconds: 5,
                            ..Default::default()
                        },
                        end: VttTimestamp {
                            seconds: 9,
                            ..Default::default()
                        },
                    },
                    payload: vec![
                        "- It will perforate your stomach.".to_string(),
                        "- You could die.".to_string(),
                    ],
                    ..Default::default()
                }
                .into()
            )
        );

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn display_header() {
        let header = VttHeader {
            description: Some(VttDescription::Side(
                "This is a description.".to_string(),
            )),
        };

        let expected = "WEBVTT This is a description.\n";

        assert_eq!(header.to_string(), expected);

        let header = VttHeader {
            description: Some(VttDescription::Below(
                "This is a description.".to_string(),
            )),
        };

        let expected = "WEBVTT\nThis is a description.\n";

        assert_eq!(header.to_string(), expected);

        let header = VttHeader {
            description: None,
        };

        let expected = "WEBVTT\n";

        assert_eq!(header.to_string(), expected);
    }

    #[test]
    fn display_cue() {
        let cue = VttCue {
            identifier: Some("1".to_string()),
            timings: VttTimings {
                start: VttTimestamp {
                    hours: 0,
                    minutes: 0,
                    seconds: 1,
                    milliseconds: 0,
                },
                end: VttTimestamp {
                    hours: 0,
                    minutes: 0,
                    seconds: 4,
                    milliseconds: 0,
                },
            },
            settings: Some(CueSettings {
                vertical: Some(Vertical::Lr),
                line: Some(Line::Percentage(
                    Percentage {
                        value: 100.0,
                    },
                    Some(LineAlignment::Center),
                )),
                position: Some(Position {
                    value: Percentage {
                        value: 50.0,
                    },
                    alignment: Some(PositionAlignment::Center),
                }),
                size: Some(Percentage {
                    value: 50.0,
                }),
                align: Some(Alignment::Center),
                region: Some("region".to_string()),
            }),
            payload: vec!["- Never drink liquid nitrogen.".to_string()],
        };

        let expected = "1\n00:00:01.000 --> 00:00:04.000 vertical:lr line:100%,center position:50%,center size:50% align:center region:region\n- Never drink liquid nitrogen.\n";

        assert_eq!(cue.to_string(), expected);

        let cue = VttCue {
            identifier: None,
            timings: VttTimings {
                start: VttTimestamp {
                    hours: 0,
                    minutes: 0,
                    seconds: 1,
                    milliseconds: 0,
                },
                end: VttTimestamp {
                    hours: 0,
                    minutes: 0,
                    seconds: 4,
                    milliseconds: 0,
                },
            },
            settings: None,
            payload: vec!["- Never drink liquid nitrogen.".to_string()],
        };

        let expected =
            "00:00:01.000 --> 00:00:04.000\n- Never drink liquid nitrogen.\n";

        assert_eq!(cue.to_string(), expected);
    }

    #[test]
    fn display_comment() {
        let comment = VttComment::Side("This is a comment.".to_string());
        let expected = "NOTE This is a comment.\n";
        assert_eq!(comment.to_string(), expected);

        let comment = VttComment::Below("This is a comment.".to_string());
        let expected = "NOTE\nThis is a comment.\n";
        assert_eq!(comment.to_string(), expected);

        let comment =
            VttComment::Side("This is a comment.\nacross line.".to_string());
        let expected = "NOTE This is a comment.\nacross line.\n";
        assert_eq!(comment.to_string(), expected);
    }

    #[test]
    fn display_style() {
        let style = VttStyle {
            style: "This is a style.".to_string(),
        };
        let expected = "STYLE\nThis is a style.\n";
        assert_eq!(style.to_string(), expected);
    }

    #[test]
    fn display_region() {
        let region = VttRegion {
            id: Some("region".to_string()),
            width: Some(Percentage {
                value: 50.0,
            }),
            lines: Some(3),
            region_anchor: Some(Anchor {
                x: Percentage {
                    value: 50.0,
                },
                y: Percentage {
                    value: 50.0,
                },
            }),
            viewport_anchor: Some(Anchor {
                x: Percentage {
                    value: 50.0,
                },
                y: Percentage {
                    value: 50.0,
                },
            }),
            scroll: Some(Scroll::Up),
        };
        let expected = "REGION\nid:region\nwidth:50%\nlines:3\nregionanchor:50%,50%\nviewportanchor:50%,50%\nscroll:up\n";
        assert_eq!(region.to_string(), expected);

        let region = VttRegion {
            id: Some("region".to_string()),
            width: Some(Percentage {
                value: 50.0,
            }),
            lines: None,
            region_anchor: None,
            viewport_anchor: None,
            scroll: None,
        };
        let expected = "REGION\nid:region\nwidth:50%\n";
        assert_eq!(region.to_string(), expected);
    }

    #[test]
    fn from_duration_to_timestamp() {
        let duration = Duration::new(1, 0);
        let timestamp: VttTimestamp = duration.into();
        assert_eq!(
            timestamp,
            VttTimestamp {
                seconds: 1,
                ..Default::default()
            }
        );

        let duration = Duration::new(1, 500_000_000);
        let timestamp: VttTimestamp = duration.into();
        assert_eq!(
            timestamp,
            VttTimestamp {
                seconds: 1,
                milliseconds: 500,
                ..Default::default()
            }
        );
    }

    #[test]
    fn from_timestamp_to_duration() {
        let timestamp = VttTimestamp {
            seconds: 1,
            ..Default::default()
        };
        let duration: Duration = timestamp.into();
        assert_eq!(duration, Duration::new(1, 0));

        let timestamp = VttTimestamp {
            seconds: 1,
            milliseconds: 500,
            ..Default::default()
        };
        let duration: Duration = timestamp.into();
        assert_eq!(duration, Duration::new(1, 500_000_000));
    }

    #[test]
    fn operate_timestamp_via_duration() {
        let start: Duration = VttTimestamp {
            seconds: 1,
            ..Default::default()
        }
        .into();

        let end: Duration = VttTimestamp {
            seconds: 4,
            ..Default::default()
        }
        .into();

        let duration = end - start;
        assert_eq!(duration, Duration::new(3, 0));

        let timestamp: VttTimestamp = (start + duration).into();
        assert_eq!(
            timestamp,
            VttTimestamp {
                seconds: 4,
                ..Default::default()
            }
        );
    }

    #[test]
    fn order_timestamp() {
        let start = VttTimestamp {
            seconds: 1,
            ..Default::default()
        };

        let end = VttTimestamp {
            seconds: 4,
            ..Default::default()
        };

        assert!(start < end);
    }
}
