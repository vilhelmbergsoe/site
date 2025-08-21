mod blog;
mod not_found;
mod root;
mod tag;
mod rss_feed;

pub use blog::handle_blog;
pub use not_found::handle_404;
pub use root::root;
pub use rss_feed::handle_rss;
pub use tag::handle_tag;
