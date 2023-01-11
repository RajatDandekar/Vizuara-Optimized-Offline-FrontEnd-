import React from "react";
import { invoke } from "@tauri-apps/api/tauri";

import Image from "next/image";

import {Square} from "../CustomFunctionalities/BasicShapes";

import styles from "../../CustomCSS/TitleBar.module.css";

//Main Title Bar Component Export this
export class TitleBar extends React.Component{
    render(){
        return (
        <div className="TitleBarContainer">
        <TitleBarBackgroundBlock>
            <TitleLogo ImageSource={this.props.Logo} TitleName={this.props.Title}>
                <TitleBarActionButtons/>
            </TitleLogo>
        </TitleBarBackgroundBlock>
        </div>
        );
    }
}

class TitleLogo extends React.Component{
    render(){
        return(
            <div>
                <Image src={this.props.ImageSource} width="20px" height="20px"/>
                <p className="TitleBarTitle">{this.props.TitleName}</p>
                <div>
                    {this.props.children}
                </div>
            </div>
        );
    }
}

//Background Block
class TitleBarBackgroundBlock extends React.Component{
    render(){
        return(
        <Square GlobalClassName="TitleBarBackgroundBlock">
            {this.props.children}
        </Square>
        );
    }
}

//#region Buttons
class TitleBarActionButtons extends React.Component{
    render(){

        async function RedButton() {
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            await invoke("quitapplication");
          }

          async function YellowButton(){
            await invoke ("resizewindow");
          }

          async function GreenButton(){
            await invoke ("minimizewindow");
          }

        return(

            <div className="TitleBarButtons">
                <ActionButton ClassName = {styles.GreenButton} onClick={GreenButton}/>
                <ActionButton ClassName = {styles.YellowButton} onClick={YellowButton}/>
                <ActionButton ClassName = {styles.RedButton} onClick={RedButton}/>
            </div>
        );
    }
}

class ActionButton extends React.Component{
    render(){

        return(
            <button className={this.props.ClassName} onClick={this.props.onClick}></button>
        );
    }
}

export default TitleBar;
//#endregion