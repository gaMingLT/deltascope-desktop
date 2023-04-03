import { AppBar, Box, Grid, Tab, Tabs, Typography } from "@mui/material";
import TabContext from "@mui/lab/TabContext";
import TabList from "@mui/lab/TabList";
import TabPanel from "@mui/lab/TabPanel";
import { useState } from "react";
import EventsAction from "./events/EventsAction";
import FilesAction from "./files/FilesAction";
// import DisplayEvents from "./DisplayEvents";
// import DisplayFiles from "./DisplayFiles";

const Actions = ({
  directory,
  images,
  setEventsParent,
}: {
  directory: any;
  images: any;
  setEventsParent: any;
}) => {
  const [value, setValue] = useState(0);

  const handleChange = (event: React.SyntheticEvent, newValue: number) => {
    setValue(newValue);
  };

  return (
    <>
    <Grid item xs>
      <div>
          <TabContext value={value.toString()}>
            <Box className="bg-white">
              <TabList
                // orientation="vertical"
                variant="fullWidth"
                onChange={handleChange}
                aria-label="lab API tabs example"
              >
                <Tab label="Events" value="0" />
                <Tab label="Files" value="1" />
              </TabList>
            </Box>

            <TabPanel value="0">
              <EventsAction images={[]} directoryName="" setEventsParent={undefined} />
              {/* <p>Events</p> */}
              {/* <DisplayEvents images={images} directoryName={directory} setEventsParent={setEventsParent} /> */}
            </TabPanel>

            <TabPanel value="1">
              {/* <p>Files</p> */}
              <FilesAction directoryName={""} />
              {/* <DisplayFiles directoryName={directory} /> */}
            </TabPanel>

          </TabContext> 
      </div>
    </Grid>
      {/* <Grid item xs={8}>
          <Box textAlign="center" padding="1rem">
            <Box textAlign="center">
              <h2>Events</h2>
            </Box>
            <Box>
              <TabContext value={value.toString()}>
                <Box sx={{ borderBottom: 1, borderColor: "divider" }}>
                  <TabList
                    onChange={handleChange}
                    aria-label="lab API tabs example"
                  >
                    <Tab label="Events" value="0" />
                    <Tab label="Files" value="1" />
                  </TabList>
                </Box>

                <TabPanel value="0">
                  <DisplayEvents images={images} directoryName={directory} setEventsParent={setEventsParent}  />
                </TabPanel>

                <TabPanel value="1">
                  <DisplayFiles directoryName={directory}  />
                </TabPanel>

              </TabContext>
            </Box>
          </Box>
      </Grid> */}
    </>
  );
};

export default Actions;
