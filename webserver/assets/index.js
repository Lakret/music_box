import React, { useState, useEffect } from "react";
import ReactDOM from "react-dom";
import "./styles/style.scss";
// TODO:
// import listArtists from "./api";

async function listArtists() {
  return fetch('/api/artists').then(data => data.json())
}

function ExampleHooksComponent() {
  const [reloadCount, setReloadCount] = useState(0);
  const [artists, setArtists] = useState([]);

  useEffect(async () => {
    listArtists().then(artists => setArtists(artists));
  }, [reloadCount]);

  return (
    <div>
      <p>Your artist: {JSON.stringify(artists)}</p>
      <p>You reloaded {reloadCount} times</p>
      <button onClick={() => setReloadCount(reloadCount + 1)}>
        Reload Artists
      </button>
    </div>
  );
}

var mountNode = document.getElementById("root");
ReactDOM.render(<ExampleHooksComponent />, mountNode);
