import { DataGrid, GridColDef, GridEventListener } from "@mui/x-data-grid";
import {
  Alert,
  Box,
  Button,
  Grid,
  Tab,
} from "@mui/material";
import TabContext from "@mui/lab/TabContext";
import TabList from "@mui/lab/TabList";
import TabPanel from "@mui/lab/TabPanel";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

const columns: GridColDef[] = [
  {
    field: "date",
    headerName: "Date",
    width: 250,
  },
  { field: "size", headerName: "Size", width: 65 },
  { field: "activity", headerName: "Acitvity", width: 150 },
  { field: "fileType", headerName: "File Type", width: 150 },
  { field: "ownerPerm", headerName: "Owner", width: 75 },
  { field: "groupPerm", headerName: "Group", width:  75 },
  { field: "otherPerm", headerName: "Other", width: 75 },
  { field: "uid", headerName: "User ID", width: 100 },
  { field: "guid", headerName: "Group ID", width: 100 },
  { field: "inode", headerName: "Inode", width: 100 },
  { field: "name", headerName: "Name", width: 500 },
];

const EventsAction = ({
  images,
  directoryName,
  setEventsParent
}: {
  images: Array<string>;
  directoryName: string;
  setEventsParent: any;
}) => {
  const [value, setValue] = useState(0);
  const [events, setEvents] = useState({
    delta: new Array(),
    base: new Array(),
    next: new Array(),
  });
  // const [eventsSet, setEventsSet] = useState<boolean>(false);
  const [errorMessage, setErrorMessage] = useState<string>("");
  const [displayError, setDisplayErrorMessage] = useState<boolean>(false);
  const [displaySelectedRow, setDisplaySelectedRow] = useState("");

  const handleChange = (event: React.SyntheticEvent, newValue: number) => {
    setValue(newValue);
  };


  const handleRowClick: GridEventListener<'rowClick'> = (params) => {
    console.log('Params: ', params)
    setDisplaySelectedRow(params.row.name);
  };

  const getActivity = (mActivity: string, aActivity: string, cActivity: string, bActivity: string) => {
    if (bActivity != ".") {
      return "Created"
    }
    else if (mActivity != ".")  {
      return "Modified"
    }      
    else if (aActivity != ".") {
      return "Accessed"
    }
    else if (cActivity != ".") {
      return "Created"
    }
  }

  const getFileType = (type: string) => {
    switch(type) {
      case "-":
        return "Unknown"
      case "r":
        return "Regular File"
      case "d":
        return "Directory"
      case "l":
        return "Link"
      default:
        return type
    }
  }

  const eventsToTable = (events: any) => {
    console.log('Events to table!')
    const tempEventsData = {
      delta: new Array(),
      base: new Array(),
      next: new Array(),
    };

    // events.forEach((key: string) => {
      
    // });
    
    Object.keys(events).map((key: string) => {

      const eventsType = events[key]

      let idCounter = 0;
      let previousDate = "";
      eventsType.forEach((element: any) => {
        
        let itemToAdd: any = {
          id: idCounter,
          date: element.date,
          size: element.size,
          activity: getActivity(element.m_activity, element.a_activity, element.c_activity, element.b_activity),
          fileType: getFileType(element.file_type[0]),
          ownerPerm: element.owner_perm,
          groupPerm: element.group_perm,
          otherPerm: element.other_perm,
          uid: element.uid,
          guid: element.gid,
          inode: element.inode,
          name: element.name
        }

        switch(key) {
          case "delta":
            tempEventsData.delta.push(itemToAdd);
            break;
          case "next":
            tempEventsData.next.push(itemToAdd);
            break;
          case "base":
            tempEventsData.base.push(itemToAdd);
            break;
          default:
            break;
        }
        
        idCounter++;
      })

        // setEventsSet(true)
    })

    setEvents(tempEventsData)
  }

  const getEvents = () => {
    console.log("Events");

    const images_storage = JSON.parse(localStorage.getItem("selectedDeltaImages") as string) as Array<String>;
    const directoryPath = JSON.parse(localStorage.getItem("directoryPath") as string) as String;

    if (!images_storage || images_storage.length != 2 || !directoryPath) {
      setErrorMessage(`Images & Path not set!`);
      return;
    }

    invoke('get_events_images', { images: images_storage, directoryPath: directoryPath })
        .then(async (data: any) => {
          console.log("Data: ", data);
          
          setEventsParent(data)
          eventsToTable(data);
        })
      .catch((e) => {
        console.log('Error: ', e);
        setErrorMessage(`Unable to retrieve events: ${e.message}`);
        setTimeout(() => setErrorMessage(""), 5000);
      });
  };



  return (
    <>
      <div className="flex gap-10 ">
       <div className="w-1/12">
          <div>
            <Button variant="contained" className="m-2 bg-slate-800" onClick={getEvents}>
              Get Events
            </Button>            
          </div>
          <div>
            {errorMessage ? (
              <Alert className="mt-4 w-max mr-auto" severity="error">
                {errorMessage}
              </Alert>
            ) : (
              ""
            )}
          </div>
        </div>
        <div className="w-11/12">
          <TabContext value={value.toString()}>
            <Box sx={{ borderBottom: 1, borderColor: "divider" }}>
              <TabList
                onChange={handleChange}
                aria-label="lab API tabs example"
              >
                <Tab label="Delta's" value="0" />
                <Tab label="Base Image" value="1" />
                <Tab label="Next Image" value="2" />
              </TabList>
            </Box>

            <TabPanel value="0">
              <Box height={'350px'} width='100%'>
                <DataGrid
                  onRowClick={handleRowClick}
                  sx={{ fontSize: "1.2rem" }}
                  rows={events.delta}
                  columns={columns}
                />
              </Box>
            </TabPanel>

            <TabPanel value="1">
              <Box height={'350px'} width='100%'>
                <DataGrid
                  onRowClick={handleRowClick}
                  sx={{ fontSize: "1.2rem" }}
                  rows={events.base}
                  columns={columns}
                />
              </Box>
            </TabPanel>

            <TabPanel value="2">
              <Box height={'350px'} width='100%'>
                <DataGrid
                  onRowClick={handleRowClick}
                  sx={{ fontSize: "1.2rem" }}
                  rows={events.next}
                  columns={columns}
                />
              </Box>
            </TabPanel>

          </TabContext>
        </div>
        {/* <div>
          <div>
            <Button variant="contained" className="m-2 bg-slate-800" onClick={getEvents}>
              Get Events
            </Button>            
          </div>
          <div>
            {errorMessage ? (
              <Alert className="mt-4 w-max mr-auto" severity="error">
                {errorMessage}
              </Alert>
            ) : (
              ""
            )}
          </div>
        </div> */}
      </div>
      {/* <Grid container spacing="2" direction="column">
        <Grid item>
        <TabContext value={value.toString()}>
                <Box sx={{ borderBottom: 1, borderColor: "divider" }}>
                  <TabList
                    onChange={handleChange}
                    aria-label="lab API tabs example"
                  >
                    <Tab label="Delta's" value="0" />
                    <Tab label="Base Image" value="1" />
                    <Tab label="Next Image" value="2" />
                  </TabList>
                </Box>

                <TabPanel value="0">
                  <Box height={'350px'} width='100%'>
                    <DataGrid
                      onRowClick={handleRowClick}
                      sx={{ fontSize: "1.2rem" }}
                      rows={events.delta}
                      columns={columns}
                    />                    
                  </Box>
                </TabPanel>

                <TabPanel value="1">
                  <Box height={'350px'}  width='100%'>
                    <DataGrid
                        onRowClick={handleRowClick}
                        sx={{ fontSize: "1.2rem" }}
                        rows={events.base}
                        columns={columns}
                      />                    
                  </Box>
                </TabPanel>

                <TabPanel value="2">
                  <Box height={'350px'}  width='100%'>
                    <DataGrid
                        onRowClick={handleRowClick}
                        sx={{ fontSize: "1.2rem" }}
                        rows={events.next}
                        columns={columns}
                      />                    
                  </Box>
                </TabPanel>

              </TabContext>
        </Grid>
        <Grid item>
          <Grid item>
            { displaySelectedRow ? 
                <Box style={{ padding: '0.5rem', fontSize: '1.25rem' }} >
                  <p><strong>Path: </strong> {displaySelectedRow}</p>
                </Box> : ''
            }
          </Grid>
        </Grid>
        <Grid item container spacing="2" direction="column">
          <Grid item>
            <Box>
              <Button variant="contained" sx={{ margin: "1rem" }} onClick={getEvents}>
                Get Events
              </Button> 
            </Box>
          </Grid>
          <Grid item>
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
  );
};

export default EventsAction;
