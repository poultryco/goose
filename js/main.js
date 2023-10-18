import {EditorView, minimalSetup} from "codemirror"
import {markdown} from "@codemirror/lang-markdown"

let view = new EditorView({
  extensions: [markdown(), minimalSetup, EditorView.lineWrapping],
  parent: document.getElementById("editor"),
  fontSize: 20,
})