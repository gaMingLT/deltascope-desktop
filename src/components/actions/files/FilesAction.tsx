import { Alert, Box, Button, Grid, Typography } from "@mui/material"
import { useState } from "react";
import FileAction from "./FileAction";
// import FileDisplay from "./FileDisplay"


const FilesAction = ({ directoryName }: {
  directoryName: string;
}) => { 

  const [files, setFiles] = useState<any>({})
  const [directoryPath, setDirectoryName] = useState<string>(directoryName)
  const [errorMessage, setErrorMessage] = useState<string>("");
  // const [displayError, setDisplayErrorMessage] = useState<boolean>(false);
  const [loadedFileContent, setLoadedFileContent] = useState<Blob>();

  const baseToFile = (base: any): any => {
    console.log('Base: ', atob(base));
    return [atob(base)]
  }

  const getFiles = () => {
    const data = { "directoryName": directoryName };

    if (!directoryName) {
      // setDisplayErrorMessage(true);
      setErrorMessage("No images selected for comparison");
      // setTimeout(() => setDisplayErrorMessage(false), 5000);
      setTimeout(() => setErrorMessage(''), 5000);
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
        // setDisplayErrorMessage(true);
        setErrorMessage("Unable to retrieve files");
        // setTimeout(() => setDisplayErrorMessage(false), 5000);
        setTimeout(() => setErrorMessage(''), 5000);
      });
  }

  const loadFile = (e: any) => {
    const fileName = e.target.getAttribute("data-name");
    const fileContentString = new Blob(baseToFile(files[fileName]))
    setLoadedFileContent(fileContentString)
  }


  return(
  <>
    <Grid container item>
      
      <Grid item className="">
        <h3 className="text-xl font-bold text-center" >Modified Files</h3>
      </Grid>

      <Grid item container direction="row">

        <Grid item className="w-full">
          <div className="flex flex-col justify-between px-2 py-2" >
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

          <div>
            <Button variant="contained" className="bg-slate-700 hover:bg-gray-600 m-2 w-max" onClick={getFiles}>
              Get Files
            </Button>
            {/* <div> */}
              {errorMessage ? (
                <Alert className="m-2" severity="error">
                  {errorMessage}
                </Alert>
              ) : (
                ""
              )}
            {/* </div>  */}
          </div> 
        </Grid>

        <Grid item className="w-3/5">
          <FileAction fileBlob={loadedFileContent} />
        </Grid>

      </Grid>

    </Grid>
    {/* <Grid container  columnGap={5} >
      <Grid item container spacing={2}  direction="column" xs={2} style={{ border: '2px solid white', borderRadius: '5px', padding: '0.5rem' }} >
        <Grid item>
          <Box >
            <Typography variant="h5" >Modified Files</Typography>
          </Box>
        </Grid>
        <Grid item container spacing="2" direction="column">
          {
            Object.keys(files).map((key: string, index: number) => {
              return (
                <Grid item key={index} > 
                  <Box bgcolor="pink" px={0.5} py={0.5} onClick={loadFile} data-name={key} style={{ cursor: 'pointer' }} >
                    {key}
                  </Box>
                </Grid>
              )
            })
          }
        </Grid>
      </Grid>
      <Grid item container spacing={2}  xs={6} style={{ border: '2px solid white', borderRadius: '5px' }} >
        <FileDisplay fileBlob={loadedFileContent} />
      </Grid>
      <Grid item container spacing={2} direction="column">
        <Grid item>
          <Box>
            <Button variant="contained" sx={{ margin: "1rem" }} onClick={getFiles}>
              Get Files
          </Button>
          </Box>
        </Grid>
        <Grid>
          <Box>
            {displayError ? (
              <Alert sx={{ marginTop: "1rem" }} severity="error">
                {ErrorMessage}
              </Alert>
            ) : (
              ""
            )}
          </Box>
        </Grid>
      </Grid>
    </Grid> */}
  </>
  )
}

export default FilesAction
