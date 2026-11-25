//! WaOS backend for opening links.
//!
//! WaOS opens a link with the app the user's system has for it, through
//! `wasdk`'s opener. Only `Browser::Default` is supported.

use crate::{Browser, BrowserOptions, Error, ErrorKind, Result, TargetType};
use wasdk::opener::{self, OpenError};

pub(super) fn open_browser_internal(
    browser: Browser,
    target: &TargetType,
    options: &BrowserOptions,
) -> Result<()> {
    if browser != Browser::Default {
        return Err(Error::new(
            ErrorKind::NotFound,
            "only default browser supported",
        ));
    }

    let opener = opener::get_opener()
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "no opener available"))?;

    if options.dry_run {
        return Ok(());
    }

    let url: &str = target;
    wasdk::block_on(opener.open_link(url.to_owned())).map_err(map_open_error)
}

fn map_open_error(error: OpenError) -> Error {
    match error {
        OpenError::Denied => Error::new(ErrorKind::PermissionDenied, "opening link denied"),
        OpenError::NoApp => Error::new(ErrorKind::NotFound, "no app to open link"),
        OpenError::Failed(message) => Error::other(message),
    }
}
