import React from "react";
import ReactDOM from "react-dom";
import { Button } from "@mui/material";
import "./App.css";
import PrimarySearchAppBar from "./components/PrimarySearchAppBar";

function App() {
  return (
    <div className="AppContent">
      <PrimarySearchAppBar />
    </div>
  );
}

export default App;
