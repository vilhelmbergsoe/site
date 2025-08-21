mod root;
mod blog;
mod tag;

mod sitemap;
mod rss_feed;
mod not_found;

pub use blog::handle_blog;
pub use not_found::handle_404;
pub use root::root;
pub use rss_feed::handle_rss;
pub use tag::handle_tag;
pub use sitemap::handle_sitemap;
