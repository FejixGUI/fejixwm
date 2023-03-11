use crate::*;

pub trait ClipboardController : ShellClientTrait {

    fn allows_clipboard_usage(&self, usage: ClipboardUsage) -> bool;

    fn offer_clipboard(&self, offers: &[ClipboardOffer]) -> Result<()>;
    fn request_clipboard(&self) -> Result<()>;

    // fn transmit_clipboard_data(&self, ???);
    // fn receive_clipboard_data(&self, ???);

}


pub struct ClipboardDescription<'a> {

    /// MIME type
    pub format: &'a str,
    
    /// Specifies what the provided data is intended for: actual pasting, previewing the content etc.
    pub usage: ClipboardUsage,

}

pub struct ClipboardOffer<'a> {

    pub description: ClipboardDescription<'a>,

    /// When true, allows FejixWM to:
    /// * convert the data to the platform's native format if such exists;
    /// * publish the data with platform's native formats synonymous to the one provided;
    /// * use conventional special transfer methods for the chosen format is such exists.
    pub allow_convert: bool,

}


#[derive(Clone, Copy)]
pub enum ClipboardUsage {
    /// Pastable data
    Content,
    
    /// Text description of the pastable data
    Description,

    /// Small graphical representation of the pastable data
    Thumbnail,

    /// Printing preview image of the pastable data
    PrintedPreview,

    /// Iconic representation of the pastable data
    Icon,

    /// Pastable data but uses the X11's primary selection mechanism
    PrimarySelection,

    /// Pastable data but uses the X11's secondary selection mechanism
    SecondarySelection,
}


pub struct ClipboardOfferEvent<'a> {

    pub offers: &'a [ClipboardDescription<'a>]

}

// struct ClipboardTrasmitEvent, ClipboardReceiveEvent