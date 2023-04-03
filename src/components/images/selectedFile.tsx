import { Button } from "@mui/material";
import { useRef, useState } from "react";


export default function SelectedFile() {


  return (
    <><div>
        <Button
          variant="contained"
          component="label"
        >
          Select Image
          <input
            id="path-input"
            type="file"
            hidden
          />
        </Button>
        <Button variant="contained" >Upload Selected</Button>
    </div>

    </>
  )
}
