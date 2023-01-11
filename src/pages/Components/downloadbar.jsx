import React from "react";
import { useState, useEffect } from "react";

import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
import {listen, emit, once} from "@tauri-apps/api/event";

import { Square } from "../CustomFunctionalities/BasicShapes";

import Image from "next/image";

import ProfileIcon from "../../assets/images/SideBar/Profile_MaleUser.png"
import DownloadIcon from "../../assets/images/SideBar/downloads.png"

import {ContentDisplayer} from "./content_list_displayer";

import downloadbar_styles from "./design/downloadbar.module.css";

const hasNavigator = typeof navigator !== 'undefined'
const hasWindow = typeof window != 'undefined';

export class DownloadBar extends React.Component{
//const hasWindow = typeof window !== 'undefined'

    constructor(props){
        super(props);
        this.state = {
            show: false,
            text: "Downloading"
        }
    }

    render(){

        listen("FileStartDownload", (message)=>{
            this.setState({show : true, text : "Downloading"})
        })

        listen("FileEndDownload", (message) => {
            this.setState({show : false, text: "Downloading Ended"})
        })

        listen("FileBeingDownloaded", (message) => {
            this.setState({show : true, text : message.payload.message})
        })
        if(this.state.show == true){
            return(
                <DownloadBarBackground GlobalClassName={downloadbar_styles.downloadbar}>
                    {this.state.text}
                </DownloadBarBackground>
            );
        }else{
            return
        }
    }
}

class DownloadBarBackground extends React.Component{
    render(){
        return (
            <Square GlobalClassName={this.props.GlobalClassName}>
                {this.props.children}
            </Square>
        );
    }
}
export default DownloadBar;