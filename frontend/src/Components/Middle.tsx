import { Button, List, ListItem, Paper } from "@mui/material";
import folder from "../assets/images/folder.png";
import refresh from "../assets/images/refresh.png";
import React from "react";

function Middle() {
  return (
    <div style={{ marginLeft: "4.3 vw", width: "85vw" }}>
      <img
        src={refresh}
        style={{
          width: "1.5vw",
          height: "1.5vw",
          marginLeft: "2vw",
          marginTop: "2vw",
        }}
      />
      <Paper
        elevation={0}
        style={{
          backgroundColor: "#F8FCFF",
          borderBottom: "1px solid lightgrey",
          borderTop: "1px solid lightgrey",
          marginTop: "1.5vw",
        }}
      >
        <List>
          <ListItem>
            <img src={folder} style={{ width: "1.4vw", height: "1.4vw" }} />
            <span
              style={{
                fontSize: "1vw",
                marginLeft: "1.2vw",
                fontWeight: "500",
              }}
            >
              Patient: 69
              <span
                style={{
                  fontSize: "1vw",
                  marginLeft: "12vw",
                  fontWeight: "200 ",
                }}
              >
                Aditya Jha
              </span>
            </span>
            <Button variant="contained" color="primary" style={{ marginLeft: "663px", height: '40px' }}>
              Approve
            </Button>
          </ListItem>
        </List>
      </Paper>

      <Paper
        elevation={0}
        style={{
          backgroundColor: "#F8FCFF",
          borderBottom: "1px solid lightgrey",
          borderTop: "1px solid lightgrey",
        }}
      >
        <List>
          <ListItem>
            <img src={folder} style={{ width: "1.4vw", height: "1.4vw" }} />
            <span
              style={{
                fontSize: "1vw",
                marginLeft: "1.2vw",
                fontWeight: "500",
              }}
            >
              Patient: 70
              <span
                style={{
                  fontSize: "1vw",
                  marginLeft: "12vw",
                  fontWeight: "200 ",
                }}
              >
                Moulik Singh Rajput
              </span>
              
            </span>
            <Button variant="contained" color="primary" style={{ marginLeft: "600px", height: '40px' }}>
              Approve
            </Button>
          </ListItem>
          
        </List>
      </Paper>

      <Paper
        elevation={0}
        style={{
          backgroundColor: "#F8FCFF",
          borderBottom: "1px solid lightgrey",
          borderTop: "1px solid lightgrey",
        }}
      >
        <List>
          <ListItem>
            <img src={folder} style={{ width: "1.4vw", height: "1.4vw" }} />
            <span
              style={{
                fontSize: "1vw",
                marginLeft: "1.2vw",
                fontWeight: "500",
              }}
            >
              Patient: 71
              <span
                style={{
                  fontSize: "1vw",
                  marginLeft: "12vw",
                  fontWeight: "200 ",
                }}
              >
                Soumyajit
              </span>
            </span>
            <Button variant="contained" color="primary" style={{ marginLeft: "670px", height: '40px' }}>
              Approve
            </Button>
          </ListItem>
        </List>
      </Paper>

      <Paper
        elevation={0}
        style={{
          backgroundColor: "#F8FCFF",
          borderBottom: "1px solid lightgrey",
          borderTop: "1px solid lightgrey",
        }}
      >
        <List>
          <ListItem>
            <img src={folder} style={{ width: "1.4vw", height: "1.4vw" }} />
            <span
              style={{
                fontSize: "1vw",
                marginLeft: "1.2vw",
                fontWeight: "500",
              }}
            >
              Patient: 72
              <span
                style={{
                  fontSize: "1vw",
                  marginLeft: "12vw",
                  fontWeight: "200 ",
                }}
              >
                Manas Jha
              </span>
            </span>
            <Button variant="contained" color="primary" style={{ marginLeft: "663px", height: '40px' }}>
              Approve
            </Button>
          </ListItem>
        </List>
      </Paper>

      <Paper
        elevation={0}
        style={{
          backgroundColor: "#F8FCFF",
          borderBottom: "1px solid lightgrey",
          borderTop: "1px solid lightgrey",
        }}
      >
        <List>
          <ListItem>
            <img src={folder} style={{ width: "1.4vw", height: "1.4vw" }} />
            <span
              style={{
                fontSize: "1vw",
                marginLeft: "1.2vw",
                fontWeight: "500",
              }}
            >
              Patient: 73
              <span
                style={{
                  fontSize: "1vw",
                  marginLeft: "12vw",
                  fontWeight: "200 ",
                }}
              >
                Raunaq Bose
              </span>
            </span>
            <Button variant="contained" color="primary" style={{ marginLeft: "644px", height: '40px' }}>
              Approve
            </Button>
          </ListItem>
        </List>
      </Paper>
    </div>
  );
}

export default Middle;
