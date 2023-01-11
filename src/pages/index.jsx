/*#region MyComponents*/
import {TitleBar} from "./Components/titlebar"
import {SideBar} from "./Components/sidebar";
import {ContentDisplayer} from "./Components/content_list_displayer";
import {DownloadBar} from "./Components/downloadbar";
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
      <div className="ContentContainer">
        <ContentDisplayer></ContentDisplayer>
      </div>
      <SideBar Logo={Logo}></SideBar>
      <TitleBar Logo={Logo} Title="Vizuara: Teacher Portal"/>
      <DownloadBar/>
    </div>
  );
}

export default App;
