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

  /*#region Event Listening Testing*/

  if (hasWindow) {
  // your code

  console.log("Actually running");
  const unlisten = listen("LoadingDescription", event =>{
      //console.log(event.payload.message);
      setLoadingText(event.payload.message);
  })

  const InitializationCompleted = listen("InitializationCompleted", event =>{
      console.log("Loading Completed!");
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
        <LoadingText LoadingDescription={LoadingDescription}/>
      </div>
      <Image src={gifFile} className={styles.Video}/>
    </div>
    </div>
  );
}

class LoadingText extends React.Component{
  render(){
    return(
      <text className="LoadingText">{this.props.LoadingDescription}</text>
    );
  }
}

export default App;