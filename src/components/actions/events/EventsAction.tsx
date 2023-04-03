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
  const [ErrorMessage, setDeltaError] = useState<string>("");
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

    
    Object.keys(events).map((key: string) => {

        const eventsType = events[key]
        let idCounter = 0;
        let previousDate = "";
        eventsType.forEach((element: any) => {
          
          let itemToAdd: any = {
            id: idCounter,
            date: element.Date,
            size: element.Size,
            activity: getActivity(element.mActivity, element.aActivity, element.cActivit, element.bActivity),
            fileType: getFileType(element.FileType[0]),
            ownerPerm: element.OwnerPerm,
            groupPerm: element.GroupPerm,
            otherPerm: element.OtherPerm,
            uid: element.UUID,
            guid: element.GUID,
            inode: element.Inode,
            name: element.Path
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
    const data = { directoryName: directoryName, images: images };

    fetch("http://localhost:8000/events/", {
      method: "POST",
      body: JSON.stringify(data),
      headers: {
        "Content-Type": "application/json",
      },
    })
      .then(async (e) => {
        let data = await e.json();
        setEventsParent(data)
        eventsToTable(data.events);
      })
      .catch((e) => {
        setDisplayErrorMessage(true);
        setDeltaError("Unable to retrieve events");
      });
  };



  return (
    <>
      <div>
        <div>
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
              {/* <Box height={'350px'} width='100%'> */}
                <DataGrid
                  onRowClick={handleRowClick}
                  sx={{ fontSize: "1.2rem" }}
                  rows={events.delta}
                  columns={columns}
                />
              {/* </Box> */}
            </TabPanel>

            <TabPanel value="1">
              {/* <Box height={'350px'} width='100%'> */}
                <DataGrid
                  onRowClick={handleRowClick}
                  sx={{ fontSize: "1.2rem" }}
                  rows={events.base}
                  columns={columns}
                />
              {/* </Box> */}
            </TabPanel>

            <TabPanel value="2">
              {/* <Box height={'350px'} width='100%'> */}
                <DataGrid
                  onRowClick={handleRowClick}
                  sx={{ fontSize: "1.2rem" }}
                  rows={events.next}
                  columns={columns}
                />
              {/* </Box> */}
            </TabPanel>

          </TabContext>
        </div>
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
