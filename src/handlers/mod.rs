mod archive;
mod assets;
mod blog;
mod not_found;
mod root;
mod rss_feed;
mod sitemap;
mod tag;

pub use archive::handle_archive;
pub use assets::{handle_asset, handle_cv, handle_robots};
pub use blog::{handle_blog, redirect_legacy_blog};
pub use not_found::handle_404;
pub use root::root;
pub use rss_feed::handle_rss;
pub use sitemap::handle_sitemap;
pub use tag::handle_tag;
