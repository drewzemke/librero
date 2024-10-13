import { useEffect, useState } from "react";
import "./App.css";

function App() {
  const [apiStuff, setApiStuff] = useState("");

  useEffect(() => {
    fetch("/api")
      .then((data) => data.text())
      .then((text) => setApiStuff(text));
  }, []);

  return (
    <>
      <h1>📚 Librero! 📚</h1>
      <p className="read-the-docs">Here's the result of the call to "/api":</p>
      {apiStuff && <p>{apiStuff}</p>}
    </>
  );
}

export default App;
