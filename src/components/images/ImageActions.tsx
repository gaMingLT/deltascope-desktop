import { Alert, Box, Button, Checkbox, FormControlLabel, FormGroup, Grid, LinearProgress, Snackbar } from "@mui/material";
import { invoke } from '@tauri-apps/api/tauri'
import { useEffect, useRef, useState } from "react";
import { listen } from '@tauri-apps/api/event'

type ResponseType = { directory_path: string, images: Array<string> }

export default function ImageActions({
  setParentDirectoryName,
  setImages,
}: {
  setParentDirectoryName: any;
  setImages: any;
}) {
  const [availableImages, setAvailableImages] = useState<Array<string>>([]);
  const [selectedImagesCheckBox, setSelectedImagesCheckBox] = useState<{[index: string]: boolean}>({});

  const [cleanedStorage, setCleanedStorage] = useState<boolean>(false);

  const [openMessage, setOpenMessage] = useState(false);
  const [openErrorMessage, setOpenErrorMessage] = useState(false);

  const handleMessage = () => {
    setOpenMessage(true);
  };

  const handleErrorMessage = () => {
    setOpenErrorMessage(true);
  };

  const handleClose = (event: React.SyntheticEvent | Event, reason?: string) => {
    if (reason === 'clickaway') {
      return;
    }

    setOpenMessage(false);
    setOpenErrorMessage(false);
  };

  const mountID = useRef(null);
  const unlistens = useRef<{[index: string]: any}>({});

  useEffect(() => {
    if (!cleanedStorage) {
      localStorage.setItem("selectedImages", JSON.stringify([]));
      setCleanedStorage(true);
    }

    const thisMountID = Math.random();
    (mountID.current as unknown as number) = thisMountID;
  
   listen('delta_finished', (event) => {
    if (mountID.current != thisMountID) {
      unlistens.current[thisMountID]();
    }
    else {
      setMessage("Deltaing Images Succesfull!");
      handleMessage();
    }}).then(new_unlisten => { unlistens.current[thisMountID] = new_unlisten});
  
    return () => {
      mountID.current = null;
    };
  }, []);

  
  const [errorMessage, setDeltaError] = useState<string>("");
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
      handleErrorMessage();
    }
    else {

      setDisplayLoading(true);

      invoke('initiate_delta', { images: selectedImages, directoryName: "" })
      .then(async (res: any) => {
        localStorage.setItem("selectedDeltaImages", JSON.stringify(res.images));
        localStorage.setItem("directoryPath", JSON.stringify(res.directory_path));
        setDisplayLoading(false); 
      })
      .catch((e) => {
        setDeltaError(`Error: ${e}!`);
        handleErrorMessage();
        setDisplayLoading(false);
      })
    }
  }

  return (
    <>
      <Grid item xs={3} className="flex flex-col py-2 px-2 h-full bg-slate-400 rounded-sm">
        <div className="flex flex-col w-max justify-between gap-5 px-2 py-1.5 h-2/4 overflow-y-auto overscroll-y-contain rounded-sm">
          {availableImages.map((element: string, index: number) => {
            return (
              <FormGroup
                key={crypto.randomUUID()}
                className="px-2 py-2 w-max bg-slate-800 text-lg rounded-md"
              >
                <FormControlLabel
                  control={
                    <Checkbox
                      checked={selectedImagesCheckBox[element]}
                      onChange={handleChange}
                      inputProps={
                        { "data-name": parsePathImage(element) } as any
                      }
                    />
                  }
                  label={parsePathImage(element)}
                />
              </FormGroup>
            );
          })}
        </div>
        <div className="mb-4 mt-auto w-full h-max align-bottom">
          <div className="flex flex-col gap-3 w-full justify-between bottom-5">
            <Button
              variant="contained"
              onClick={loadAvailableImages}
              onLoad={loadAvailableImages}
              className="bg-slate-800 w-100 hover:bg-gray-600"
            >
              Get Available Images
            </Button>
            <Button
              variant="contained"
              onClick={initiateDelta}
              className="bg-slate-800 w-100  hover:bg-gray-600"
            >
              Initiate Delta
            </Button>
          </div>
          <div>
            <Box>
              <Snackbar open={openErrorMessage} autoHideDuration={6000} onClose={handleClose}>
                <Alert onClose={handleClose} severity="error" sx={{ width: '100%' }}>
                  {errorMessage}
                </Alert>
              </Snackbar>
            </Box>
            <Box>
              <Snackbar open={openMessage} autoHideDuration={6000} onClose={handleClose}>
                <Alert onClose={handleClose} severity="success" sx={{ width: '100%' }}>
                  {message}
                </Alert>
              </Snackbar>
            </Box>
            <Box bgcolor="white" className="mt-4">
              {displayLoading ? (
                // TODO: Make size not fixed
                <LinearProgress
                  className="rounded-md w-52 mx-auto"
                  color="secondary"
                />
              ) : (
                ""
              )}
            </Box>
          </div>
        </div>
      </Grid>
    </>
  );
}
