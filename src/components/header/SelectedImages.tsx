import { Button } from "@mui/material";
import { useEffect, useRef, useState } from "react";


export default function DisplaySelectedImages() {
  const [selectedImages, setSelectImages] = useState<Array<String>>();

  const LoadSelectedImages = () => {
      const temp = JSON.parse(localStorage.getItem("selectedImages") as string);
      
      if (temp && temp.length == 2) {
        setSelectImages(temp);
      }
      else {
        setSelectImages(["No images selected!"]);
      }
  }

  useEffect(() => {
      if (!selectedImages || selectedImages.length == 0) {
        LoadSelectedImages();

      }
    
  })


  return (
    <>
      <div className="flex justify-between gap-4 mx-auto items-center px-2 py-2 bg-gray-800 rounded-md">
        <p className="font-bold">Images: </p>
        <div>
          {
            selectedImages?.map((image) => {
              return (
                <p key={crypto.randomUUID()} >{image}</p>
              )
            })
          }
        </div>
      </div>
    </>
  )
}
