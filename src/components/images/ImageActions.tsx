import { Alert, Box, Button, Checkbox, FormControlLabel, FormGroup, Grid, LinearProgress } from "@mui/material";
import { invoke } from '@tauri-apps/api/tauri'
import { use, useState } from "react";

export default function ImageActions() {
  const [availableImages, setAvailableImages] = useState<Array<string>>([]);
  const [selectedImagesCheckBox, setSelectedImagesCheckBox] = useState<{[index: string]: boolean}>({});
  const [selectedImages, setSelectedImages] = useState<Array<string>>([]);
  

  const [ErrorMessage, setDeltaError] = useState<string>("");
  // const [displayError, setDisplayErrorMessage] = useState<boolean>(false);
  const [message, setMessage] = useState<string>("");
  // const [displayMessage, setDisplayMessage] = useState<boolean>(false);
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
    // console.log('Images: ', selectedImages);
    // console.log('Image:', image);

    setSelectedImagesCheckBox((selectedImagesCheckBox) => {
      const temp = selectedImagesCheckBox;
      temp[image] = !temp[image];
      return temp;
    })

    // console.log('Checkbox: ', selectedImagesCheckBox); 
    
    setSelectedImages((selectedImages) => {
      if (selectedImages.includes(image)) {
        const imagesArray = selectedImages.filter((name) => image !== name);
        return imagesArray;
      } else {
        return [image, ...selectedImages];
      }
    });
    // console.log('Images updated: ', selectedImages);
  }

  const initiateDelta = async () => {
    if (selectedImages.length != 2) {
      setDeltaError("Unable to initiate delta - amount of selected imgaes not supported (must be exactly 2)");
      setTimeout(() => setDeltaError(""), 5000);
    }
    else {
      const data = { images: selectedImages, directoryNames: "" };

      setDisplayLoading(true);

      invoke('initiate_delta', { images: selectedImages, directoryName: "" })
      .then(async (res) => {
        console.log('Reponse: ', res);
        // let data = await res.json();

        setMessage('Deltaing images - succesfully');
        setTimeout(() => setMessage(''), 5000);
      })
      .catch((e) => {
        console.log('Error Response: ', e);
    
        setDeltaError(`Error: ${e}!`);
        setTimeout(() => setDeltaError(''), 5000);
      })
    }

    setDisplayLoading(false);
  }

  return (
    <>
      <Grid item xs={3} className="py-2 px-2">
        <div className="flex flex-col w-max justify-between gap-5 px-2 py-1.5" >
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
              <Box bgcolor="white">
                {displayLoading ? (
                  <LinearProgress color="primary" />
                ) : ("")
                }
              </Box>
          </div>
        </div>
      </Grid>
    </>
  )
}
