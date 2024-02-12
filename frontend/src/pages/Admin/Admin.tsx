import React from "react";
import { useNavigate } from "react-router-dom";

const Admin = () => {
  const navigate = useNavigate();
  return (
    <div
      style={{
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
        height: "100vh",
        backgroundColor: "#fff", // Set background to white
      }}
    >
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          justifyContent: "center",
          gap: "32px", // Add gap between buttons and title
          backgroundColor: "#f5f5f5", // Add light gray background to box
          padding: "32px", // Add padding to box
          borderRadius: "8px", // Add border radius for rounded corners
          boxShadow: "0px 4px 4px rgba(0, 0, 0, 0.25)", // Add subtle shadow
        }}
      >
        <h1
          style={{ fontSize: "24px", fontWeight: "bold", marginBottom: "16px" }}
        >
          Admin Panel
        </h1>
        <div
          style={{
            display: "flex",
            justifyContent: "space-between", // Distribute buttons evenly
            width: "100%", // Make buttons fill available width
          }}
        >
          <button
            style={{
              backgroundColor: "#da4954", // Change button background to primary
              color: "#fff", // Change button text color to white
              width: "180px", // Adjust button width
              height: "50px",
              borderRadius: "8px",
              boxShadow: "0px 4px 4px rgba(0, 0, 0, 0.25)",
              fontSize: "16px",
              fontWeight: "bold",
              transition: "all 0.2s ease-in-out",
              marginRight: "1vw", "&:hover": {
                backgroundColor: "#f2f2f2",
                transform: "scale(1.05)",
              },
            }}
            onClick={() => navigate("/admin/doctor-data")}
          >
            Doctor Data
          </button>
          <button
            style={{
              ...buttonStyle, // Use spread operator to reuse button styles
            }}
            onClick={() => navigate("/admin/patient-data")}
          >
            Patient Data
          </button>
        </div>
      </div>
    </div>
  );
};

const buttonStyle = {
  backgroundColor: "#da4954",
  color: "#fff",
  width: "180px",
  height: "50px",
  borderRadius: "8px",
  boxShadow: "0px 4px 4px rgba(0, 0, 0, 0.25)",
  fontSize: "16px",
  fontWeight: "bold",
  transition: "all 0.2s ease-in-out",
  "&:hover": {
    backgroundColor: "#f2f2f2",
    transform: "scale(1.05)",
  },
};

export default Admin;
