mod root;
mod archive;
mod blog;
mod cv;
mod math_font;
mod tag;

mod sitemap;
mod rss_feed;
mod stats;
mod not_found;

pub use archive::handle_archive;
pub use blog::{handle_blog, redirect_legacy_blog};
pub use cv::handle_cv;
pub use math_font::handle_math_font;
pub use not_found::handle_404;
pub use root::root;
pub use rss_feed::handle_rss;
pub use tag::handle_tag;
pub use sitemap::handle_sitemap;
pub use stats::handle_stats;
