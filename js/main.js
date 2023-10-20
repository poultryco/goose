import {EditorView, minimalSetup} from "codemirror"
import {EditorState} from "@codemirror/state"
import {markdown} from "@codemirror/lang-markdown"

let root = document.querySelector(":root");

let sync_val = "";

function char_count(e) {
  let chars = 0;

  e.split("").forEach((e) => {
    chars += 1;
  });
  document.getElementById("charcount").innerText = chars + " characters";
}

let state = EditorState.create({
  extensions: [
    markdown(), 
    minimalSetup, 
    EditorView.lineWrapping,
    EditorView.updateListener.of(function(e) {
      char_count(e.state.doc.toString());
    })
  ]
})

let view = new EditorView({
  state: state,
  parent: document.getElementById("editor"),
})

document.getElementById("fsize").addEventListener("click", () => {
  console.log("cicket");
});