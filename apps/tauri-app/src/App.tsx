import { Routes, Route } from "react-router-dom";
import Bookshelf from "./components/Bookshelf";
import Reader from "./components/Reader";
import "./App.css";

function App() {
  return (
    <div className="App">
      <Routes>
        <Route path="/" element={<Bookshelf />} />
        <Route path="/reader/:filename" element={<Reader />} />
      </Routes>
    </div>
  );
}

export default App;
