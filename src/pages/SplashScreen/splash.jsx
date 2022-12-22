/*#region MyComponents*/

/*#endregion*/

import React from "react";

import { invoke } from "@tauri-apps/api/tauri";
import {listen, emit} from "@tauri-apps/api/event";

import { useState } from "react";

import Image from "next/image";

import Logo from "../../assets/images/logo/vizlogo.gif";
import gifFile from "../../assets/images/VizuaraLoadingScreen.gif"

import styles from "./splash.module.css";

const hasWindow = typeof window !== 'undefined'

/*#endregion*/
 function App() {

  const [LoadingDescription, setLoadingText] = useState("");
  const [ApplicationVersion, setApplicationVersionText] = useState("");

  /*#region Event Listening Testing*/

  if (hasWindow) {
  // your code

  console.log("Actually running");
  const unlisten_loading_desc = listen("LoadingDescription", event =>{
      //console.log(event.payload.message);
      setLoadingText(event.payload.message);
  })

  const unlisten_version_desc = listen("ApplicationVersion", event =>{
    //console.log(event.payload.message);
      setApplicationVersionText(event.payload.message);
  })

  const InitializationCompleted = listen("InitializationCompleted", event =>{
      //console.log("Loading Completed!");
      emit("InitializationCompleted")
  })
}

async function LoadingScreenLoaded(){
  if(hasWindow){
  await invoke("loadingscreenloaded");
  }
}
  return (
    <div className={styles.SplashScreen} onLoad={LoadingScreenLoaded}>
    <div className={styles.SplashContainer}>

      <div className={styles.SplashTextContainer}>
        <Text text={LoadingDescription}/>
      </div>

      <div className={styles.SplashVersionContainer}>
        <Text text={ApplicationVersion}/>
      </div>

      <Image src={gifFile} className={styles.Video}/>
    </div>
    </div>
  );
}

class Text extends React.Component{
  render(){
    return(
      <text className={styles.LoadingText}>{this.props.text}</text>
    );
  }
}

export default App;