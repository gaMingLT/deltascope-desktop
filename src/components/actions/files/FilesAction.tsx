import { Alert, Box, Button, Grid, Snackbar } from "@mui/material"
import { useState } from "react";
import FileAction from "./FileAction";

const FilesAction = ({ directoryName }: {
  directoryName: string;
}) => { 

  const [files, setFiles] = useState<any>({})
  const [directoryPath, setDirectoryName] = useState<string>(directoryName)
  const [loadedFileContent, setLoadedFileContent] = useState<Blob>();

  const [errorMessage, setErrorMessage] = useState<string>("");
  const [openErrorMessage, setOpenErrorMessage] = useState(false);

  const baseToFile = (base: any): any => {
    return [Buffer.from(base, 'base64')]
  }

  const getFiles = () => {
    const data = { "directoryName": directoryName };

    if (!directoryName) {
      setErrorMessage("No images selected for comparison");
      handleErrorMessage();
      return;
    } 

    fetch("http://localhost:8000/diff/files", {
      method: "POST",
      body: JSON.stringify(data),
      headers: {
        "Content-Type": "application/json",
      },
    })
      .then(async (e) => {
        let data = await e.json();
        console.log("Data: ", data);
        setFiles(data["diff_files"])
      })
      .catch((e) => {
        setErrorMessage("Unable to retrieve files");
        handleErrorMessage();
      });
  }

  const loadFile = (e: any) => {
    const fileName = e.target.getAttribute("data-name");
    const fileContentString = new Blob(baseToFile(files[fileName]))
    setLoadedFileContent(fileContentString)
  }

  const handleErrorMessage = () => {
    setOpenErrorMessage(true);
  };

  const handleClose = (event: React.SyntheticEvent | Event, reason?: string) => {
    if (reason === 'clickaway') {
      return;
    }

    setOpenErrorMessage(false);
  };


  return(
  <>
    <Grid container item className="py-4 px-4">
      
      <Grid item className="w-max mx-auto mb-8">
        <h3 className="text-2xl font-bold text-center" >Modified Files</h3>
      </Grid>

      <Grid className="flex flex-row justify-between gap-4 w-full">

        <Grid item className="flex flex-col  w-1/5 h-full bg-slate-400 py-4 px-4 rounded-lg items-center">
          <div className="flex flex-col justify-between px-2 py-2 h-60 overflow-y-auto" >
            {
              Object.keys(files).map((key: string, index: number) => {
                return (
                  <div key={crypto.randomUUID()} data-name={key} className="px-2 py-2 cursor-pointer bg-pink-400 rounded-sm" >
                    {key}
                  </div>
                )
              })
            } 
          </div>

          <div className="align-bottom w-full">
            <Button variant="contained" className="bg-slate-700 hover:bg-gray-600 m-2 w-full" onClick={getFiles}>
              Get Files
            </Button>
            <Box>
              <Snackbar open={openErrorMessage} autoHideDuration={6000} onClose={handleClose}>
                <Alert onClose={handleClose} severity="error" sx={{ width: '100%' }}>
                  {errorMessage}
                </Alert>
              </Snackbar>
            </Box>
          </div> 
        </Grid>

        <Grid item className="w-4/5 bg-slate-400 rounded-md">
          <FileAction fileBlob={loadedFileContent} />
        </Grid>

      </Grid>

    </Grid>
  </>
  )
}

export default FilesAction
