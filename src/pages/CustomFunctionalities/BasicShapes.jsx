import React from "react";

export class Square extends React.Component{
    render(){
        return(
            <div className={this.props.GlobalClassName}>
                {this.props.children}
            </div>
        );
    }
}

export class BasicShapes extends React.Component{
    render(){
        return(
            <div>

            </div>
        );
    }
}

export default BasicShapes;
