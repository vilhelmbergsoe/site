mod assets;
mod blog_index;
mod blog_post;
mod not_found;
mod root;
mod rss_feed;
mod sitemap;
mod tag;

pub use assets::{handle_asset, handle_cv, handle_robots};
pub use blog_index::handle_blog_index;
pub use blog_post::handle_blog_post;
pub use not_found::handle_404;
pub use root::root;
pub use rss_feed::handle_rss;
pub use sitemap::handle_sitemap;
pub use tag::handle_tag;
