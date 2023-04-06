import { Box, Button, Grid, TextareaAutosize, Typography } from "@mui/material";
import Textarea from '@mui/joy/Textarea';
import { useEffect, useRef, useState } from "react";
import  Editor, { useMonaco } from "@monaco-editor/react";

const FileAction = ({ fileBlob }: { fileBlob: Blob | undefined }) => {
  const [fileContent, setFileContent] = useState<string | ArrayBuffer | null>(
    ""
  );
  const [fileSet, setFileSet] = useState<boolean>(false);
  const monaco = useMonaco();

  useEffect(() => {
    if (fileBlob) {
      loadFile()
      setFileSet(true)
    }

  })

  const setEditorContent = (content: string) => {
    monaco?.editor.getModels()[0]?.setValue(content);
  } 

  const loadFile = () => {
    const reader = new FileReader();

    reader.addEventListener(
      "load",
      () => {
        setFileContent(reader.result);
        setEditorContent(reader.result as string);
      },
      false
    );

    if (fileBlob) {
      reader.readAsText(fileBlob);
    }
  };

  return (
    <>
      <div className="px-2 py-2 rounded-md">
        <Editor
          height="30vh"
          theme="vs-light"
          defaultLanguage="text"
          defaultValue={"File Content Here!"}
        />
      </div>
    </>
  );
};

export default FileAction;
