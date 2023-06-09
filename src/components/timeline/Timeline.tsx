import { Box, Grid } from "@mui/material";
import { useEffect, useState } from "react";
import { Timeline } from "vis-timeline";

const DisplayTimeline = ({ eventsData }: { eventsData: any }) => {
  // const [timeLineLoaded, setTimeLineLoaded] = useState<boolean>(false)
  const [selectedEventsTimeline, setSelectedEventsTimeline] = useState<{ modified: boolean, accessed: boolean, changed: boolean, created: boolean }>(
    {
      "modified": true,
      "accessed": true,
      "changed": true,
      "created": true
    }
  );

  // useEffect(() => { 
  //   if (!timeLineLoaded && eventsData) {
  //     createTimeline()
  //     setTimeLineLoaded(true)
  //   }
  // }, [])

  const changeEventsShown = (type: string) => {
    switch(type) {
      case "modified":
        selectedEventsTimeline.modified = !selectedEventsTimeline.modified;
        break;
      case "accessed":
        selectedEventsTimeline.accessed = !selectedEventsTimeline.accessed;
        break;
      case "changed":
        selectedEventsTimeline.changed = !selectedEventsTimeline.changed;
        break;
      case "created":
        selectedEventsTimeline.created = !selectedEventsTimeline.created;
        break;
    }
    createTimeline();
  }

  const addToTimeline = (element: string) => {
    let res = false 

    for (const key in selectedEventsTimeline) {
      const temp= selectedEventsTimeline as any;
      if (element == key) {
        res = temp[key]
      } 
    }

    return res;
  }

  const createTimeline = () => {
    const container = document.getElementById("visualization") as HTMLElement;
    container.innerHTML = "";

    const items: Array<any> = []
    let idCounter = 0;
    const deltaEvents = eventsData.delta;
    deltaEvents.map((element: any) => {
      const path = element.name
      let fileType;

      switch(element.file_type[0]) {
        case "-":
          fileType = "Unknown"
          break;
        case "r":
          fileType = "Regular File"
          break;
        case "d":
          fileType = "Directory"
          break;
        case "l":
          fileType = "Link"
          break;
        default:
          fileType = element.file_type[0]
          break;
      }

      const divContent = `      
        <div style={{ width: '25px', height: '25px', padding: '0.5rem', wordWrap: 'break-word' }} >
          <p>Path: ${path}</p>
          <p>Type: ${fileType}</p>
         </div>`

      // Date to date
      let itemToAdd: any = { id: idCounter, content: divContent , start: element.date }

      if (element.b_activity != ".") {
        itemToAdd.className = "created"
      }
      else if (element.m_activity != ".")  {
        itemToAdd.className = "modified";
      }      
      else if (element.a_activity != ".") {
        itemToAdd.className = "accessed"
      }
      else if (element.c_activity != ".") {
        itemToAdd.className = "changed"
      }


      if (addToTimeline(itemToAdd.className)) {
        items.push(itemToAdd);
        idCounter++;
      }

    })

    console.log("Adding items to timeline", items);

    // Configuration for the Timeline
    const options = {
      height: '300px',
      stack: true,
      horizontalScroll: true,
    }

    // Create a Timeline
    const timeline = new Timeline(container, items, options);
  }


  return (
    <>
      <Grid item container xs direction="column" className="px-2 py-2">
        <Grid item>
          <Box id="visualization" className=" border-black border-1 border-solid bg-white m-2 rounded-lg h-80" style={{ height: '350px'}} /* style={{ border: '1px solid black', height: '350px', margin: '1rem', padding: '0.5rem', backgroundColor: "white" }} */>
          </Box>
        </Grid>
        { /* style={{ backgroundColor: '#42a5f5' }} */ }
        <div className="flex gap-5 justify-center items-center">
          <div className="bg-green-400 px-2 py-2 rounded-md w-max cursor-pointer" onClick={() => changeEventsShown("modified")}>
            <h3 className="text-black font-bold" > Modified</h3>
          </div>
          <div className="bg-gray-400 px-2 py-2 rounded-md w-max cursor-pointer" onClick={() => changeEventsShown("accessed")}>
            <h3 className="text-black font-bold" >Accessed</h3>
          </div>
          <div className="bg-red-400 px-2 py-2 rounded-md w-max cursor-pointer" onClick={() => changeEventsShown("changed")}>
            <h3 className="text-black font-bold">Changed: </h3>
          </div>
          <div className="bg-blue-400 px-2 py-2 rounded-md w-max cursor-pointer" onClick={() => changeEventsShown("created")}>
            <h3 className="text-black font-bold" >Created: </h3>
          </div>
        </div>
      </Grid>
    </>
  )
}

export default DisplayTimeline;
