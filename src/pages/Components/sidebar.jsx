import React from "react";
import { useState } from "react";

import { invoke } from "@tauri-apps/api/tauri";

import { Square } from "../CustomFunctionalities/BasicShapes";

import Image from "next/image";

import ProfileIcon from "../../assets/images/SideBar/Profile_MaleUser.png"

export class SideBar extends React.Component{

    constructor(props) {
        super(props);
        this.state = {
            classes : [{ id: 1, value: "Class 8" }, { id: 2, value: "Class 9" }]
        };

        this.set_classes = this.set_classes.bind(this) 
      }

    //<summary>
    //Use this function to update the state so that it would be displayed
    //</summary>
    set_classes(new_classes){
        this.setState(new_classes); 
    }
    
    //<summary>
    //  async function SendExtractionData(){
    //      await invoke("extractcustom", {archiveFilePath, destinationFilePath});
    //  }
    //</summary>
    render(){

        async function VisitWebsite(){
            await invoke("visitwebsite", {urlToOpen: "https://vizuara.com"});
        }

        const get_classes_data_from_rust = async () => {
            invoke("getclassesdata").then((classes) => {
                this.set_classes(classes);
                console.log(classes);
            });
        }
    
        return(
            <div className="SideBarContainer" onLoad={get_classes_data_from_rust}>
                <SideBarBackground>
                    <SideBarButton ImageToDisplay={this.props.Logo} ButtonClass="SideBarButton" value="Vizuara" ClickAction={()=>{VisitWebsite();}}/>
                    <SideBarButton ImageToDisplay={ProfileIcon} ButtonClass="SideBarButton ProfileButton" value="Profile"/>
                    <div className="SideBarClassesContainer">
                    <div className="SideBarClasses">
                        {
                        this.state.classes.map(element => {
                            return (<SideBarButton ImageToDisplay={ProfileIcon} ButtonClass="SideBarButton" value={element.value}/>)
                        })
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