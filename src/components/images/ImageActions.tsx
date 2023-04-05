import { Alert, Box, Button, Checkbox, FormControlLabel, FormGroup, Grid, LinearProgress } from "@mui/material";
import { invoke } from '@tauri-apps/api/tauri'
import { useEffect, useState } from "react";

type ResponseType = { directory_path: String, images: Array<String> }

export default function ImageActions({
  setParentDirectoryName,
  setImages,
}: {
  setParentDirectoryName: any;
  setImages: any;
}) {
  const [availableImages, setAvailableImages] = useState<Array<string>>([]);
  const [selectedImagesCheckBox, setSelectedImagesCheckBox] = useState<{[index: string]: boolean}>({});
  // const [selectedImages, setSelectedImages] = useState<Array<string>>([]);

  const [cleanedStorage, setCleanedStorage] = useState<boolean>(false);

  useEffect(() => {
    if (!cleanedStorage) {
      localStorage.setItem("selectedImages", JSON.stringify([]));
      setCleanedStorage(true);
    }
  })
  
  const [ErrorMessage, setDeltaError] = useState<string>("");
  const [message, setMessage] = useState<string>("");
  const [displayLoading, setDisplayLoading] = useState<boolean>(false);

  const loadAvailableImages = async () => {
    const stored_paths: Array<string> = await invoke('get_stored_paths');
    const temp: {[index: string]:any} = {};
    stored_paths.forEach((element: string) => {
      temp[parsePathImage(element)] = false;
    });
    setSelectedImagesCheckBox(temp);

    setAvailableImages(stored_paths);
  }

  const parsePathImage = (path: string) => {
    const pathSplit = path.split('\\');
    return pathSplit[pathSplit.length-1]
  }

  const handleChange = (e:any ) => {
    const image =  e.target.getAttribute('data-name');
    console.log('Image:', image);

    setSelectedImagesCheckBox((selectedImagesCheckBox) => {
      const temp = selectedImagesCheckBox;
      temp[image] = !temp[image];
      return temp;
    })
    
    // setSelectedImages((selectedImages) => {
    //   if (selectedImages.includes(image)) {
    //     const imagesArray = selectedImages.filter((name) => image !== name);
    //     return imagesArray;
    //   } else {
    //     console.log("In here: else", image, ...selectedImages);
    //     return [...selectedImages, image];
    //   }
    // });
    const images_storage = JSON.parse(localStorage.getItem("selectedImages") as string) as Array<String>;

    if (images_storage) {
      if (images_storage.includes(image)) {
        const temp_images = images_storage.filter(value => value != image)
        localStorage.setItem("selectedImages", JSON.stringify(temp_images)); 
      }
      else {
        images_storage.push(image)
        localStorage.setItem("selectedImages", JSON.stringify(images_storage)); 
      }
    }
    else {
      localStorage.setItem("selectedImages", JSON.stringify([image]));
    }
    // console.log("Test: ", JSON.parse(localStorage.getItem("selectedImages") as string) as Array<String>);
  }

  const initiateDelta = async () => {
    const selectedImages = JSON.parse(localStorage.getItem("selectedImages") as string) as Array<String>;
    if (selectedImages.length != 2) {
      setDeltaError("Unable to initiate delta - amount of selected imgaes not supported (must be exactly 2)");
      setTimeout(() => setDeltaError(""), 5000);
    }
    else {
      const data = { images: selectedImages, directoryNames: "" };

      setDisplayLoading(true);

      invoke('initiate_delta', { images: selectedImages, directoryName: "" })
      .then(async (res: any) => {
        // setParentDirectoryName(res.directory_path);
        // setImages(res.images);
        localStorage.setItem("selectedDeltaImages", JSON.stringify(res.images));
        localStorage.setItem("directoryPath", JSON.stringify(res.directory_path));
        setMessage('Deltaing images - succesfully');
        setTimeout(() => setMessage(''), 5000);
        setDisplayLoading(false); 
      })
      .catch((e) => {
        setDeltaError(`Error: ${e}!`);
        setTimeout(() => setDeltaError(''), 5000);
        setDisplayLoading(false);
      })
    }

    // setDisplayLoading(false);
  }

  return (
    <>
      <Grid item xs={3} className="py-2 px-2">
        <div className="flex flex-col w-max justify-between gap-5 px-2 py-1.5">
          {
            availableImages.map((element: string,index: number) => {
              return (
                <FormGroup key={crypto.randomUUID()} className="px-2 py-2 w-max bg-slate-800 text-lg rounded-md">
                  <FormControlLabel control={<Checkbox checked={selectedImagesCheckBox[element]} onChange={handleChange} inputProps={{ "data-name": parsePathImage(element) } as any} />}  label={parsePathImage(element)} />
                </FormGroup>
              )
            })
          }
        </div>
        <div>
          <div className="flex flex-col gap-3 w-max justify-between">
            <Button variant="contained" onClick={loadAvailableImages} onLoad={loadAvailableImages} className="bg-slate-800 w-100 hover:bg-gray-600" >Get Available Images</Button>
            <Button variant="contained" onClick={initiateDelta} className="bg-slate-800 w-100  hover:bg-gray-600" >Initiate Delta</Button> 
          </div>
          <div>
            <Box>
              {ErrorMessage ? (
                <Alert className="mt-5 w-max" severity="error">
                  {ErrorMessage}
                </Alert>
              ) : (
                ""
              )}
            </Box>
            <Box>
                {message ? (
                  <Alert className="mt-5 w-max" severity="success">
                    {message}
                  </Alert>
                ) : (
                  ""
                )}
              </Box>
              <Box bgcolor="white" className="mt-4">
                {displayLoading ? (
                  // TODO: Make size not fixed
                  <LinearProgress className="rounded-md w-52 mx-auto" color="secondary" />
                ) : ("")
                }
              </Box>
          </div>
        </div>
      </Grid>
    </>
  )
}
