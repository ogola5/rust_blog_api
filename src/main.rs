use warp::Filter;
use warp::http::StatusCode;
use serde::{Serialize, Deserialize};
use std::sync::{Arc,Mutex};
use std:: collections::VecDeque;



#[derive(Serialize, Deserialize, Debug,Clone)] // Ensure Serialize, Deserialize, and Debug are derived
struct BlogPost {
    title: String,
    content: String,
}

#[tokio::main]
async fn main() {
    //Initialize an in-memory store for blog posts
    let blog_post = Arc::new(Mutex::new(VecDeque::new()));
    
    
    //Clone the Arc to move a refence into the filters
    let blog_posts_filter = warp::any().map(move || blog_post.clone());
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type"])
        .allow_methods(vec!["GET","POST","PUT","DELETE"]);
    let create_post  = warp::path("blog")
        .and(warp::post())
        .and(warp::body::json())
        .and(blog_posts_filter.clone())
        .map(|new_post: BlogPost,blog_posts:   Arc<Mutex<VecDeque<BlogPost>>>| {
            let mut posts = blog_posts.lock().unwrap();
            posts.push_back(new_post.clone());
            println!("New blog post: {:?}", new_post);
            StatusCode::CREATED
        });
    let get_posts = warp::path("blog")
        .and(warp::get())
        .and(blog_posts_filter.clone())
        .map(|blog_posts:Arc<Mutex<VecDeque<BlogPost>>>|{
            let posts = blog_posts.lock().unwrap();
            warp::reply::json(&*posts)
        });
// PUT endpoint to update a blog post
let update_post = warp::path("blog")
.and(warp::put())
.and(warp::body::json())
.and(blog_posts_filter.clone())
.map(|updated_post: BlogPost, blog_posts: Arc<Mutex<VecDeque<BlogPost>>>| {
    let mut posts = blog_posts.lock().unwrap();
    let mut response = StatusCode::NOT_FOUND;
    for post in posts.iter_mut() {
        if post.title == updated_post.title {
            post.content = updated_post.content.clone();
            response = StatusCode::OK;
            println!("Updated blog post: {:?}", post);
            break;
        }
    }
    response // Return response
});

    // DELETE endpoint to delete a blog post
    let delete_post = warp::path("blog")
        .and(warp::delete())
        .and(warp::body::json())
        .and(blog_posts_filter.clone())
        .map(|target_post: BlogPost, blog_posts: Arc<Mutex<VecDeque<BlogPost>>>| {
            let mut posts = blog_posts.lock().unwrap();
            let initial_len = posts.len();
            posts.retain(|post| post.title != target_post.title);
            let response = if posts.len() < initial_len {
                println!("Deleted blog post: {:?}", target_post.title);
                StatusCode::OK
            } else {
                StatusCode::NOT_FOUND
            };
            response // Return response
        });
    let routes = create_post.or(get_posts).or(update_post).or(delete_post).with(cors);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
