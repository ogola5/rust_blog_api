import React ,{useState}from 'react';
import "../styles/home.css"
const Home = () => {
    const [title,setTitle]= useState('');
    const [content,setContent] = useState('')

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
      
    </div>
  );
};

export default Home;
