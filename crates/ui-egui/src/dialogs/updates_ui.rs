//! Inanna updates are downloaded explicitly from its own release page.
use crate::SlideApp;
pub(crate) fn request_check(app: &mut SlideApp) {
    app.open_url("https://github.com/tomblanchard312/Inanna/releases");
}
