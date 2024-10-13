import { useEffect, useState } from "react";

import "./App.css";
import { Libro } from "./api";

function App() {
  const [apiStuff, setApiStuff] = useState("");
  const [libros, setLibros] = useState<Libro[]>([]);

  useEffect(() => {
    fetch("/api")
      .then((data) => data.text())
      .then((text) => setApiStuff(text));
    fetch("/api/libros")
      .then((data) => data.json())
      .then((data) => setLibros(data));
  }, []);

  return (
    <>
      <h1>📚 Librero! 📚</h1>
      <p className="read-the-docs">Aquí están tus libros:</p>
      {libros.length > 0 && (
        <ul>
          {libros.map((libro) => (
            <li>
              {libro.isbn}: {libro.title} por {libro.author}
            </li>
          ))}
        </ul>
      )}
      <p className="read-the-docs">Here's the result of the call to "/api":</p>
      {apiStuff && <p>{apiStuff}</p>}
    </>
  );
}

export default App;
