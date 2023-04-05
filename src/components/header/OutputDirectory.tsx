import { Button } from "@mui/material";
import { useEffect, useRef, useState } from "react";


export default function OutputDirectory() {
  const [outputDirectory, setOutputDirectory] = useState<String>();

  const LoadOutputDirectory = () => {
      const temp = JSON.parse(localStorage.getItem("outputDirectory") as string);
      
      if (temp) {
        setOutputDirectory(temp);
      }
      else {
        setOutputDirectory("No output path set!");
      }
  }

  useEffect(() => {
    LoadOutputDirectory();
  })


  return (
    <>
      <div className="flex justify-between gap-4 mx-auto items-center px-2 py-2 bg-gray-800 rounded-md">
        <p className="font-bold">Output Directory: </p>
        <p>{outputDirectory}</p>
      </div>
    </>
  )
}
