/*#region MyComponents*/
import {TitleBar} from "./Components/titlebar"
import { SideBar } from "./Components/sidebar";
/*#endregion*/

import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import reactLogo from "../assets/react.svg";
import tauriLogo from "../assets/tauri.svg";
import nextLogo from "../assets/next.svg";

import Logo from "../assets/images/logo/vizlogo.gif";

/*#region Archive Testing*/

/*#endregion*/

function App() {
  const [archiveFilePath, setArchiveFilePath] = useState("");
  const [destinationFilePath, setDestinationFilePath] = useState("");

  async function SendExtractionData(){
    await invoke("extractcustom", {archiveFilePath, destinationFilePath});
  }
  return (
    <div className="MainContainer">
      <TitleBar Logo={Logo} Title="Vizuara: Teacher Portal"/>
      <SideBar Logo={Logo}></SideBar>
      <div className="ContentContainer">
        You can test out the extract-functionality using this, enter the path of the file and destination directory below
        <br></br>
        Archive Path: <input type="text" value = {archiveFilePath} onChange={e=>{setArchiveFilePath(e.currentTarget.value);}}/>
        <br></br>
        Destination Path: <input type="text" value = {destinationFilePath} onChange={e => {setDestinationFilePath(e.currentTarget.value)}}/>
        <br></br>
        <button onClick={SendExtractionData}>Extract</button>
      </div>
    </div>
  );
}

export default App;
