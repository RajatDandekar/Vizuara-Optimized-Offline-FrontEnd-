import React from "react";
import { useState, useEffect } from "react";

import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
import {listen, emit, once} from "@tauri-apps/api/event";

import { Square } from "../CustomFunctionalities/BasicShapes";

import Image from "next/image";

import ProfileIcon from "../../assets/images/SideBar/Profile_MaleUser.png"
import DownloadIcon from "../../assets/images/SideBar/downloads.png"

import {ContentDisplayer} from "./content_list_displayer";

import sidebar_styles from "./design/sidebar.module.css";

const hasNavigator = typeof navigator !== 'undefined'
const hasWindow = typeof window != 'undefined';

export class SideBar extends React.Component{
//const hasWindow = typeof window !== 'undefined'

static done_loading = false;
static selected_class_id = 0; //0 because no chapters is selected
static selected_class_name = ""; //Current Class Name

constructor(props){
    super(props);
    this.state = {
        classes : [{ id: 1, name: "Class 8", thumbnailpath: "" }, { id: 2, name: "Class 9", thumbnailpath: "" }]
    }
}

render(){
    
    /*#region preparation */

    //<summary>
    //if will control the process so that it will only run for one time
    //</summary>
    if(!SideBar.done_loading){
        const InitEvent = once("InitializationCompleted", async ()=>{
            if(!SideBar.done_loading && hasWindow && hasNavigator){
                console.log("Init Event Received");
                invoke("getclassesdata").then((c) => {
                    //console.log(c[0].id);
                    //this.state.classes = c;
                    this.setState({classes: c});
                    //console.log(this.state.classes[0].thumbnailpath);
                    select_class_to_display(this.state.classes[0].id, this.state.classes[0].name);
                });

                SideBar.done_loading = true;
            }
        })
    }
    /*#endregion*/

    /*#region Function To Run*/
    async function VisitWebsite(){
        await invoke("visitwebsite", {urlToOpen: "https://vizuara.com"});
    }
    
    function Get_Image_File(file_path){
            //console.log("has navigator bro "+localDataDir);
            //console.log("Original Path: "+file_path+", Converted File Path: "+convertFileSrc(file_path));
            //console.log("https://asset.localhost/"+file_path.replace("\\","\\\\"));
            return ("https://asset.localhost/"+file_path.replace("\\","\\\\"));
    }

    function select_class_to_display(class_id, class_name){
        SideBar.selected_class_id = parseInt(class_id);
        SideBar.selected_class_name = class_name;

        console.log("selecting class to display id -> " + class_id);
        invoke("set_class_id_method", {classId : SideBar.selected_class_id});
    }   
    /*#endregion*/

    /*#region return*/
    return (
        <div className={sidebar_styles.SideBarContainer}>
        <SideBarBackground>
            <SideBarButton ImageToDisplay={this.props.Logo} ButtonClass={sidebar_styles.SideBarButton} value="Vizuara" ClickAction={()=>{VisitWebsite();}}/>
            <SideBarButton ImageToDisplay={ProfileIcon} ButtonClass={sidebar_styles.SideBarButton+" "+sidebar_styles.ProfileButton} value="Profile"/>

            <div className={sidebar_styles.SideBarClassesContainer}>

                <div className={sidebar_styles.SideBarClasses}>
                {
                    this.state.classes.map(function(element, i){
                        return <SideBarButton ImageToDisplay={Get_Image_File(element.thumbnailpath)} ButtonClass={sidebar_styles.SideBarButton} value={element.name} ClickAction={()=>{select_class_to_display(element.id, element.name)}}/>
                    })
                }
                </div>

            </div>
        </SideBarBackground>
    </div>
    );
    /*#endregion */
}
}

class SideBarBackground extends React.Component{
    render(){
        return(
            <Square GlobalClassName={sidebar_styles.SideBarBackgroundBlock}>
                {this.props.children}
            </Square>
        );
    }
}

class SideBarButton extends React.Component{
    render(){
        return(
            <button className={this.props.ButtonClass} onClick={this.props.ClickAction}>
                <div  className={sidebar_styles.SideBarImage}>
                    <Image className={sidebar_styles.SideBarImageActual} src={this.props.ImageToDisplay} width={70} height={70}/>
                </div>
                <div className={sidebar_styles.SideBarButtonText}>
                    {this.props.value}
                </div>
            </button>
        );
    }
}

export default SideBar;