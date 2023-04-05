import Actions from '@/components/actions/Actions';
import OutputDirectory from '@/components/header/OutputDirectory';
import Routes from '@/components/header/Routes';
import DisplaySelectedImages from '@/components/header/SelectedImages';
import ImageActions from '@/components/images/ImageActions';
import DisplayTimeline from '@/components/timeline/Timeline';
import { Grid } from '@mui/material';
import Head from 'next/head'
import { useState } from 'react';

export default function Home() {
  const [directoryName, setDirectoryName] = useState<string>("");
  const [selectedImages, setSelectedImages] = useState<Array<string>>([]);
  const [eventsParent, setEventsParent] = useState<any>();

  return (
    <>
      <Head>
        <title>Deltascope Desktop</title>
        <meta name="description" content="Generated by create next app" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main className="h-screen bg-slate-600">
        <Grid container >
          <Grid item xs={12}>
              <div>
                <h1 className="text-3xl px-2 py-2 font-mono text-center" >Deltascope</h1>
              </div>          
          </Grid>
          <Grid item container>
            <OutputDirectory />
            <DisplaySelectedImages />
            <Routes />
          </Grid>
          <Grid>
            <ImageActions setImages={undefined} setParentDirectoryName={undefined} />
          </Grid>
          {/* TODO: Make this stretch to bottom of page */}
          {/* <Grid item xs={12} className="flex flex-col gap-5 h-full my-auto">
              <div>
                <DisplayTimeline eventsData={eventsParent} />
              </div>
              <div className='bg-slate-900 h-1' ></div>
              <Grid item container className="h-full px-2 py-2">
                <ImageActions setImages={setSelectedImages} setParentDirectoryName={setDirectoryName} />
                <Actions directory={directoryName} images={selectedImages} setEventsParent={setEventsParent} />
              </Grid>          
          </Grid>           */}
        </Grid>
      </main>
    </>
  )
}
