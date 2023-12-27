use warp::Filter;
use warp::http::StatusCode;
use serde::{Serialize, Deserialize};
use std::sync::{Arc,Mutex};
use std:: collections::VecDeque;



#[derive(Serialize, Deserialize, Debug,Clone)] // Ensure Serialize, Deserialize, and Debug are derived
struct BlogPost {
    id:usize,
    title: String,
    content: String,
}

#[tokio::main]
async fn main() {
    //Initialize an in-memory store for blog posts
    let blog_post = Arc::new(Mutex::new(VecDeque::new()));
    let next_id = Arc::new(Mutex::new(1));
    
    //Clone the Arc to move a refence into the filters
    let blog_posts_filter = warp::any().map(move || blog_post.clone());
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type"])
        .allow_methods(vec!["GET","POST","PUT","DELETE"]);
    // Note: Assuming `next_id` is an `Arc<Mutex<usize>>`
    let create_post = warp::path("blog")
        .and(warp::post())
        .and(warp::body::json())
        .and(blog_posts_filter.clone())
        .and_then(move |new_post: BlogPost, blog_posts: Arc<Mutex<VecDeque<BlogPost>>>| {
            let next_id = next_id.clone(); // Move the clone inside the closure
            async move {
                let mut id_guard = next_id.lock().unwrap();
                let mut posts = blog_posts.lock().unwrap();
                let post_with_id = BlogPost {
                    id: *id_guard, // Assign the next available ID
                    title: new_post.title,
                    content: new_post.content,
                };
                posts.push_back(post_with_id);
                *id_guard += 1; // Increment the ID counter
                println!("New blog post: {:?}", posts.back().unwrap());
                let reply = warp::reply::with_status("Created", StatusCode::CREATED);
                Ok::<_, warp::Rejection>(reply)
            }
        });


    let get_posts = warp::path("blog")
        .and(warp::get())
        .and(blog_posts_filter.clone())
        .map(|blog_posts:Arc<Mutex<VecDeque<BlogPost>>>|{
            let posts = blog_posts.lock().unwrap();
            warp::reply::json(&*posts)
        });
    // PUT endpoint to update a blog post by id
    let update_post = warp::path!("blog" / usize) // 'usize' is the type of your id
        .and(warp::put())
        .and(warp::body::json::<BlogPost>())
        .and(blog_posts_filter.clone())
        .map(|id: usize, updated_post: BlogPost, blog_posts: Arc<Mutex<VecDeque<BlogPost>>>| {
            let mut posts = blog_posts.lock().unwrap();
            let mut response = StatusCode::NOT_FOUND;
            
            for post in posts.iter_mut() {
                if post.id == id { // Compare IDs
                    post.title = updated_post.title.clone(); // Update title
                    post.content = updated_post.content.clone(); // Update content
                    response = StatusCode::OK;
                    println!("Updated blog post: {:?}", post);
                    break; // Exit loop after updating
                }
            }
            
            response // Return response
        });

    // DELETE endpoint to delete a blog post
    let delete_post = warp::path!("blog"/usize)
        .and(warp::delete())
        //.and(warp::body::json())
        .and(blog_posts_filter.clone())
        .map(|id: usize, blog_posts: Arc<Mutex<VecDeque<BlogPost>>>| {
            let mut posts = blog_posts.lock().unwrap();
            let initial_len = posts.len();
            posts.retain(|post| post.id != id);
            let response = if posts.len() < initial_len {
                println!("Deleted blog post: {:?}", id);
                StatusCode::OK
            } else {
                StatusCode::NOT_FOUND
            };
            response // Return response
        });
    let routes = create_post.or(get_posts).or(update_post).or(delete_post).with(cors);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
