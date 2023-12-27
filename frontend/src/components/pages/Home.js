import React ,{useState,useEffect}from 'react';
import "../styles/home.css"
const Home = () => {
    const [title,setTitle]= useState('');
    const [content,setContent] = useState('')
    const [posts,setPosts] = useState([]);

    

    const handleSubmit = async (e) =>{
        e.preventDefault();
        const newPost = {title,content};
        const url ='http://localhost:3030/blog';
        try{
            const response = await fetch (url,{
                method:'POST',
                headers:{
                    'Content-Type':'application/json',
                },
                body: JSON.stringify(newPost),
            });
            if (response.ok){
                console.log("Post created successfully");
                setTitle('');
                setContent('');
            }else {
                console.errors("Failed to create post",response.status,response.statusText);
            }
        }catch(error){
            console.error("Failed to connect to the server",error);
        };
        
    }
    const fetchPosts = async () => {
        const url = 'http://localhost:3030/blog';
        const response = await fetch(url);
        const data = await response.json();
        setPosts(data);
    };

    useEffect(() => {
        fetchPosts();
    }, []);
    const handleUpdate = async(postId,updatedPost) =>{
        const url = `http://localhost:3030/blog/${postId}`;
        try{
            const response = await fetch(url,{
                method:'PUT',
                headers:{
                    'Content-Type':'application/json'
                },
                body:JSON.stringify(updatedPost),
            });
            if (response.ok){
                console.log("Post updated successfully");
            }else{
                console.error("Failed to updatepost",response.status.response.statusText);

            }
        }catch (error){
            console.error("Failed to connect to the server",error);
        }
        
    };
    const handleDelete = async (postId) => {
        const url = `http://localhost:3030/blog/${postId}`;
        try {
            const response = await fetch(url, {
                method: 'DELETE',
            });
    
            if (response.ok) {
                console.log("Post deleted successfully");
                // Refresh the list of posts or update the UI accordingly
            } else {
                console.error("Failed to delete post", response.status, response.statusText);
            }
        } catch (error) {
            console.error("Failed to connect to the server", error);
        }
    };
  return (
    <div className="home-container">
      <h1 className="home-title">Welcome to My Blog</h1>
      <p className="home-intro">This is the homepage of my personal blog.</p>
      <form onSubmit={handleSubmit} className="blog-form">
        <input
            type="text"
            value={title}
            onChange={(e) =>setTitle(e.target.value)}
            placeholder="Post Title"
            required
        />
        <textarea
            value={content}
            onChange={(e) =>setContent(e.target.value)}
            placeholder ="Post Content"
            required
        ></textarea>
        <button type="submit">Submit Post</button>

      </form>
      <div>
        {posts.map(post =>(
            <div key={post.title}>
                <h2>{post.title}</h2>
                <p>{post.content}</p>
                <button onClick={() =>handleUpdate(post.id,post)}>Update</button>
                <button onClick={() =>handleDelete(post.id)}>Delete</button>

            </div>
        ))}
      </div>
      
    </div>
  );
};

export default Home;
