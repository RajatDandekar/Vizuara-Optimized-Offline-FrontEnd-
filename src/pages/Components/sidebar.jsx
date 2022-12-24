import React from "react";
import { useState, useEffect } from "react";

import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
import {listen, emit, once} from "@tauri-apps/api/event";

import { Square } from "../CustomFunctionalities/BasicShapes";

import Image from "next/image";

import ProfileIcon from "../../assets/images/SideBar/Profile_MaleUser.png"

const hasNavigator = typeof navigator !== 'undefined'
const hasWindow = typeof window != 'undefined';
export class SideBar{
//const hasWindow = typeof window !== 'undefined'

static done_loading = false;

constructor(Logo){
    //super();
    const[classes, set_classes] = useState([{ id: 1, value: "Class 8" }, { id: 2, value: "Class 9" }]);
    //const {tauri_interior} = require("@tauri-apps/api/tauri")
    //console.log(SideBar.done_loading);
    //<summary>
    //Use this function to update the state so that it would be displayed
    //</summary>
    useEffect(()=>{

        return async () => {
            //tauri_interior.
            //control++;
            console.log("Use Effect being called " + SideBar.done_loading);
                
            if(!SideBar.done_loading){
                const InitEvent = once("InitializationCompleted", async ()=>{
                    if(!SideBar.done_loading){
                        console.log("Init Event Received");

                        invoke("getclassesdata").then((c) => {
                            
                            console.log(classes);
                            console.log(c);
                            set_classes(c);
                            //console.log("getting classes data -> "+classes);
                            //InitializationCompletedEvent()
                        });

                        SideBar.done_loading = true;
                    }
                })
            }
            /*if(control == 1){
                invoke("getclassesdata").then((c) => {
                    set_classes(c);
                    //console.log("getting classes data -> "+classes);
                    //InitializationCompletedEvent()
                });
            }*/
        };
    }, [classes]);
    //<summary>
    //  async function SendExtractionData(){
    //      await invoke("extractcustom", {archiveFilePath, destinationFilePath});
    //  }
    //</summary>
    async function VisitWebsite(){
        await invoke("visitwebsite", {urlToOpen: "https://vizuara.com"});
    }
    
    function Get_Image_File(file_path){
        if(hasNavigator && hasWindow){
            //console.log("has navigator bro "+localDataDir);
            console.log(convertFileSrc(file_path));
            return convertFileSrc(file_path);
        }
    }

        return(
            <div className="SideBarContainer">
                <SideBarBackground>
                    <SideBarButton ImageToDisplay={Logo} ButtonClass="SideBarButton" value="Vizuara" ClickAction={()=>{VisitWebsite();}}/>
                    <SideBarButton ImageToDisplay={ProfileIcon} ButtonClass="SideBarButton ProfileButton" value="Profile"/>

                    <div className="SideBarClassesContainer">
                    <div className="SideBarClasses">
                        {
                            //classes.map(element => {
                               (<SideBarButton ImageToDisplay={Get_Image_File("C:/Users/User/AppData/Local/VizData/thumbnails/classes/1.jpg")} ButtonClass="SideBarButton" value={"C:/Users/User/AppData/Local/VizData/thumbnails/classes/1.jpg"}/>)
                           // })
                        }
                    </div>
                </div>
            </SideBarBackground>
        </div>
        );
}
}

class SideBarBackground extends React.Component{
    render(){
        return(
            <Square GlobalClassName="SideBarBackgroundBlock">
                {this.props.children}
            </Square>
        );
    }
}

class SideBarButton extends React.Component{
    render(){
        return(
            <button className={this.props.ButtonClass} onClick={this.props.ClickAction}>
                <div  className="SideBarImage">
                    <Image src={this.props.ImageToDisplay} width={100} height={100}/>
                </div>
                <div className="SideBarButtonText">
                    {this.props.value}
                </div>
            </button>
        );
    }
}

export default SideBar;