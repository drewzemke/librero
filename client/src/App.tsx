import { useEffect, useState } from "react";

import "./App.css";
import { Libro } from "./api";

function App() {
  const [libros, setLibros] = useState<Libro[]>([]);

  useEffect(() => {
    fetch("/api/libros")
      .then((data) => data.json())
      .then((data) => setLibros(data));
  }, []);

  return (
    <>
      <h1 className="bg-dark header">Librero</h1>
      <div className="bg-light container two-columns">
        {libros.length > 0 && (
          <ul className="libro-list">
            {libros.map((libro) => (
              <li key={libro.isbn} className="libro-item">
                <img src={libro.cover_path} />
                <span className="title">{libro.title}</span>
                <span className="author">{libro.author}</span>
              </li>
            ))}
          </ul>
        )}
        <h2>Featured</h2>
      </div>
      <div className="bg-medium container two-columns">
        <h2>Search</h2>
        <input />
      </div>
      <div className="bg-light container two-columns">
        {libros.length > 0 && (
          <ul className="libro-list">
            {libros.map((libro) => (
              <li key={libro.isbn} className="libro-item">
                <img src={libro.cover_path} />
                <span className="title">{libro.title}</span>
                <span className="author">{libro.author}</span>
              </li>
            ))}
          </ul>
        )}
        <h2>Recent Additions</h2>
      </div>
    </>
  );
}

export default App;
