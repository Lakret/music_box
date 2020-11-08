import React from "react";
import ReactDOM from "react-dom";
import "./styles/style.scss";

class HelloMessage extends React.Component {
  render() {
    return <h1>Hello {this.props.name}</h1>;
  }
}

var mountNode = document.getElementById("root");
ReactDOM.render(<HelloMessage name="Jane" />, mountNode);
