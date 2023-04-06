import FilesAction from "@/components/actions/files/FilesAction";
import OutputDirectory from "@/components/header/OutputDirectory";
import Routes from "@/components/header/Routes";
import DisplaySelectedImages from "@/components/header/SelectedImages";
import { Grid } from "@mui/material";
import Head from "next/head";

export default function Home() {

  return (
    <>
    <Head>
      <title>Deltascope Desktop</title>
      <meta name="description" content="Generated by create next app" />
      <meta name="viewport" content="width=device-width, initial-scale=1" />
      <link rel="icon" href="/favicon.ico" />
    </Head>
    <main className="h-screen bg-slate-600">
      <Grid className="flex flex-col h-full gap-4">
        <Grid container>
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
        </Grid>
        <Grid>
            <FilesAction directoryName="" />
        </Grid>        
      </Grid>

    </main>
  </>
  )
}
