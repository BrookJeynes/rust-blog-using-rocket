import React, {useState, useEffect} from 'react';
import {AiOutlinePlus, AiOutlineClose} from 'react-icons/ai';
import './App.css';

interface Post {
  id: number,
  title: string,
  body: string,
  genre: string,
  published: boolean,
}

const BlogPost = (props: any) => {
  const post: Post = props.post;
  
  return (
    <div className="flex justify-end items-start h-[8rem] w-11/12 mx-5 p-5 border-t border-neutral-700 text-white">
      <div className="w-1/4">
        <span className="text-lg rounded-[3rem] bg-white text-[#151515] py-1 px-4 font-light">{post.genre.toUpperCase()}</span>
      </div>
      <div className="w-full font-medium text-2xl cursor-pointer" onClick={() => {}}>
        {post.title}
      </div>
      <div className="flex flex-col items-end justify-between h-full w-full">
        <div className="flex w-full justify-end">
          <button className={`mr-10 ${post.published && "opacity-40"}`} disabled={post.published} onClick={async () => await props.publishPost(post.id)}>Publish</button>
          <button className={`mx-1`} onClick={async () => await props.deletePost(post.id)}>Delete</button>
        </div>
        { post.published && <span className="ml-1 text-sm opacity-70">Published</span> }
      </div>
    </div>
  );
};

const App = () => {
  const [posts, setPosts] = useState(Array<Post>);
  const [loading, setLoading] = useState(true);
  const [openModal, setOpenModal] = useState(false);

  const deletePost = async (id: number) => {
    const posts = await fetch(`/api/delete/${id}`).then(res => res.json());

    setPosts(posts.body.Posts);
  };

  const createPost = async (title: string, genre: string, body: string) => {
    const post = await fetch("/api/new_post", {
      method: "POST", 
      headers: {
        "content-type": "application/json"
      },
      body: JSON.stringify({
        title: title,
        body: body,
        genre: genre
      })
    }).then(res => res.json());

    setPosts([...posts, post.body.Post].sort((a, b) => {
      return a.id - b.id;
    }));
    setOpenModal(false);
  }

  const publishPost = async (id: number) => {
    const post = await fetch(`/api/publish/${id}`).then(res => res.json());

    setPosts([post.body.Post, ...posts.filter(e => e.id !== post.body.Post.id)].sort((a, b) => {
      return a.id - b.id;
    }));
  };

  const handleSubmit = async (e: any) => {
    e.preventDefault();

    const title = e.target.elements.title.value;
    const genre = e.target.elements.genre.value;
    const body = e.target.elements.body.value;

    await createPost(title, genre, body)
  }

  const ModalForm = (props: any) => {
    return (
      <div className="flex flex-col absolute p-5 w-3/4 h-3/4 bg-[#151515] border border-white rounded-lg overflow-scroll scrollbar-hide">
        <div className="flex justify-between items-center text-white text-xl mb-10 font-medium pb-5 border-b">
          Create Blog Post
          <div className="cursor-pointer" onClick={() => setOpenModal(false)}>
            <AiOutlineClose color="white" size={24} />
          </div>
        </div>

        <form onSubmit={handleSubmit} className="flex flex-col justify-between h-full w-full text-white">
          <div className="flex flex-col">
            <div className="flex justify-between mb-10">
              <div className="w-full">
                <span className="text-lg">Title:</span>
                <input type="text" name="title" className="border-b bg-[#151515] ml-5 focus:outline-none w-2/3" />
              </div> 
              <div className="w-full">
                <span className="text-lg">Genre:</span>
                <input type="text" name="genre" className="border-b bg-[#151515] ml-5 focus:outline-none w-4/12" />
              </div>
            </div>

            <div className="w-full flex flex-col">
              <span className="text-lg">Body:</span>
              <textarea rows={20} name="body" className="border bg-[#151515] mt-4 focus:outline-none w-1/2 max-h-[15rem] p-2"></textarea>
            </div>
          </div>

          <div className="w-full flex justify-end items-center">
            <input type="submit" value="Create post" className="cursor-pointer" />
          </div>
        </form>
      </div>
    );
  };

  useState(() => {
    const fetchPosts = async () => {
      const posts = await fetch("/api/").then(res => res.json()); 

      setPosts(posts.body.Posts);
    }

    fetchPosts().finally(() => setLoading(false))
  });

  return (
    <div className="flex justify-center items-center w-full">
      { loading ?
        <span>Loading...</span>
      :
        <div className="flex flex-col justify-center items-center p-5 w-full h-screen bg-[#151515]">
          { posts.map(post => <BlogPost key={post.id} post={post} deletePost={deletePost} publishPost={publishPost} />)}
          { openModal && <ModalForm /> }
          <div className="select-none absolute bottom-12 right-12 text-white w-[7rem] h-[3rem] flex items-center justify-center cursor-pointer" onClick={() => setOpenModal(true)}>
            <AiOutlinePlus size={24} /> <span className="ml-1 font-medium">Add post</span>
          </div>
        </div>
      }
    </div>
  );
}

export default App;
