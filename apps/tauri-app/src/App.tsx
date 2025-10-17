import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';
import { readTextFile } from '@tauri-apps/plugin-fs';
import React from "react";
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
