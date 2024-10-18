import { useEffect, useState } from "react";

import "./App.css";
import { AddLibro, Libro } from "./api";

function App() {
  const [libros, setLibros] = useState<Libro[]>([]);

  const getLibros = () =>
    fetch("/api/libros")
      .then((data) => data.json())
      .then((data) => setLibros(data));

  useEffect(() => {
    getLibros();
  }, []);

  const addBook = () => {
    const newLibro: AddLibro = { isbn: Math.random().toString() };
    fetch("/api/libros", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(newLibro),
    }).then(getLibros);
  };
  return (
    <>
      <h1 className="bg-dark header">Librero</h1>
      <div className="bg-light container two-columns">
        {libros.length > 0 && (
          <ul aria-label="featured-libros" className="libro-list">
            {libros.map((libro) => (
              <li
                key={libro.isbn}
                aria-label={libro.title}
                className="libro-item"
              >
                <img src={libro.cover_path ?? ""} />
                <span className="title">{libro.title}</span>
                <span className="author">{libro.author}</span>
              </li>
            ))}
          </ul>
        )}
        <h2>Featured</h2>
      </div>
      <button type="button" onClick={addBook}>
        Add Book
      </button>
    </>
  );
}

export default App;
