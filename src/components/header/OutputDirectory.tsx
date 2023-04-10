import { useEffect, useState } from "react";
import { listen } from '@tauri-apps/api/event'

export default function OutputDirectory() {
  const [outputDirectory, setOutputDirectory] = useState<string>();

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
    listen("output-directory-set", (event: any) => {
      console.log("Event received!", event.payload);
      setOutputDirectory(event.payload.path)
      localStorage.setItem("outputDirectory", JSON.stringify(event.payload.path))
    })
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
