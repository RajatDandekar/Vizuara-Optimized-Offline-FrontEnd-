import React from "react";
import { useState, useEffect } from "react";

import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
import {listen, emit, once} from "@tauri-apps/api/event";

import content_style from "./design/content_displayer.module.css";
import SideBar from "./sidebar";
import Image from "next/image";

import viz_image from "../../assets/images/logo/vizlogo.gif"

import lab_image from "../../assets/images/icons/laboratory_icon.png"
import vid_image from "../../assets/images/icons/video_icon.jpg"

function update_selection_class(class_id){
    invoke("set_class_id_method", {classId : class_id})
}

function update_selection_chapter(chapter_id){
    invoke("set_chapter_id_method", {chapterId: chapter_id})
}

function update_selection_file_type(file_type){
    invoke("set_file_type_method", {fileType: file_type})
}

function update_selection_file(file_id){
    invoke("set_file_method", {file: file_id})
}
//<summary>
//  This class will contain the Content in cards format, for videos and webgl they will be displayed literally
//
//
//</summary>
export class ContentDisplayer extends React.Component{
    //static [selected_class_name, set_selected_class_name] = useState("");

    constructor(props){
        super(props);
        this.state = {
            //All this same variable here to just to store the data for a while and reused
            class_id: 0,        //the id of the class that is currently selected    //(if 0 nothing will be displayed)
            chapter_id: 0,      //the id of the chapter that is currently selected  //(if 0, chapters will be displayed)
            file_type: "",
            file_id: 0,         //the id of the file that is currently selected     //(if 0, available selection of files will be displayed, if 1, the content will be displayed)
        
            contents :   [{
                name        : "Forces and Motion",
                subname     : "Chapter 1",
                id          : "1",
                available   : true
            }],
            content_count: 1
        }
    }
    render(){

        const display_text_event = listen("SelectionUpdated",async (message) => {
            //sets(message)

            console.log(message);

            if(message.payload.chapter_id == 0){
                
                //this.render();
                invoke("get_chapters_within_current_classes").then((...msg)=>{
                    
                    console.log(msg);
                    //Content_Title.class_text = message.payload.message;
                    this.setState({
                            class_id: message.payload.class_id, 
                            chapter_id: 0, 
                            file_type: "",
                            file_id: 0,
                
                            contents: msg[0],
                            content_count: msg[0].length
                    });
                });
            }else{
                if(message.payload.file_type == ""){
                    
                    invoke("get_file_types").then((...msg)=>{
                        console.log(msg);
                        //Content_Title.class_text = message.payload.message;
                        this.setState({
                                class_id: message.payload.class_id, 
                                chapter_id: message.payload.chapter_id, 
                                file_type: "",
                                file_id: 0,
                
                                contents: msg[0],
                                content_count: msg[0].length
                        });
                    });
                }else{
                    if(message.payload.file_id == 0){
                        
                        invoke("get_files").then((...msg)=>{
                            console.log(msg);
                            //Content_Title.class_text = message.payload.message;
                            this.setState({
                                    class_id: message.payload.class_id, 
                                    chapter_id: message.payload.chapter_id, 
                                    file_type: message.payload.file_type,
                                    file_id: 0,
                
                                    contents: msg[0],
                                    content_count: msg[0].length
                            });
                        });
                    }else{
                        invoke("get_display_file_path").then((display_file_path)=>{
                            
                            this.setState({
                                class_id: message.payload.class_id, 
                                chapter_id: message.payload.chapter_id, 
                                file_type: message.payload.file_type,
                                file_id: message.payload.file_id,
                                
                                html_path: display_file_path,
                                contents: null,
                                content_count: 0
                            });
                        });

                    }
                }
            }
        });
        
        function Get_PATH_ASSET(file_path){
            //console.log("has navigator bro "+localDataDir);
            //console.log("Original Path: "+file_path+", Converted File Path: "+convertFileSrc(file_path));
            //console.log("https://asset.localhost/"+file_path.replace("\\","\\\\"));
            return ("https://asset.localhost/"+file_path.replace("\\","\\\\"));
        }

        function display_content_displayer(class_id, chapter_id, file_type, file_id, content_type, contents_to_display, content_count, html_path){
            if(class_id != 0){
                if(chapter_id == 0){

                    const rows = [];
                    //Display Chapters
                    let key = 0;
                    for (let i = 0; i < content_count; i++){
                        rows.push(
                            <ContentChooser key={key} subname={contents_to_display[i].subname} mainname={contents_to_display[i].name} is_chapter={true} available={contents_to_display[i].available} DownloadLink={contents_to_display[i].link} class_id={class_id} chapter_id={contents_to_display[i].id} click_action={()=>{update_selection_chapter(+contents_to_display[i].id)}}
                                            imagesrc={Get_PATH_ASSET(contents_to_display[i].thumbnailpath)} version={contents_to_display[i].version} shouldupdate={contents_to_display[i].shouldupdate}></ContentChooser>
                        );
                        key++;
                        rows.push(
                            <ContentSpace key={key}></ContentSpace>
                        );
                        key++;
                    }
                    return rows;
                    
                }else{
                    if(file_type != ""){
                        if(file_id == 0){
                            
                        //Display Available Files
                        const rows = [];
                        //Display Files
                        let key = 0;
                        for (let i = 0; i < content_count; i++){

                                rows.push(
                                    <ContentChooser key={key} mainname={contents_to_display[i].name} is_chapter={false} available={true} DownloadLink={""}
                                    class_id={class_id} 

                                    chapter_id={contents_to_display[i].id} 
                                    click_action={()=>{update_selection_file(+contents_to_display[i].id)}}

                                    imagesrc={Get_PATH_ASSET(contents_to_display[i].thumbnail_path)}></ContentChooser>
                                );
                                key++;
                                rows.push(
                                    <ContentSpace key={key}></ContentSpace>
                                );
                                key++;
                            }
                            return rows;

                        }else{
                            //Display the file itself
                            console.log("http://localhost:8080/static/"+class_id+"_"+chapter_id+"/"+file_type+"/"+file_id+"/index.html");
                            return(
                                <iframe className={content_style.iframe_screen} 
                                        src={"http://localhost:8080/static/"+class_id+"_"+chapter_id+"/"+file_type+"/"+file_id+"/index.html"} 
                                        frameborder="0">

                                        </iframe>
                            )
                        }
                    }else{
                        //Display Available File Types
                        const rows = [];
                        //Display Chapters
                        let key = 0;
                        for (let i = 0; i < content_count; i++){
                            var image_to_show = (contents_to_display[i].name=="lab")? lab_image: vid_image;
                            var text_to_display = (contents_to_display[i].name=="lab")? "Laboratories": "Videos";

                            rows.push(
                                <ContentChooser key={key} mainname={text_to_display} is_chapter={false} available={true} DownloadLink={""} class_id={class_id} chapter_id={contents_to_display[i].id} click_action={()=>{update_selection_file_type(contents_to_display[i].name)}}
                                imagesrc={image_to_show}></ContentChooser>
                            );
                            key++;
                            rows.push(
                                <ContentSpace key={key}></ContentSpace>
                            );
                            key++;
                        }
                        return rows;
                    }
                }
            }else{
                return ("None");
            }
        }
        return(
            <div className={content_style.content_background}>
                <div className={content_style.ContentContainer}>
                    {display_content_displayer(this.state.class_id, this.state.chapter_id, this.state.file_type, this.state.file_id, this.state.content_type,this.state.contents, this.state.content_count, this.state.html_path)}
                </div>
                <Content_Title></Content_Title>
            </div>
        );
    }
}

//<summary>
//This title will display the currently selected class + chapters
//<summary>
class Content_Title extends React.Component{

    constructor(props){
        super(props);
        this.state = {
            class_id: 0, //For Searching Purposes
            class_name: "", //For Displaying Purposes
            
            chapter_id: 0,
            chapter_name: "",
          
            file_type: "",

            file_id: 0,
            file_name: ""
        };
    }

    render(){

        const display_text_event = listen("SelectionUpdated", (message) => {
            //sets(message);
            console.log("Message Received-> " + message.payload);
            //Content_Title.class_text = message.payload.message;
            this.setState(message.payload)
            //this.render();
        });

        function main_rendering(class_id, class_name, chapter_id, chapter_name, file_type, file_id, file_name){

                function right_arrow(){
                    return ">";
                }
                function class_button(){
                    if(class_id > 0){
                        return(

                        <button className={content_style.navigation_button} 
                            onClick={()=>{update_selection_class(class_id)}}>
                            <b>
                                {class_name}
                            </b>
                        </button> 
                        
                        );
                    }else{
                        return ("Please select a class from the sidebar.");
                    }
                }

                function chapter_button(){
                    if(chapter_id > 0){
                        return(
                        <div>
                            {right_arrow()}
                            <button className={content_style.navigation_button}
                                onClick={()=>{update_selection_chapter(chapter_id)}}>
                                    <b>
                                    {chapter_name}
                                    </b>
                            </button> 
                        </div>
                        );
                    }
                }

                function file_type_button(){
                    if(file_type != ""){
                        return(
                            <div>
                                {right_arrow()}
                                <button className={content_style.navigation_button} 
                                        onClick={()=>{update_selection_file_type(file_type)}}>
                                            <b>
                                                {(file_type=="lab")?"Laboratory":"Video"}
                                            </b>
                                </button> 
                            </div>
                        );
                    }
                }

                function file_button(){
                    if(file_id > 0){
                        return(
                            <div>
                                {right_arrow()}
                                <button className={content_style.navigation_button}>{file_name}</button> 
                            </div>
                        );
                    }
                }
                return (
                <div className={content_style.header_content}>
                    {class_button()} 
                    {chapter_button()}
                    {file_type_button()}
                    {file_button()}
                </div>
                )
            //}
        }
        return main_rendering(this.state.class_id, this.state.class_name, this.state.chapter_id, this.state.chapter_name, this.state.file_type ,this.state.file_id, this.state.file_name)
              
    }
}

class ContentChooser extends React.Component{

    render(){

        var download_button_visibility = this.props.available? "hidden": "visible";
        var delete_button_visibility = this.props.available? "visible": "hidden";

        function display_downloadbutton_or_deletebutton(available, is_chapter, class_id, chapter_id, download_link, version_code, shouldupdate){
            
            if(is_chapter){
                if(available){
                    if(!shouldupdate) {
                    return (
                        <div className={content_style.interior_button} onClick={async (e)=>{
                            e.stopPropagation();
                            console.log("delete button");
                            await invoke("delete_chapter", {folder: class_id + "_" + chapter_id});
                        }}>Delete</div>
                    )
                 }else{
                    return(
                    <div className={content_style.interior_button} onClick={async (e)=>{
                        e.stopPropagation();
                        await invoke("download_data_and_extract", {data : download_link, folder: class_id + "_" + chapter_id, version: version_code });
                    }}>
                        Update
                    </div>
                    )
                 }
                }else{
                    return(
                        <div className={content_style.interior_button} onClick={async (e)=>{
                            e.stopPropagation();
                            await invoke("download_data_and_extract", {data : download_link, folder: class_id + "_" + chapter_id, version: version_code });
                        }}>
                            Download
                        </div>
                    )
                }
            }
        }
        return(
                <button className={content_style.card} onClick={()=>{
                    
                    if(this.props.available){
                        this.props.click_action()
                    }else{
                        console.log("not available, do nothing here");
                    }
                    }}>
                    <div className={content_style.image_container + " " + ((this.props.available)? content_style.available: "")}>
                    
                        <Image src={this.props.imagesrc} width={100} height={100}>
                        
                        </Image>

                        <h4 className={content_style.ChapterNumber}>{this.props.subname}</h4>
                        <h5 className={content_style.ChapterName}> {this.props.mainname}</h5>
                    </div>
                    <div>
                        <div className={content_style.button_container}>
                            {
                                display_downloadbutton_or_deletebutton(this.props.available, this.props.is_chapter, this.props.class_id, this.props.chapter_id, this.props.DownloadLink, this.props.version, this.props.shouldupdate)
                            }
                        </div>
                    </div>
                </button>
        );
    }
}
class ContentSpace extends React.Component{

    render(){
        return(
            <div className={content_style.cardspace}>
            </div>
        )
    }
}
export default ContentDisplayer;

