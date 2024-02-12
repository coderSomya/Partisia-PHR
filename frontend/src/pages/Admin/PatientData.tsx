import { Grid } from "@mui/material";
import React, { useState } from "react";
import Navbar from "../../Components/Navbar";
import Leftpanel from "../../Components/Leftpanel";
import Middle from "../../Components/Middle";
import Rightpanel from "../../Components/Rightpanel";
import Footer from "../../Components/Footer";

function PatientData() {
  const [subCollect, setSubCollect] = useState("");
  const [search, setSearch] = useState("");

  return (
    <div>
      <Grid container>
        <Grid item xs={12}>
          <Navbar setSearch={setSearch} />
        </Grid>
        <Grid item xs={2}>
          <Leftpanel setSubCollect={setSubCollect} />
        </Grid>
        <Grid item xs={9}>
          <Middle search={search} subCollect={subCollect} />
        </Grid>
        <Grid item xs={1}>
          <Rightpanel />
        </Grid>
        <Grid item xs={12}>
          <Footer />
        </Grid>
      </Grid>
    </div>
  );
}

export default PatientData;
