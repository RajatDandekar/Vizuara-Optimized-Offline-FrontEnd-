import React from "react";
import { invoke } from "@tauri-apps/api/tauri";

import { Square } from "../CustomFunctionalities/BasicShapes";

import Image from "next/image";

import ProfileIcon from "../../assets/images/SideBar/Profile_MaleUser.png"

export class SideBar extends React.Component{
    render(){

        async function VisitWebsite(){
            await invoke("visitwebsite", {urlToOpen: "https://vizuara.com"});
        }

        return(
            <div className="SideBarContainer">
                <SideBarBackground>
                    <SideBarButton ImageToDisplay={this.props.Logo} ButtonClass="SideBarButton" value="Vizuara" ClickAction={VisitWebsite}/>
                    <SideBarButton ImageToDisplay={ProfileIcon} ButtonClass="SideBarButton ProfileButton" value="Profile"/>
                    <div className="SideBarClassesContainer">
                    <div className="SideBarClasses">
                        <SideBarButton ImageToDisplay={"data:image/jpeg;base64,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"} ButtonClass="SideBarButton" value="Class 6"/>
                        <SideBarButton ImageToDisplay={ProfileIcon} ButtonClass="SideBarButton" value="Class 7"/>
                        <SideBarButton ImageToDisplay={ProfileIcon} ButtonClass="SideBarButton" value="Class 8"/>
                        <SideBarButton ImageToDisplay={ProfileIcon} ButtonClass="SideBarButton" value="Class 9"/>
                        <SideBarButton ImageToDisplay={ProfileIcon} ButtonClass="SideBarButton" value="Class 10"/>
                        <SideBarButton ImageToDisplay={ProfileIcon} ButtonClass="SideBarButton" value="Class 10"/>
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